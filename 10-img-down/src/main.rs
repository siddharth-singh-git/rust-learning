use std::error::Error;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let temp = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let res = reqwest::get(target).await?;

    let mut desc = {
        let fname = res
            .url() // Get the final URL after any redirects
            .path_segments() // Split the URL path into segments
            .and_then(|segments| segments.last()) // Get the last segment, which is typically the file name
            .and_then(|name| if name.is_empty() { None } else { Some(name) }) // Ensure the name is not empty
            .unwrap_or("tmp.bin"); // Fallback to "tmp.bin" if the file name cannot be determined

        println!("File to: download {}", fname);

        let fname = temp.path().join(fname);
        println!("File downloaded to: {:?}", fname);
        File::create(fname)?
    };
    let content = res.text().await?;
    copy(&mut content.as_bytes(), &mut desc)?;

    Ok(())
}
