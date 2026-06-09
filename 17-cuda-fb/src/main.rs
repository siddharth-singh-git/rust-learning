use ::std::collections::HashMap;

use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {
    let user: User = User {
        name: "Siddharth".to_string(),
        email: "test1@gmail.com".to_string(),
        age: 20,
    };

    let firebase = Firebase::new("https://rust-cuda1-default-rtdb.firebaseio.com/").unwrap();

    let response = set_user(&user, &firebase).await;

    let users = get_users(&firebase).await;
    println!("{:?}", users);

    let mut user_choice = get_user(&firebase, &response.name).await;
    println!("{:?}", user_choice);

    user_choice.email = "updatedtest1@gmail.com".to_string();
    let update = update_user(&response.name, &firebase, &user_choice).await;
    println!("{:?}", update);

    let _del = delete_user(&response.name, &firebase).await;
    println!("User deleted!");
}

async fn set_user(user: &User, firebase_client: &Firebase) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return str_to_res(&_users.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("{:?}", users);
    return users.unwrap();
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(id: &String, firebase_client: &Firebase, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return str_to_user(&_user.unwrap().data);
}

async fn delete_user(id: &String, firebase_client: &Firebase) {
    let firebase = firebase_client.at("users").at(&id);
    let _del = firebase.delete().await;
}

// String to Response
fn str_to_res(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

// String to User
fn str_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
