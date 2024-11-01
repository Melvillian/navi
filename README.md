
<div align="center">
  <img src="img/navi.webp">
</div>

# Navi (In Rust 🦀)

A tool for expanding the power of your [exobrain](https://beepb00p.xyz/exobrain/) ⚡🧠

## Getting Started

1. Follow the instructions for creating an [internal Notion integration here](https://www.notion.so/help/create-integrations-with-the-notion-api#create-an-internal-integration) 
   - You will need to create an integration [here](https://www.notion.so/profile/integrations)
   - And copy the token into your `.env` file
   - Provide the integration with the following capabilities: 

<div align="center">
  <img src="img/notion-integration.png" alt="Notion Connection Instructions">
</div>

2. Connect the integration to the Notion Pages you want this CLI to have access to by following [this Notion guide](https://www.notion.so/help/add-and-manage-connections-with-the-api#add-connections-to-pages)

<div align="center">
  <img src="img/notion-connection.png" alt="Notion Connection Instructions">
</div>

3. Run `cp .env.example .env` and fill in the env var values.
4. `cargo build`
5. `RUST_LOG=debug cargo run`
6. Profit!

## Helpful Tools

1. [notion-cli-rs](https://github.com/Melvillian/notion-cli-rs): useful for quickly fetching Notion data when you need to debug your exobrain.

## Roadmap

- [x] Build a service for ingesting exobrain text (Notion, Obsidian, Apple Notes, etc.)
- [ ] Use RAG and LLM Prompting to periodically run a personalized retro for your life
- [ ] Expand memory powers using SRS on exobrain
- [ ] Use Navi to identify [The One Thing](https://en.wikipedia.org/wiki/The_One_Thing_(book)) to iterate on
- [ ] Learn from users what new exobrain powers they would like  
=======

## TODO

- [x] Ingest latest 7 days of edited blocks from notion
- [ ] Write retro prompt
- [ ] Prompt retro with latest 7 days of exobrain data
- [ ] Build conversation datastructure so I can have a retro with my last 7 days of exobrain data
- [ ] optimization: remove blocks from the final expanded block roots Tree datastructure that were last edited prior to the cutoff
- [ ] Make more TODOs
- [ ] TODO: need to handle the case where I edit BlockA in PageA, and blockA references as a child BlockB in PageB which I also edited. This is a problem because it means our resulting PromptText will contain duplicate blocks (BlockA and BlockB). It's not the end of the world, but it's not ideal and at least a case for optimization via caching.