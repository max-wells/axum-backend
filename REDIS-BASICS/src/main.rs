use redis::Commands;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
}

const URL_USERS: &str = "https://jsonplaceholder.typicode.com/users";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = store_user_with_id_1().await;

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
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut connection = client.get_connection()?;

    // Store user data in Redis
    let _: () = connection.hset(format!("user:{}", user.id), "name", user.name)?;
    let _: () = connection.hset(format!("user:{}", user.id), "username", user.username)?;
    let _: () = connection.hset(format!("user:{}", user.id), "email", user.email)?;

    println!("User data stored in Redis for user ID: {}", user.id);
    // └──> 🖥️ HGETALL user:1

    Ok(())
}