
<div align="center">
  <img src="img/navi.webp">
</div>

# Navi

A tool for expanding the power of your [exobrain](https://beepb00p.xyz/exobrain/) ⚡🧠.

The point is to have an personalized digital mentor that understands you, and can help guide you  through the process of reflecting on your week.
 
To use it, you connect your notesource (for now, just [Notion](https://www.notion.com/)) and then run the Navi CLI, which ingests the last week's of your notes and uses them to inform an LLM digital mentor ([OpenAI](https://openai.com/) for now) on how best to do a retrospective with you on your week.

## Getting Started (With Notion as your exobrain)
First you will need to make Navi aware of your Notion workspace.

1.  Follow the instructions for creating an [internal Notion integration here](https://www.notion.so/help/create-integrations-with-the-notion-api#create-an-internal-integration) 
   - You will need to create an integration [here](https://www.notion.so/profile/integrations)
   - Make a `.env` file using the `.env.example` template with the command `cp .env.example .env`
   - Then copy the API token from your integration into your `.env` file
   - Provide the integration with the following capabilities: 

<div align="center">
  <img src="img/notion-integration.png" alt="Notion Connection Instructions">
</div>

2. Now that you have a Notion integration setup, you need to associate Pages in your Notion Workspace with the integration. You can do this by following [this Notion guide](https://www.notion.so/help/add-and-manage-connections-with-the-api#add-connections-to-pages)

<div align="center">
  <img src="img/notion-connection.png" alt="Notion Connection Instructions">
</div>

3. Go to [OpenAI's API page](https://platform.openai.com/settings/organization/api-keys) and make an API key. For the permissions you can give it the following

<div align="center">
  <img src="img/openai_api_permissions.png" alt="OpenAI API Permissions">
</div>

4. Add the OpenAI API key to your `.env` file
5. `cargo build`
6. `cargo run`
7. Finally, have a retro conversation with Navi!

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
- [x] Write retro prompt
- [x] Prompt retro with latest 7 days of exobrain data
- [x] Build conversation datastructure so I can have a retro with my last 7 days of exobrain data
- [ ] optimization: remove blocks from the final expanded block roots Tree datastructure that were last edited prior to the cutoff
- [ ] Make more TODOs
