use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User{
    login : String,
    id : u32,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = format!("https://api.github.com/repos/{user}/{repo}/stargazers", 
        user = "rust-lang-nursery",
        repo = "rust-cookbook" );
    println!("Fetching stargazers from: {}", url);


    let client = reqwest::Client::new();
    let response = client.get(&url).header(USER_AGENT, "Rust API Client").send().await?;

    let user :Vec<User>= response.json().await?;
    println!("{:?}",user);
    for us in user {
    println!("{} ({})", us.login, us.id);
    }
    Ok(())
}
