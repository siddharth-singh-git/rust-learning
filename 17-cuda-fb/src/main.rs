use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response{
    name : String,
}


#[tokio::main]
async fn main() {
    let user : User = User{
        name : "Siddharth".to_string(),
        email : "test1@gmail.com".to_string(),
        age : 20,
    };

    let firebase= Firebase::new("https://rust-cuda1-default-rtdb.firebaseio.com/").unwrap();

    let user = set_user(&user, &firebase).await;

    let users = get_users(&firebase).await;
    println!("{:?}", users);

    let mut user_choice = get_user(&firebase, &response.name).await;
    println!("{:?}", user_choice);

    let del = delete_user(&user, &firebase).await;
    println!("User deleted!");

    user.email = "updatedtest1@gmail.com".to_string();
    let update = update_user(&user, &firebase).await;
    println!("{:?}", update);

}

fn set_user(&user, &database) -> () {

}
