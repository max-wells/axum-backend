use anyhow::Result;
use serde_json::json;

const LOCALHOST_8000: &str = "http://localhost:8000";
const URL_API_LOGIN: &str = "/api/login";
const URL_API_TICKETS: &str = "/api/tickets";

// * cargo watch -q -c -w examples/ -x "run --example quick_dev"

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client(LOCALHOST_8000)?;

	// hc.do_get("/hello2/Mike").await?.print().await?;
	// hc.do_get("/src/main.rs").await?.print().await?;

	// ---------------- LOGIN ----------------

	let req_login = hc.do_post(
		URL_API_LOGIN,
		json!({
			"username": "my_uername",
			"password": "my_password"
		}),
	);
	req_login.await?.print().await?;

	// ---------------- CREATE TICKET ----------------

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
