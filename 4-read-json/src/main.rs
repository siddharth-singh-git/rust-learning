use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
    "article": "work with json in Rust",
    "author": "sid",
    "paragraph": [
        {
        "name": "starting sentences"
        },
        {
        "name": "body here"
        },
        {
        "name": "ending"
        }
    ]
    }
    "#;

    let parsed: Article = read_json(json);

    println!("The name of the article is {}", parsed.paragraph[0].name);
    println!("The body of the article is {}", parsed.paragraph[1].name);
    println!("The end of the article is {}", parsed.paragraph[2].name);
}

fn read_json(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}
