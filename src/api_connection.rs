// use std::collections::HashMap;
use crate::models::Post;

#[tokio::main]
pub async fn get_data(url: &str) -> Result<Vec<Post>, Box<dyn std::error::Error>>  {
    let resp = reqwest::get(url)
        .await?
        .json::<Vec<Post>>()
        .await?;
    // println!("{:#?}", resp);
    Ok(resp)
}

#[tokio::main]
pub async fn post_data(url:&str,post: Post) -> Result<Post, Box<dyn std::error::Error>>  {
    let new_post = reqwest::Client::new()
        .post(url)
        .json(&post)
        .send()
        .await?
        .json::<Post>()
        .await?;
  Ok(new_post)   
}