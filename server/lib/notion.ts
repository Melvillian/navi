import { DateTime } from 'luxon';
import { Client } from '@notionhq/client';
import { 
  BlockObjectResponse, 
  PageObjectResponse,
  SearchResponse,
  ListBlockChildrenResponse
} from '@notionhq/client/build/src/api-endpoints';
import { Block, BlockID, Page, PageID } from './datatypes';

/**
 * Simple tree structure to replace dendron dependency
 */
export interface BlockTree {
  block: Block;
  children: BlockTree[];
}

/**
 * TypeScript implementation of the Notion client, based on the Rust version.
 */
export class Notion {
  private client: Client;

  /**
   * Creates a new Notion client with the given token.
   */
  constructor(token: string) {
    this.client = new Client({ auth: token });
  }

  /**
   * Returns all Pages in the Notion integration's workspace that have been edited since the cutoff date.
   * The Pages will be ordered by last edited date in descending order.
   */
  async getLastEditedPages(cutoff: DateTime): Promise<Page[]> {
    const pages: Page[] = [];
    let currentCursor: string | undefined = undefined;
    
    do {
      // Search for pages, sorted by last edited time
      const response = await this.client.search({
        filter: {
          value: 'page',
          property: 'object'
        },
        sort: {
          timestamp: 'last_edited_time',
          direction: 'descending'
        },
        page_size: 100,
        start_cursor: currentCursor
      }) as SearchResponse;

      currentCursor = response.next_cursor || undefined;
      
      // Filter to only include pages (not databases)
      const notionPages = response.results.filter(
        result => result.object === 'page'
      ) as PageObjectResponse[];
      
      // Find the cutoff index - where pages are older than our cutoff date
      const cutoffIndex = notionPages.findIndex(page => 
        DateTime.fromISO(page.last_edited_time) < cutoff
      );
      
      // If we found a cutoff point, only process pages up to that point
      const pagesToProcess = cutoffIndex >= 0 
        ? notionPages.slice(0, cutoffIndex) 
        : notionPages;
      
      // Convert Notion pages to our Page type
      for (const notionPage of pagesToProcess) {
        const page = await this.notionPageToNaviPage(notionPage);
        pages.push(page);
      }
      
      // Exit if we've reached the cutoff or there are no more pages
      if (cutoffIndex >= 0 || !response.has_more) {
        break;
      }
    } while (currentCursor);
    
    return pages;
  }

  /**
   * For a given Page, retrieve all of its non-empty children, grandchildren, etc... Blocks 
   * that were edited since the cutoff date.
   */
  async getPageBlockRoots(
    page: Page, 
    cutoff: DateTime,
    duplicatesChecker: Set<string> = new Set()
  ): Promise<Block[]> {
    const blocksToProcess: Block[] = [...page.child_blocks];
    const blockRoots: Block[] = [];
    
    // Set a time limit for fetching children to avoid spending too much time on huge pages
    const abortTime = DateTime.now().plus({ seconds: 30 });
    
    while (blocksToProcess.length > 0) {
      // Check if we've exceeded our time limit
      if (DateTime.now() > abortTime) {
        console.debug(`Aborting page retrieval due to time limit for Page: ${page.title}`);
        break;
      }
      
      const block = blocksToProcess.shift()!;
      
      // Check for duplicates to prevent infinite loops
      const blockKey = block.id.toString();
      if (duplicatesChecker.has(blockKey)) {
        console.trace(`Already visited block ${block.id}, skipping it...`);
        continue;
      }
      duplicatesChecker.add(blockKey);
      
      // If the block was updated recently enough, include it in the results
      if (block.update_date >= cutoff) {
        if (!block.isEmpty()) {
          blockRoots.push(block);
        }
        continue;
      }
      
      // If the block has children, fetch them and add to processing queue
      if (block.has_children) {
        console.trace(`Fetching children block roots of block with id ${block.id}`);
        const children = await this.retrieveAllBlockChildren(
          block.id, 
          page.id
        );
        
        for (const childBlock of children) {
          console.trace(`Fetched child block: (id: ${childBlock.id}, text: ${childBlock.text})`);
          blocksToProcess.push(childBlock);
        }
      }
    }
    
    console.debug(`Fetched ${blockRoots.length} descendant Blocks from Page ${page.title}`);
    return blockRoots;
  }

