
<div align="center">
  <img src="img/navi.webp">
</div>

# Navi

A tool for expanding the power of your [exobrain](https://beepb00p.xyz/exobrain/) ⚡🧠.

The point is to have an personalized digital mentor that understands you, and can help guide you  through the process of reflecting on your week.
 
To use it, you connect your notesource (for now, just [Notion](https://www.notion.com/)) and then run the Navi CLI, which ingests the last week's of your notes and uses them to inform an LLM digital mentor ([OpenAI](https://openai.com/) for now) on how best to do a retrospective with you on your week.

## Example Retro Conversation

```markdown
➜  navi git:(add-intelligence-first-attempt) ✗ RUST_LOG=debug cargo run                                [09:45:21]
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/navi`
[2024-11-09T14:45:22Z DEBUG notion] Using cached prompt info from prompt_info.md
[2024-11-09T14:45:22Z INFO  notion] Analysis complete! Navi is now ready to guide you through the process of reflecting on your notes
[2024-11-09T14:45:22Z INFO  notion] Let's begin by asking Navi to start the retro, and see what Navi's response is...
[2024-11-09T14:45:22Z DEBUG reqwest::connect] starting new connection: https://api.openai.com/
[2024-11-09T14:45:28Z INFO  intelligence] --- Response: ### Step 1: Synopsis of Your Weekly Notes
    
    #### a. Categorization of Topics Mentioned
    1. **Job Search and Interviews**
       - Communications and processes at Uniswap and Coinbase
       - Interview preparation strategies
       
    2. **Healthcare Research**
       - Exploration of Crowd Health as an alternative healthcare plan
    
    3. **Data and Software Development Insights**
       - Insights on software characteristics from *Designing Data-Intensive Applications*
       - Notes on housing challenges and the book *Strong Towns: Escaping the Housing Trap*
    
    4. **Personal Conversations and Reflections**
       - Discussions with Evan Litwin and David about various topics including politics and housing
       
    5. **Financial Decisions**
       - Adjustments to personal wealth portfolio and strategies for cryptocurrency investments
    
    #### b. List of People Communicated With
    - **Evan Litwin**: Discussed housing and politics
    - **David (Marathon)**: Discussed bikes, personal upbringing, and socio-political issues
    - **Josh**: Interview connection at Uniswap
    - **Chris**: Uniswap backend team member
    - **Kevin Quinn**: Interview connection at Coinbase
    
    #### c. List of Accomplishments
    - Explored and created detailed notes for interviews at Coinbase and Uniswap.
    - Engaged in multiple insightful conversations regarding healthcare, political representation, and community issues.
    - Developed a deeper understanding of software characteristics through *Designing Data-Intensive Applications*.
    
    Congratulations on your accomplishments! It sounds like you've been very productive and insightful in both your personal and professional endeavors over the past week.
    
    Do you have anything to add to the synopsis that I might have missed?
    
[2024-11-09T14:45:28Z INFO  intelligence] --- Enter your input or type 'exit()' to exit
I am wondering how it is best to proceed with the Uniswap interview, which is for a position working on their Uniswap-focused L2 called Unichain. I am also interviewing at Coinbase to work on Base but I care a lot more about Uniswap. 
[2024-11-10T19:34:02Z DEBUG reqwest::connect] starting new connection: https://api.openai.com/
[2024-11-10T19:34:04Z INFO  intelligence] --- Response: Thank you for sharing that! I will keep your focus on the Uniswap interview in mind as we proceed. 
    
    Are you ready to move on to Step 2: "What went well since the last retro?" If so, please share your thoughts!
    
[2024-11-10T19:34:04Z INFO  intelligence] --- Enter your input or type 'exit()' to exit
Yes. I think this week was mostly a period of rest for me. I had a lot of fun taking care of my daughter Gwen, and I finally finished the MVP of the CLI version of Navi. 
[2024-11-10T19:35:40Z DEBUG reqwest::connect] starting new connection: https://api.openai.com/
[2024-11-10T19:35:41Z INFO  intelligence] --- Response: That sounds wonderful! Taking a restful week to enjoy time with your daughter while also completing the MVP of the CLI version of Navi is a fantastic combination of personal and professional success. Kudos to you!
    
    Now, let’s move on to Step 3: "What didn't go well since the last retro?" What challenges or issues did you encounter during this period?
    
[2024-11-10T19:35:41Z INFO  intelligence] --- Enter your input or type 'exit()' to exit
I didn't accomplish much. I had wanted to do more of my Navi tasks, and do more coding challenges to prepare for the Uniswap and Coinbase interviews. I felt like my normal ratio of work-to-personal-consumption skewed more towards work than I'm used to. This week I want to be more intentional and productive. 
[2024-11-11T15:56:45Z DEBUG reqwest::connect] starting new connection: https://api.openai.com/

[2024-11-11T15:56:47Z INFO  intelligence] --- Response: It's totally understandable to feel that way, especially when balancing personal time and professional aspirations. It's great that you've gained awareness about your work-to-personal consumption ratio. Acknowledging it is the first step toward finding a better balance.
    
    Now, let's move on to Step 4: "What are 1 to 3 takeaways from this retro that can be made into actionable tasks to work on for the next retro?" 
    
    What tasks do you think would help you be more intentional and productive in the upcoming week? Feel free to propose a few, and we can work on making them SMART (specific, measurable, achievable, relevant, and time-oriented) together!
    
[2024-11-11T15:56:47Z INFO  intelligence] --- Enter your input or type 'exit()' to exit
[2024-11-11T15:56:50Z INFO  intelligence] --- Response: It seems like there was no response. No worries! Let's take it step by step. 
    
    Considering your desire to be more intentional and productive, here are a few potential tasks you might consider:
    
    1. **Set a Schedule for Coding Challenges**: Dedicate specific times each day to practice coding challenges (e.g., one hour each weekday). 
       
    2. **Daily Goals for Navi Tasks**: Break down your work on the CLI version of Navi into smaller tasks and set a daily goal, such as completing one portion per day.
    
    3. **Interview Preparation**: Allocate time each day to focus on Uniswap and Coinbase interview preparation, perhaps through research or practicing interviews.
    
    What do you think? Feel free to adjust these suggestions or add your own! We can refine them into SMART tasks together.
    
[2024-11-11T15:56:50Z INFO  intelligence] --- Enter your input or type 'exit()' to exit
These sound good to me
[2024-11-11T16:03:40Z DEBUG reqwest::connect] starting new connection: https://api.openai.com/
[2024-11-11T16:03:44Z INFO  intelligence] --- Response: Great! Let's refine them into SMART tasks:
    
    1. **Set a Schedule for Coding Challenges**:
       - **Specific**: Dedicate one hour each weekday.
       - **Measurable**: Complete a coding challenge during each session.
       - **Achievable**: Aim for 5 total hours of coding challenges per week.
       - **Relevant**: Prepare specifically for your interviews at Uniswap and Coinbase.
       - **Time-Oriented**: Start this from the upcoming Monday, aiming to complete it by Friday.
    
    2. **Daily Goals for Navi Tasks**:
       - **Specific**: Choose a specific portion of the CLI version of Navi to work on.
       - **Measurable**: Complete at least one distinct task each day.
       - **Achievable**: Ensure that the tasks are broken down into manageable parts.
       - **Relevant**: Directly contribute to your project progress.
       - **Time-Oriented**: Implement this daily for the next week.
    
    3. **Interview Preparation**:
       - **Specific**: Spend 30 minutes each day researching or practicing for Uniswap and Coinbase interviews.
       - **Measurable**: Track by noting which topics or questions you've covered each day.
       - **Achievable**: Starting this week allows you to build on it over time.
       - **Relevant**: Focus on areas of the interview that are most crucial for your potential roles.
       - **Time-Oriented**: Commit to do this from now until your interviews.
    
    How does that sound? Do you feel that these tasks will help you achieve your goals before the next retro?
    
[2024-11-11T16:03:44Z INFO  intelligence] --- Enter your input or type 'exit()' to exit
exit()
```

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

3. Go to [OpenAI's API page](https://platform.openai.com/settings/organization/api-keys) and make an API key. For the permissions you can give "All", but this is not secure and you should limit it to the minimum permissions needed for your use case when/if you deploy to production.

4. Add the OpenAI API key to your `.env` file
5. `cargo build`
6. `RUST_LOG=debug cargo run` # debug will give more info, and will cache the exobrain note data to a local file called prompt_info.md
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
