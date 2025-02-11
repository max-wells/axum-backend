#![allow(dead_code)]

use redis::Commands;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
}

const URL_REDIS: &str = "redis://127.0.0.1/";
const URL_USERS: &str = "https://jsonplaceholder.typicode.com/users";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let _ = store_user_with_id_1().await;

    let names = vec!["John", "Jane", "Jim", "Jill"];
    let _ = add_user_names(names).await;

    let names = get_user_names().await?;
    println!("User names: {:?}", names);

    Ok(())
}



/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

async fn store_user_with_id_1() -> Result<(), Box<dyn std::error::Error>> {
    let user_id = 1;
    let url = format!("{}/{}", URL_USERS, user_id);
    let user: User = reqwest::get(&url).await?.json().await?;

    // Connect to Redis
    let client = redis::Client::open(URL_REDIS)?;
    let mut connection = client.get_connection()?;

    // Store user data in Redis
    let _: () = connection.hset(format!("user:{}", user.id), "name", user.name)?;
    let _: () = connection.hset(format!("user:{}", user.id), "username", user.username)?;
    let _: () = connection.hset(format!("user:{}", user.id), "email", user.email)?;

    println!("User data stored in Redis for user ID: {}", user.id);
    // └──> 🖥️ HGETALL user:1

    Ok(())
}



async fn add_user_names(names: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Redis
    let client = redis::Client::open(URL_REDIS)?;
    let mut connection = client.get_connection()?;

    // Add multiple user names to the list
    for name in names {
        let _: () = connection.rpush("user_names", name)?;
        println!("Added user name: {}", name);
    }

    Ok(())
}


async fn get_user_names() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Connect to Redis
    let client = redis::Client::open(URL_REDIS)?;
    let mut connection = client.get_connection()?;

    // Retrieve all user names from the list
    let names: Vec<String> = connection.lrange("user_names", 0, -1)?;
    Ok(names)
}