use chrono::{Duration, Utc};
use clap::Parser;
use dotenv::dotenv;
use log::{debug, info, trace};
use navi::{
    core::{
        datatypes::{Block, ParsedNotionPage},
        helpers::build_markdown_from_trees,
    },
    intelligence::assistant_flow,
    notion::Notion,
};
use notion_client::NotionClientError;
use std::{collections::HashSet, env, fs, path::Path};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of days to look back for notes
    #[arg(short, long, default_value = "7")]
    days: i64,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let args = Args::parse();
    let dur: Duration = Duration::days(match env::var("RUST_LOG") {
        Ok(log_level) => match log_level.to_lowercase().as_str() {
            "debug" | "trace" => 1,
            _ => args.days,
        },
        Err(_) => args.days,
    });

    let notion = Notion::new(env::var("NOTION_TOKEN").expect("NOTION_TOKEN must be set")).unwrap();

    // ingest notes data from Notion (or from a cached file if it exists)
    let prompt_info = if matches!(env::var("RUST_LOG").as_deref(), Ok("debug" | "trace"))
        && Path::new("prompt_info.md").exists()
    {
        debug!(target: "notion", "Using cached prompt info from prompt_info.md");
        fs::read_to_string("prompt_info.md").unwrap()
    } else {
        let parsed_pages = parse_last_edited(notion, dur).await.unwrap();
        let prompt = to_prompt_text(parsed_pages).unwrap();
        fs::write("prompt_info.md", &prompt).unwrap();
        debug!(target: "notion", "prompt info:\n{}", prompt);
        prompt
    };

    info!(target: "notion", "Analysis complete! Navi is now ready to guide you through the process of reflecting on your notes");
    info!(target: "notion", "Let's begin by asking Navi to start the retro, and see what Navi's response is...");

    assistant_flow(prompt_info).await.unwrap();
}

async fn parse_last_edited(
    notion: Notion,
    dur: Duration,
) -> Result<Vec<ParsedNotionPage>, NotionClientError> {
    info!(target: "notion", "Thanks for choosing Navi as your digital mentor! Navi will begin by analyzing your last {} {} of notes. This may take several minutes, depending on how dedicated a notetaker you are...", dur.num_days(), if dur.num_days() == 1 { "day" } else { "days" });
    let cutoff = Utc::now() - dur;

    let pages_edited_after_cutoff_date = notion.get_last_edited_pages(cutoff).await.unwrap();
    info!(target: "notion", "retrieved {} Pages edited in the last {} days", pages_edited_after_cutoff_date.len(), dur.num_days());
    info!(target: "notion", "From these Pages, Navi will fetch the notes it needs to guide you in reflecting on the past {} days", dur.num_days());

    // TODO: idea: instead of storing the whole Block data, which is 95% worthless data, just strip out the
    // text and id, store that in a struct, and use that to build the markdown
    let mut parsed_pages = Vec::new();

    let mut block_roots_duplicates_checker: HashSet<Block> = HashSet::new();
    let mut expanded_blocks_duplicates_checker: HashSet<Block> = HashSet::new();
    for page in pages_edited_after_cutoff_date {
        debug!(target: "notion", "Page URL: {}", page.url);

        let new_block_roots = notion
            .get_page_block_roots(&page, cutoff, &mut block_roots_duplicates_checker)
            .await
            .unwrap();

        if new_block_roots.len() > 0 {
            debug!(target: "notion", "found {} new block roots for page: {}",  new_block_roots.len(), page.title);
            let trees = notion
                .expand_block_roots(new_block_roots, &mut expanded_blocks_duplicates_checker)
                .await
                .unwrap();

            parsed_pages.push(ParsedNotionPage {
                page_id: page.id,
                title: page.title,
                page_content: trees,
            });
        }
    }

    debug!(target: "notion", "retrieved {} pages with non-empty block roots", parsed_pages.len());
    trace!(target: "notion", "the parsed pages look like:\n{:#?}", parsed_pages.iter().map(|p| (&p.page_id, p.page_content.iter().map(|t| (t.root().borrow_data().id.clone(), t.root().borrow_data().text.clone())).collect::<Vec<_>>())).collect::<Vec<_>>());

    Ok(parsed_pages)
}

fn to_prompt_text(notion_pages: Vec<ParsedNotionPage>) -> Result<String, NotionClientError> {
    let mut every_prompt_markdown = Vec::new();
    for page in notion_pages {
        let single_page_prompt_markdown = build_markdown_from_trees(page.page_content);
        every_prompt_markdown.push(format!(
            "Page Title: {}\n{}",
            page.title, single_page_prompt_markdown
        ));
    }

    Ok(every_prompt_markdown.join("\n\n"))
}
