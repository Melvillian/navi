use std::env;

use async_openai::{
    config::OpenAIConfig,
    types::{
        AssistantToolFileSearchResources, AssistantToolsFileSearch, CreateAssistantRequestArgs,
        CreateFileRequest, CreateMessageRequestArgs, CreateRunRequestArgs, CreateThreadRequestArgs,
        CreateVectorStoreRequest, FilePurpose, InputSource, MessageContent, MessageRole,
        ModifyAssistantRequest, RunStatus,
    },
    Client,
};
use log::{debug, info};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IntelligenceError {
    #[error("OpenAI API error: {0}")]
    OpenAI(#[from] async_openai::error::OpenAIError),
    #[error("Rate limit exceeded")]
    RateLimit,
    #[error("Token limit exceeded")]
    TokenLimit,
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("Network error: {0}")]
    Network(String),
}

const NAVI_INSTRUCTIONS: &str = "> I want you to lead a retrospective for me using the information from my weekly notes. You have a vast experience with running retrospectives, and have read all of the most important writing on how to deliver fun, useful, and high-signal retro's from people like Esther Derby and Ben Linders.

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

const ASSISTANT_NAME: &str = "Navi Digital Mentor";

pub async fn assistant_flow(markdown_notes: String) -> Result<(), IntelligenceError> {
    let query = [("limit", "1")]; //limit the list responses to 1 message

    //create a client
    let client =
        Client::with_config(OpenAIConfig::new().with_api_key(env::var("OPENAI_API_KEY").unwrap()));

    //create a thread for the conversation
    let thread_request = CreateThreadRequestArgs::default().build()?;
    let thread = client.threads().create(thread_request.clone()).await?;
    debug!(target: "intelligence", "Thread created with id: {}", thread.id);

    // //ask the user for the name of the assistant
    // info!(target: "intelligence", "--- Enter the name of your assistant");
    // //get user input
    // let mut assistant_name = String::new();
    // std::io::stdin().read_line(&mut assistant_name).unwrap();

    // //ask the user for the instruction set for the assistant
    // info!(target: "intelligence", "--- Enter the instruction set for your new assistant");
    // //get user input
    // let mut instructions = "".to_string();

    //create the assistant
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(&ASSISTANT_NAME.to_string())
        .instructions(&NAVI_INSTRUCTIONS.to_string())
        .tools(vec![AssistantToolsFileSearch::default().into()])
        .model("gpt-4o-mini")
        .build()?;
    let assistant = client.assistants().create(assistant_request).await?;
    debug!(target: "intelligence", "Created assistant with id: {}", assistant.id);

    // upload file to add to vector store
    let openai_file = client
        .files()
        .create(CreateFileRequest {
            file: async_openai::types::FileInput {
                source: InputSource::VecU8 {
                    filename: "weekly_notes.md".to_string(),
                    vec: markdown_notes.as_bytes().to_vec(),
                },
            },
            purpose: FilePurpose::Assistants,
        })
        .await?;

    // Create a vector store called "Navi Weekly Notes"
    // add uploaded file to vector store
    let vector_store = client
        .vector_stores()
        .create(CreateVectorStoreRequest {
            name: Some("Navi Weekly Notes".into()),
            file_ids: Some(vec![openai_file.id.clone()]),
            ..Default::default()
        })
        .await?;
    debug!(target: "intelligence", "Created vector store with id: {}", vector_store.id);

    //
    // Step 3: Update the assistant to to use the new Vector Store
    //

    let assistant = client
        .assistants()
        .update(
            &assistant.id,
            ModifyAssistantRequest {
                tool_resources: Some(
                    AssistantToolFileSearchResources {
                        vector_store_ids: vec![vector_store.id.clone()],
                    }
                    .into(),
                ),
                ..Default::default()
            },
        )
        .await?;
    //get the id of the assistant
    let assistant_id = &assistant.id;

    let mut is_first_loop_iteration = true;

    loop {
        let input = if is_first_loop_iteration {
            is_first_loop_iteration = false;
            "Let's begin the retro".to_string()
        } else {
            info!(target: "intelligence", "--- Enter your input or type 'exit()' to exit");
            //get user input
            let mut user_input = String::new();
            std::io::stdin().read_line(&mut user_input).unwrap();
            user_input
        };

        //break out of the loop if the user enters exit()
        if input.trim() == "exit()" {
            break;
        }

        //create a message for the thread
        let message = CreateMessageRequestArgs::default()
            .role(MessageRole::User)
            .content(input)
            .build()?;

        //attach message to the thread
        let _message_obj = client
            .threads()
            .messages(&thread.id)
            .create(message)
            .await?;

        //create a run for the thread
        let run_request = CreateRunRequestArgs::default()
            .assistant_id(assistant_id)
            .build()?;
        let run = client
            .threads()
            .runs(&thread.id)
            .create(run_request)
            .await?;

        //wait for the run to complete
        let mut awaiting_response = true;
        while awaiting_response {
            //retrieve the run
            let run = client.threads().runs(&thread.id).retrieve(&run.id).await?;
            //check the status of the run
            match run.status {
                RunStatus::Completed => {
                    awaiting_response = false;
                    // once the run is completed we
                    // get the response from the run
                    // which will be the first message
                    // in the thread

                    //retrieve the response from the run
                    let response = client
                        .threads()
                        .messages(&thread.id)
                        .list(&query)
                        .await
                        .map_err(|e| match e {
                            e if e.to_string().contains("rate limit") => {
                                IntelligenceError::RateLimit
                            }
                            e if e.to_string().contains("token limit") => {
                                IntelligenceError::TokenLimit
                            }
                            e if e.to_string().contains("invalid api key") => {
                                IntelligenceError::InvalidApiKey
                            }
                            e if e.to_string().contains("network") => {
                                IntelligenceError::Network(e.to_string())
                            }
                            e => IntelligenceError::OpenAI(e),
                        })?;
                    //get the message id from the response
                    let message_id = response.data.first().unwrap().id.clone();
                    //get the message from the response
                    let message = client
                        .threads()
                        .messages(&thread.id)
                        .retrieve(&message_id)
                        .await?;
                    //get the content from the message
                    let content = message.content.first().unwrap();
                    //get the text from the content
                    let text = match content {
                        MessageContent::Text(text) => text.text.value.clone(),
                        MessageContent::ImageFile(_) | MessageContent::ImageUrl(_) => {
                            panic!("imaged are not expected in this example");
                        }
                        MessageContent::Refusal(refusal) => refusal.refusal.clone(),
                    };
                    //print the text
                    info!(target: "intelligence", "--- Response: {}\n", text);
                }
                RunStatus::Failed => {
                    awaiting_response = false;
                    info!(target: "intelligence", "--- Run Failed: {:#?}", run);
                }
                RunStatus::Queued => {
                    info!(target: "intelligence", "--- Run Queued");
                }
                RunStatus::Cancelling => {
                    info!(target: "intelligence", "--- Run Cancelling");
                }
                RunStatus::Cancelled => {
                    info!(target: "intelligence", "--- Run Cancelled");
                }
                RunStatus::Expired => {
                    info!(target: "intelligence", "--- Run Expired");
                }
                RunStatus::RequiresAction => {
                    info!(target: "intelligence", "--- Run Requires Action");
                }
                RunStatus::InProgress => {
                    info!(target: "intelligence", "--- Waiting for response from OpenAI ...");
                }
                RunStatus::Incomplete => {
                    info!(target: "intelligence", "--- Run Incomplete");
                }
            }
            //wait for 1 second before checking the status again
            debug!(target: "intelligence", "--- Waiting for 1 second");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    //once we have broken from the main loop we can delete the assistant and thread
    client.assistants().delete(assistant_id).await?;
    client.threads().delete(&thread.id).await?;

    Ok(())
}
