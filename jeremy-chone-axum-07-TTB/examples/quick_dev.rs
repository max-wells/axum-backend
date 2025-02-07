use anyhow::Result;
use serde_json::json;

const LOCALHOST_8000: &str = "http://localhost:8000";

// * cargo watch -q -c -w examples/ -x "run --example quick_dev"

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client(LOCALHOST_8000)?;

	demo_basics(&hc).await?;
	demo_login(&hc).await?;
	demo_create_and_get_ticket(&hc).await?;

	Ok(())
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// 1. Basics
async fn demo_basics(hc: &httpc_test::Client) -> Result<()> {
	hc.do_get("/hello2/Mike").await?.print().await?;
	hc.do_get("/src/main.rs").await?.print().await?;

	Ok(())
}

// 2. Login
async fn demo_login(hc: &httpc_test::Client) -> Result<()> {
	const URL_API_LOGIN: &str = "/api/login";

	let req_login = hc.do_post(
		URL_API_LOGIN,
		json!({
			"username": "my_username",
			"password": "my_password"
		}),
	);
	req_login.await?.print().await?;

	Ok(())
}

// 3. Create Ticket
async fn demo_create_and_get_ticket(hc: &httpc_test::Client) -> Result<()> {
	const URL_API_TICKETS: &str = "/api/tickets";

	let req_create_ticket = hc.do_post(
		URL_API_TICKETS,
		json!({
			"title": "Ticket AAA"
		}),
	);
	req_create_ticket.await?.print().await?;

	hc.do_get(URL_API_TICKETS).await?.print().await?;

	Ok(())
}