  /**
   * Given a Block that has been recently edited, expand it into a tree with all its descendants.
   */
  private async expandBlockRoot(
    blockRoot: Block,
    duplicatesChecker: Set<string> = new Set()
  ): Promise<BlockTree> {
    const root: BlockTree = {
      block: blockRoot,
      children: []
    };
    
    // Queue of nodes to process, with their parent references
    const queue: Array<{node: BlockTree, block: Block}> = [
      {node: root, block: blockRoot}
    ];
    
    while (queue.length > 0) {
      const current = queue.shift()!;
      const block = current.block;
      const parentNode = current.node;
      
      console.debug(`Processing block: ${block.id}, ${block.text}`);
      
      const blockKey = block.id.toString();
      if (duplicatesChecker.has(blockKey)) {
        console.trace(`Already visited block ${block.id}, ${block.text}, skipping it...`);
        continue;
      }
      duplicatesChecker.add(blockKey);
      
      if (block.has_children) {
        console.trace(`Block with id ${block.id} has children, fetching them...`);
        
        const children = await this.retrieveAllBlockChildren(
          block.id,
          block.page_id
        );
        
        for (const child of children) {
          console.debug(`Child: ${child.id}, ${child.text}`);
          
          const childKey = child.id.toString();
          if (duplicatesChecker.has(childKey)) {
            console.trace(`Already visited child block ${child.id}, ${child.text}, skipping it...`);
            continue;
          } else if (!child.isEmpty()) {
            // Create a new tree node for this child
            const childNode: BlockTree = {
              block: child,
              children: []
            };
            
            // Add it to the parent's children
            parentNode.children.push(childNode);
            
            // Add it to the queue to process its children
            queue.push({node: childNode, block: child});
          }
        }
      }
    }
    
    return root;
  }

  /**
   * Given a Vec of Blocks (call these Blocks "roots") that have been updated recently,
   * return a Vec of BlockTree's where each BlockTree contains the Block root and all of its descendants.
   */
  async expandBlockRoots(
    blockRoots: Block[],
    duplicatesChecker: Set<string> = new Set()
  ): Promise<BlockTree[]> {
    const expandedRoots: BlockTree[] = [];
    
    for (const block of blockRoots) {
      const tree = await this.expandBlockRoot(block, duplicatesChecker);
      expandedRoots.push(tree);
    }
    
    return expandedRoots;
  }

  /**
   * Retrieves all of the children Blocks of a Block with the given ID.
   */
  async retrieveAllBlockChildren(
    blockId: BlockID,
    pageId: PageID
  ): Promise<Block[]> {
    const childrenBlocks: Block[] = [];
    let currentCursor: string | undefined = undefined;
    
    do {
      try {
        const response = await this.client.blocks.children.list({
          block_id: blockId.toString(),
          start_cursor: currentCursor,
          page_size: 100
        }) as ListBlockChildrenResponse;
        
        const blocks = response.results as BlockObjectResponse[];
        childrenBlocks.push(
          ...blocks.map(block => Block.fromNotionBlockResponse(block, pageId.toString()))
        );
        
        currentCursor = response.next_cursor || undefined;
        
        if (!response.has_more) {
          break;
        }
      } catch (error) {
        console.error('Error retrieving block children:', error);
        throw error;
      }
    } while (currentCursor);
    
    return childrenBlocks;
  }

  /**
   * Converts a Notion Page to a Navi Page.
   */
  private async notionPageToNaviPage(notionPage: PageObjectResponse): Promise<Page> {
    // Extract title from URL (similar to the Rust implementation)
    let title = "Unknown Page Title";
    const urlParts = notionPage.url.split('/');
    const lastPart = urlParts[urlParts.length - 1];
    
    if (lastPart) {
      const parts = lastPart.split('-');
      if (parts.length > 1) {
        // Remove the last part which is the ID
        title = parts.slice(0, -1).join(' ');
      }
    }
    
    // Retrieve child blocks
    const pageId = new PageID(notionPage.id);
    const childBlocks = await this.retrieveAllBlockChildren(
      new BlockID(notionPage.id),
      pageId
    );
    
    return new Page(
      pageId,
      title,
      notionPage.url,
      DateTime.fromISO(notionPage.created_time),
      DateTime.fromISO(notionPage.last_edited_time),
      childBlocks
    );
  }
} 