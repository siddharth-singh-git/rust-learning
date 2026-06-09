use dotenv::dotenv;
use gemini_rust::Gemini;
use gemini_rust::Model;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // load env variables from .env file
    // Get API key from environment variable
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Create a Gemini client with default settings (Gemini 2.5 Flash)
    let client = Gemini::with_model(api_key, Model::Gemini3Flash)?;

    // Send a prompt to the model (e.g., gemini-2.5-flash)
    let response = client
        .generate_content()
        .with_system_prompt("You are a helpful assistant specializing in Rust programming.")
        .with_user_message("What makes Rust a good choice for systems programming?")
        .execute()
        .await?;

    println!("Response: {}", response.text());

    Ok(())
}
