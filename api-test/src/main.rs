use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("http://127.0.0.1:3000/");
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?.text().await?;
    println!("{:?}",response);

    // let users: Vec<User> = response.json().await?;
    // println!("{:?}", users);
    Ok(())
}