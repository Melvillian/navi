use chrono::{Duration, Utc};
use dotenv::dotenv;
use log::{debug, info, trace};
use navi::{
    core::{datatypes::Block, helpers::build_markdown_from_trees},
    intelligence::assistant_flow,
    notion::Notion,
};
use notion_client::NotionClientError;
use std::{collections::HashSet, env, fs, path::Path};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    // TODO, make this a CLI arg, for now we're just differentiating
    // between DEBUG and non-debug to speed iterating on debugging
    let dur: Duration = Duration::days(match env::var("RUST_LOG") {
        Ok(log_level) => match log_level.to_lowercase().as_str() {
            "debug" | "trace" => 1,
            _ => 7,
        },
        Err(_) => 7, // it means RUST_LOG is not set, so we default to 7 days
    });

    let notion = Notion::new(env::var("NOTION_TOKEN").expect("NOTION_TOKEN must be set")).unwrap();

    // ingest notes data from Notion (or from a cached file if it exists)
    let prompt_info = if matches!(env::var("RUST_LOG").as_deref(), Ok("debug" | "trace"))
        && Path::new("prompt_info.md").exists()
    {
        debug!(target: "notion", "Using cached prompt info from prompt_info.md");
        fs::read_to_string("prompt_info.md").unwrap()
    } else {
        let prompt = ingest_notion(notion, dur).await.unwrap();
        fs::write("prompt_info.md", &prompt).unwrap();
        debug!(target: "notion", "prompt info:\n{}", prompt);
        prompt
    };

    info!(target: "notion", "Analysis complete! Navi is now ready to guide you through the process of reflecting on your notes");
    info!(target: "notion", "Let's begin by asking Navi to start the retro, and see what Navi's response is...");

    assistant_flow(prompt_info).await.unwrap();
}

async fn ingest_notion(notion: Notion, dur: Duration) -> Result<String, NotionClientError> {
    info!(target: "notion", "Thanks for choosing Navi as your digital mentor! Navi will begin by analyzing your last {} days of notes. This may take several minutes, depending on how dedicated a notetaker you are...", dur.num_days());
    let cutoff = Utc::now() - dur;

    let pages_edited_after_cutoff_date = notion.get_last_edited_pages(cutoff).await.unwrap();
    info!(target: "notion", "retrieved {} Pages edited in the last {} days", pages_edited_after_cutoff_date.len(), dur.num_days());
    info!(target: "notion", "From these Pages, Navi will fetch the notes it needs to guide you in reflecting on the past {} days", dur.num_days());
    let mut pages_and_block_roots = Vec::new();

    // TODO: idea: instead of storing the whole Block data, which is 95% worthless data, just strip out the
    // text and id, store that in a struct, and use that to build the markdown

    let mut duplicates_checker: HashSet<Block> = HashSet::new();
    for page in pages_edited_after_cutoff_date {
        debug!(target: "notion", "Page URL: {}", page.url);

        let new_block_roots = notion
            .get_page_block_roots(&page, cutoff, &mut duplicates_checker)
            .await
            .unwrap();

        if new_block_roots.len() > 0 {
            debug!(target: "notion", "found {} new block roots for page: {}",  new_block_roots.len(), page.title);
            pages_and_block_roots.push((page, new_block_roots));
        }
    }

    debug!(target: "notion", "retrieved {} pages with non-empty block roots, now we will expand them", pages_and_block_roots.len());
    trace!(target: "notion", "the pages and block roots look like:\n{:#?}", pages_and_block_roots.iter().map(|(p, br)| (&p.title, br.iter().map(|b| (b.id.clone(), b.text.clone())).collect::<Vec<_>>())).collect::<Vec<_>>());

    let mut every_prompt_markdown = Vec::new();
    let mut duplicates_checker: HashSet<Block> = HashSet::new();
    for (page, block_roots) in pages_and_block_roots {
        debug!(target: "notion", "expanding {} block roots for page: {}", block_roots.len(), page.title);
        let trees = notion
            .expand_block_roots(block_roots, &mut duplicates_checker)
            .await
            .unwrap();

        let single_page_prompt_markdown = build_markdown_from_trees(trees);
        every_prompt_markdown.push(format!(
            "Page Title: {}\n{}",
            page.title, single_page_prompt_markdown
        ));
    }

    return Ok(every_prompt_markdown.join("\n\n"));
}
