use chrono::Duration;
use clap::Parser;
use log::{debug, info};
use navi::{intelligence::assistant_flow, notion::Notion};
use std::{env, fs, path::Path, time::Instant};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of days to look back for notes
    #[arg(short, long, default_value = "7")]
    days: i64,

    /// If true, use the prompt_info.md file to store the prompt info, otherwise generate a new prompt
    #[arg(short, long, default_value = "false")]
    use_prompt_info_file: bool,
}

#[tokio::main]
async fn main() {
    let program_start = Instant::now();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dotenv_path = format!("{}/.env", manifest_dir);
    dotenv::from_path(dotenv_path).ok();
    env_logger::init();

    let Args {
        days,
        use_prompt_info_file,
    } = Args::parse();
    let dur = Duration::days(days);

    let notion = Notion::new(env::var("NOTION_TOKEN").expect("NOTION_TOKEN must be set")).unwrap();

    // ingest notes data from Notion (or from a cached file if it exists)
    let prompt_info = if use_prompt_info_file && Path::new("prompt_info.md").exists() {
        info!(target: "notion", "Using cached prompt info from prompt_info.md");
        fs::read_to_string("prompt_info.md").unwrap()
    } else {
        let parsed_pages = notion.parse_last_edited(dur).await.unwrap();
        let prompt = Notion::to_prompt_text(parsed_pages).unwrap();
        if use_prompt_info_file {
            fs::write("prompt_info.md", &prompt).unwrap();
        }
        debug!(target: "notion", "prompt info:\n{}", prompt);
        prompt
    };

    info!(target: "notion", "Analysis complete! Navi is now ready to guide you through the process of reflecting on your notes");
    info!(target: "notion", "Let's begin by asking Navi to start the retro, and see what Navi's response is...");

    // Log the total time to get to the first prompt
    let total_elapsed = program_start.elapsed();
    info!(target: "intelligence", "--- Total time to first prompt: {:.2} seconds", total_elapsed.as_secs_f64());

    assistant_flow(prompt_info).await.unwrap();
}
