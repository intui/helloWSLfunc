use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

extern crate handler;
extern crate diesel;

use self::handler::*;
use self::models::*;
use self::diesel::prelude::*;

#[tokio::main]
async fn main() {
    use handler::schema::posts::dsl::*;
    println!("Trace: main() started");
    
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("httpexample"))
        .and(warp::query::<HashMap<String, String>>())
        .map(move |p: HashMap<String, String>| match p.get("name") {
            Some(name) => {
                let connection = establish_connection();
                let results = posts.filter(published.eq(true))
                    .limit(5)
                    .load::<Post>(&connection)
                    .expect("Error loading posts");
            
                println!("Displaying {} posts", results.len());
                let results_len = results.len();
                
                for post in results {
                    println!("{}", post.title);
                    println!("----------\n");
                    println!("{}", post.body);
                }
                Response::builder().body(format!("Hello dear user {}! Nice to see you. We have {} results.", name, results_len))
            },
            None => Response::builder().body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::UNSPECIFIED, port)).await
}