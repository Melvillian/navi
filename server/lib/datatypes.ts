import { DateTime } from 'luxon';
import type { BlockObjectResponse, RichTextItemResponse } from '@notionhq/client/build/src/api-endpoints';

// Equivalent TypeScript types for the Rust datatypes

/**
 * The identifier for a Notion `Page`. This exists to distinguish
 * between `Page` and `Block` identifiers at compile-time.
 */
export class PageID {
  constructor(private id: string) {}

  toString(): string {
    return this.id;
  }
}

/**
 * The identifier for a Notion `Block`. This exists to distinguish
 * between `Block` and `Page` identifiers at compile-time.
 */
export class BlockID {
  constructor(private id: string) {}

  toString(): string {
    return this.id;
  }
}

/**
 * Type representing the different types of blocks in Notion
 */
export type BlockType = 
  | { type: 'paragraph', paragraph: any }
  | { type: 'heading_1', heading_1: any }
  | { type: 'heading_2', heading_2: any }
  | { type: 'heading_3', heading_3: any }
  | { type: 'bulleted_list_item', bulleted_list_item: any }
  | { type: 'numbered_list_item', numbered_list_item: any }
  | { type: 'to_do', to_do: any }
  | { type: 'toggle', toggle: any }
  | { type: 'code', code: any }
  | { type: 'callout', callout: any }
  | { type: 'quote', quote: any }
  | { type: 'image', image: any }
  | { type: 'bookmark', bookmark: any }
  | { type: 'divider', divider: any }
  | { type: 'table', table: any }
  | { type: 'column_list', column_list: any }
  | { type: 'column', column: any }
  | { type: 'table_of_contents', table_of_contents: any }
  | { type: 'embed', embed: any }
  | { type: 'video', video: any }
  | { type: 'file', file: any }
  | { type: 'pdf', pdf: any }
  | { type: 'synced_block', synced_block: any }
  | { type: 'template', template: any }
  | { type: 'link_preview', link_preview: any }
  | { type: 'unsupported', unsupported: any };

/**
 * Parent type for blocks (simplified from Notion API)
 */
export interface Parent {
  type: string;
  page_id?: string;
  database_id?: string;
  block_id?: string;
  workspace?: boolean;
}

/**
 * A Block represents a single unit of notetaking, and its structure is heavily borrowed
 * from the Notion API's Block object.
 *
 * A Block is always contained within a Page. Its content is defined by the `block_type` field,
 * but a plain form of the Block's text is stored in the `text` field. Finally, a Block
 * can have children, but you need to fetch them separately.
 */
export class Block {
  id: BlockID;
  page_id: PageID;
  block_type: BlockType;
  text: string;
  creation_date: DateTime;
  update_date: DateTime;
  parent?: Parent;
  has_children: boolean;

  constructor(
    id: BlockID,
    page_id: PageID,
    block_type: BlockType,
    text: string,
    creation_date: DateTime,
    update_date: DateTime,
    parent: Parent | undefined,
    has_children: boolean
  ) {
    this.id = id;
    this.page_id = page_id;
    this.block_type = block_type;
    this.text = text;
    this.creation_date = creation_date;
    this.update_date = update_date;
    this.parent = parent;
    this.has_children = has_children;
  }

  /**
   * Converts a Notion API BlockObjectResponse to our application's Block type
   */
  static fromNotionBlockResponse(blockResponse: BlockObjectResponse, pageId: string): Block {
    // Extract the plain text from the rich_text arrays based on block type
    const extractPlainText = (block: BlockObjectResponse): string => {
      const blockType = block.type;
      
      // Handle different block types to extract text
      switch (blockType) {
        case 'paragraph':
        case 'heading_1':
        case 'heading_2':
        case 'heading_3':
        case 'bulleted_list_item':
        case 'numbered_list_item':
        case 'quote':
        case 'to_do':
        case 'toggle':
        case 'callout':
          // @ts-ignore - TypeScript doesn't understand the discriminated union here
          return block[blockType].rich_text
            .map((rt: RichTextItemResponse) => rt.plain_text || '')
            .join(' ');
            
        case 'code':
          // @ts-ignore
          return block.code.rich_text
            .map((rt: RichTextItemResponse) => rt.plain_text || '')
            .join(' ');
            
        case 'image':
        case 'file':
        case 'pdf':
        case 'video':
        case 'embed':
        case 'bookmark':
          // @ts-ignore
          return block[blockType].caption ? 
            // @ts-ignore
            block[blockType].caption
              .map((rt: RichTextItemResponse) => rt.plain_text || '')
              .join(' ') : '';
              
        case 'table_row':
          // @ts-ignore
          return block.table_row.cells
            .map((cell: RichTextItemResponse[]) => 
              cell.map((rt: RichTextItemResponse) => rt.plain_text || '').join(' ')
            )
            .join(' | ');
            
        case 'equation':
          // @ts-ignore
          return block.equation.expression || '';
          
        case 'link_preview':
          // @ts-ignore
          return block.link_preview.url || '';
          
        case 'divider':
        case 'breadcrumb':
        case 'table_of_contents':
        case 'column_list':
        case 'column':
        case 'link_to_page':
        case 'synced_block':
        case 'template':
        case 'table':
        case 'unsupported':
          return ''; // These block types don't have direct text content
          
        default:
          return '';
      }
    };

    // Create the block type object
    const blockTypeObj: BlockType = {
      type: blockResponse.type,
      [blockResponse.type]: (blockResponse as any)[blockResponse.type]
    } as BlockType;

    return new Block(
      new BlockID(blockResponse.id),
      new PageID(pageId),
      blockTypeObj,
      extractPlainText(blockResponse),
      DateTime.fromISO(blockResponse.created_time),
      DateTime.fromISO(blockResponse.last_edited_time),
      blockResponse.parent as Parent,
      blockResponse.has_children
    );
  }

  /**
   * Converts the block to markdown format
   */
  toMarkdown(): string {
    switch (this.block_type.type) {
      case 'heading_1':
        return `# ${this.text}`;
      case 'heading_2':
        return `## ${this.text}`;
      case 'heading_3':
        return `### ${this.text}`;
      case 'bulleted_list_item':
        return `- ${this.text}`;
      case 'numbered_list_item':
        return `1. ${this.text}`;
      case 'to_do':
        return `- [ ] ${this.text}`;
      case 'toggle':
        return `> ${this.text}`;
      default:
        return this.text;
    }
  }

  /**
   * Checks if the block is empty
   */
  isEmpty(): boolean {
    return this.text.length === 0;
  }
}

/**
 * A Page is a container for Blocks, and its structure is heavily borrowed
 * from the Notion API's Page object.
 *
 * The Page struct exists because the data sources (such as Notion) that we ingest
 * from all have the concept of a collection of Blocks, and provide APIs for fetching
 * recently edited collections of Blocks. We call those Pages.
 */
export class Page {
  id: PageID;
  title: string;
  url: string;
  creation_date: DateTime;
  update_date: DateTime;
  child_blocks: Block[];

  constructor(
    id: PageID,
    title: string,
    url: string,
    creation_date: DateTime,
    update_date: DateTime,
    child_blocks: Block[]
  ) {
    this.id = id;
    this.title = title;
    this.url = url;
    this.creation_date = creation_date;
    this.update_date = update_date;
    this.child_blocks = child_blocks;
  }
}

/**
 * Converts a Block to Markdown format
 */
export function blockToMarkdown(block: Block): string {
  return block.toMarkdown();
}