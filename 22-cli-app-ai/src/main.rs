use dotenv::dotenv;
use gemini_rust::Gemini;
use gemini_rust::Model;
use spinners::{Spinner, Spinners};
use std::env;
use std::io::stdin;
use std::io::{Write, stdout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // load env variables from .env file
    // Get API key from environment variable
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Create a Gemini client with default settings (Gemini 2.5 Flash)
    let client = Gemini::with_model(api_key, Model::Gemini3Flash)?;

    let preamble = "Generate a Sql code for the given statement. Only give the sql code . Avoid any explanaiton of not asked. Just give the SQL code.  Return ONLY the SQL query. No explanation, no markdown, no code fences, no extra text."; // preamble for the prompt

    // Send a prompt to the model (e.g., gemini-2.5-flash)

    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> "); // println flushes it, print doesn't
        stdout().flush().unwrap(); // flush stdout
        let mut user_text = String::new();

        stdin().read_line(&mut user_text).expect("Failed to read");
        println!("");

        if user_text.trim().eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }
        let sp = Spinner::new(&Spinners::Dots12, "\t\tOpenAI is Thinking...".into()); // spinner from spinners crate that displays "Thinking..." next to a spinner

        let response = client
            .generate_content()
            .with_system_prompt(preamble)
            .with_user_message(user_text)
            .with_max_output_tokens(1024)
            .execute()
            .await?;

        // we've got response by now, stop the spinner

        // stopping the spinner
        sp.stop();

        println!(""); // println!: same as print! but a newline is appended.

        println!("Response: {}", response.text());
    }

    Ok(())
}
