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

const KEY_USER_NAMES: &str = "user_names";
const VEC_NAMES: [&str; 4] = ["John", "Jane", "Jim", "Jill"];

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = redis::Client::open(URL_REDIS)?;
    let mut connection = client.get_connection()?;

    let _ = store_user_with_id_1(&mut connection).await;

    let _ = rpush_and_lrange_user_names(VEC_NAMES, &mut connection).await;


    Ok(())
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

async fn store_user_with_id_1(
    connection: &mut redis::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    let user_id = 1;
    let url = format!("{}/{}", URL_USERS, user_id);
    let user: User = reqwest::get(&url).await?.json().await?;

    // Store user data in Redis
    let _: () = connection.hset(format!("user:{}", user.id), "name", user.name)?;
    let _: () = connection.hset(format!("user:{}", user.id), "username", user.username)?;
    let _: () = connection.hset(format!("user:{}", user.id), "email", user.email)?;

    println!("User data stored in Redis for user ID: {}", user.id);
    // └──> 🖥️ HGETALL user:1

    Ok(())
}

async fn rpush_and_lrange_user_names(
    names: [&str; 4],
    connection: &mut redis::Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    // Add multiple user names to the list
    for name in names {
        let _: () = connection.rpush(KEY_USER_NAMES, name)?;
        println!("Added user name: {}", name);
    }

    let names: Vec<String> = connection.lrange(KEY_USER_NAMES, 0, -1)?;
    println!("User names: {:?}", names);

    Ok(())
}

