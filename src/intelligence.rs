use async_openai::{
    error::OpenAIError,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use log::info;

const NAVI_INSTRUCTIONS: &str = "I want you to lead a retrospective for me using the information from my weekly notes. You have a vast experience with running retrospectives, and have read all of the most important writing on how to deliver fun, useful, and high-signal retro's from people like Esther Derby and Ben Linders.

You must adhere to the 4-step outline below in order from 1 to 4 for the retro. Begin with fulfilling the requirements for 1, then 2, then 3, and finally 4. Only proceed to the next step after you have asked the user if it's alright to proceed, and the user acknowledges. The 4 steps are:
1. Give a synopsis of the user's notes since the last retro. After giving your synopsis, ask the user if the user can add anything to the synopsis that you might have missed. This synopsis must contain:
    a. a categorization of the topics mentioned in the user's notes
    b. a list of the people the user communicated with
    c. a list of the accomplishments the user made. You must sincerely congratulate the user on their accomplishments
2. Ask the user 'What went well since the last retro?'
3. Ask the user 'What didn't go well since the last retro?'
4. Ask the user 'What are 1 to 3 takeaways from this retro that can be made into actionable tasks to work on for the next retro?'

For Step 4, have a conversation with the user about what those tasks should be. Get feedback from the user, but also push back if the user's suggested tasks are not SMART (simple, measurable, achievable, relevant, time oriented).

Above all, make sure the vibe for your retro is fun and concise. Do not ramble or go on tangents about topics that will distract the user. Stick to the 4-step outline, and praise the user for their accomplishments.";

pub async fn assistant_flow(markdown_notes: String) -> Result<(), OpenAIError> {
    let mut messages = vec![
        ChatCompletionRequestSystemMessageArgs::default()
            .content(NAVI_INSTRUCTIONS)
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(format!("Here are my weekly notes:\n{}", markdown_notes).to_string())
            .build()?
            .into(),
    ];
    //create a client
    let client = Client::new();

    // main conversation loop, which consists of first asking the user for input
    // (except for the first loop iteration), then sending that input to the
    // assistant, and finally receiving the assistant's response and printing it
    loop {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(5120u32)
            .model("gpt-4o-mini")
            .messages(&*messages)
            .n(1) // only 1 response
            .build()?;

        let response = client.chat().create(request).await?;
        let new_message = response.choices[0].message.content.clone().unwrap();

        info!(target: "intelligence", "--- Response: {}\n", &new_message);

        messages.push(
            ChatCompletionRequestAssistantMessageArgs::default()
                .content(new_message)
                .build()?
                .into(),
        );

        info!(target: "intelligence", "--- Enter your input or type 'exit()' to exit");
        //get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        //break out of the loop if the user enters exit()
        if input.trim() == "exit()" {
            break;
        }

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(input)
                .build()?
                .into(),
        );
    }
    Ok(())
}
