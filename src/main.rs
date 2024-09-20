use chrono::{Duration, Utc};
use dotenv::dotenv;
use dross::{core::helpers::build_markdown_from_trees, notion::Notion};
use log::{debug, info, trace};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let dur: Duration = Duration::days(match env::var("RUST_LOG") {
        Ok(log_level) => match log_level.to_lowercase().as_str() {
            "debug" | "trace" => 2,
            _ => 7,
        },
        Err(_) => 7,
    }); // TODO, make this a CLI arg, for now we're just differentiating
        // between DEBUG and non-debug to speed iterating on debugging

    // ingest notes data from Notion
    let notion_token: String = env::var("NOTION_TOKEN").expect("NOTION_TOKEN must be set");
    let notion = Notion::new(notion_token).unwrap();

    let cutoff = Utc::now() - dur;

    let pages_edited_after_cutoff_date = notion.get_last_edited_pages(cutoff).await.unwrap();
    info!(target: "notion", "retrieved {} Pages edited in the last {} days", pages_edited_after_cutoff_date.len(), dur.num_days());
    let mut pages_and_block_roots = Vec::new();
    for page in pages_edited_after_cutoff_date {
        debug!(target: "notion", "Page URL: {}", page.url);

        let new_block_roots = notion.get_page_block_roots(&page, cutoff).await.unwrap();
        info!(target: "notion", "found {} new block roots for page: {}",  new_block_roots.len(), page.title);
        if new_block_roots.len() > 0 {
            pages_and_block_roots.push((page, new_block_roots));
        }
    }

    debug!(target: "notion", "retrieved {} pages with non-empty block roots, now we will expand them!", pages_and_block_roots.len());

    let mut every_prompt_markdown = Vec::new();
    for (page, block_roots) in pages_and_block_roots {
        info!(target: "notion", "expanding {} block roots for page: {}", block_roots.len(), page.title);
        trace!(target: "notion", "the block roots look like: {:?}", block_roots.iter().map(|b| (&b.text, &b.id, &b.block_type)).collect::<Vec<_>>());
        let trees = notion.expand_block_roots(block_roots).await.unwrap();

        let single_page_prompt_markdown = build_markdown_from_trees(trees);
        every_prompt_markdown.push(format!(
            "Page Title: {}\n{}",
            page.title, single_page_prompt_markdown
        ));
    }
    let prompt_info = every_prompt_markdown.join("\n\n");
    info!(target: "notion", "prompt info:\n{}", prompt_info);

    info!(target: "notion", "notion page ingestion successful");
}
