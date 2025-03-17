import { Express, Request, Response } from 'express';
import { Notion, BlockTree } from './notion';
import { DateTime } from 'luxon';
import { blockToMarkdown } from './datatypes';

export default (app: Express) =>
  app

    .get('/', async (req: Request, res: Response) => {
      res.render('index', {
        currentTime: new Date().toISOString(),
      });
    })

    .get('/api/fetch-blocks', async (req: Request, res: Response) => {
      try {
        const notion = new Notion(process.env.NOTION_TOKEN!);
        const blocks = await notion.getLastEditedPages(DateTime.now().minus({ days: 1 }));
        const markdown = await Promise.all(blocks.map(async page => {
          const blockTrees = await notion.expandBlockRoots(page.child_blocks);
          
          // Recursive function to process block tree
          const processBlockTree = (tree: BlockTree, depth: number = 0): string => {
            const blockMarkdown = blockToMarkdown(tree.block);
            if (tree.children.length === 0) {
              return blockMarkdown;
            }
            
            // Calculate indentation based on depth
            const indentation = '  '.repeat(depth);
            
            // Process children with increased depth
            const childrenMarkdown = tree.children
              .map(child => processBlockTree(child, depth + 1))
              .join('\n');
            
            return `${blockMarkdown}<br>${indentation}${childrenMarkdown}`;
          };

          return blockTrees.map(tree => processBlockTree(tree, 0)).join('<br>');
        })).then(pageMarkdowns => pageMarkdowns.join('<br>'));
        const html = require('marked').parse(markdown);
        res.send(html);        // res.json({ blocks: ["yay"] });
      } catch (error) {
        res.json({ blocks: ["oops"] });
      }
    })

    .get('/api/server-time', async (req: Request, res: Response) => {
      res.render('partials/serverTime', {
        currentTime: new Date().toISOString(),
      });
    });
