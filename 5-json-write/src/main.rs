use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragrpah: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("work with json in Rust"),
        author: String::from("sid"),
        paragrpah: vec![
            Paragraph {
                name: String::from("starting sentences"),
            },
            Paragraph {
                name: String::from("body here"),
            },
            Paragraph {
                name: String::from("ending"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("{}", json);
}
