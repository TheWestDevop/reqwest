use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    id:Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
    // origin:String
} impl Post {
    pub fn new(id:Option<i32>,title:String, body:String, user_id:i32) -> Post {
        Post {
          id,
          title,
          body,
          user_id
        }
    }
}