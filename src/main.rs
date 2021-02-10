extern crate serde;


mod fizzbuzz;
mod models;
mod  api_connection;

use crate::models::Post;


fn main() {
//    fizzbuzz::for_loop();
//    fizzbuzz::while_loop();
//    fizzbuzz::infinite_loop();



 //get data from remote endpoint
let fetchdata = api_connection::get_data("https://jsonplaceholder.typicode.com/posts");
match fetchdata {
    Ok(result) => print!("fetched data successfully response {:#?}", result),
    Err(e) => print!("fetched data error  response {:?}", e)
};

let new_post = Post::new(None,"Reqwest.rs".into(),"https://docs.rs/reqwest".into(),1);


//post data to remote endpoint
let fetchdata = api_connection::post_data("https://jsonplaceholder.typicode.com/posts", new_post);
match fetchdata {
    Ok(result) => print!("data sent successfully response {:#?}", result),
    Err(e) => print!("data sent error  response {:?}", e)
};


}
