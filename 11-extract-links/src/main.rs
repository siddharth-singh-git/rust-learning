use std::error::Error;
use select::document::Document;
use select::predicate::Name;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let res = reqwest::get("https://www.rust-lang.org/") // Send a GET request to the specified URL
    .await?  // .text() returns the body of the response as a String
    .text()  // await the future returned by .text() to get the actual String
    .await?;  // Print the HTML content of the page

    Document::from(res.as_str() // Create a Document from the HTML string
    ).find(Name("a"))// Find all <a> elements in the document
    .filter_map(|n| n.attr("href")) // For each <a> element, get the value of the "href" attribute, if it exists
    .for_each(|link| println!("{}", link)); // Print each link to the console


    //println!("{}", &res[..500]); // Print the first 500 characters of the HTML content for verification
    Ok(())
}
