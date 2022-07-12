use alloc::format;
use alloc::string::String;

use sentc_crypto::user;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/**
# Get the user input from the user client

This is used when the register endpoint should only be called from the backend and not the clients.

For full register see register()
*/
#[wasm_bindgen]
pub fn prepare_register(username: &str, password: &str) -> Result<String, JsValue>
{
	Ok(user::register(username, password)?)
}

/**
# Register a new user for the app

Do the full req incl. req.
No checking about spamming and just return the user id.
*/
#[wasm_bindgen]
pub async fn register(base_url: String, auth_token: String, username: String, password: String) -> Result<String, JsValue>
{
	let register_input = user::register(username.as_str(), password.as_str())?;

	let url = format!("{}/api/v1/register", base_url);

	let mut opts = RequestInit::new();
	opts.method("POST");
	opts.mode(RequestMode::NoCors);
	opts.body(Some(&JsValue::from_str(register_input.as_str())));

	let res = make_req(url.as_str(), auth_token.as_str(), &opts).await?;

	Ok(res)
}

/**
# Login the user to this app

*/
#[wasm_bindgen]
pub async fn login(base_url: String, auth_token: String, user_id: String, password: String) -> Result<String, JsValue>
{
	let user_id_input = user::prepare_login_start(user_id.as_str())?;

	let url = format!("{}/api/v1/pre_login", base_url);

	let mut opts = RequestInit::new();
	opts.method("POST");
	opts.mode(RequestMode::NoCors);
	opts.body(Some(&JsValue::from_str(user_id_input.as_str())));

	let res = make_req(url.as_str(), auth_token.as_str(), &opts).await?;

	//prepare the login, the auth key is already in the right json format for the server
	let (auth_key, master_key_encryption_key) = user::prepare_login(password.as_str(), res.as_str())?;

	let url = format!("{}/api/v1/login", base_url);

	//send the auth key to the server
	let mut opts = RequestInit::new();
	opts.method("POST");
	opts.mode(RequestMode::NoCors);
	opts.body(Some(&JsValue::from_str(auth_key.as_str())));

	//the done login server output
	let server_output = make_req(url.as_str(), auth_token.as_str(), &opts).await?;

	let keys = user::done_login(master_key_encryption_key.as_str(), server_output.as_str())?;

	Ok(keys)
}

async fn make_req(url: &str, bearer_header: &str, req_opts: &RequestInit) -> Result<String, JsValue>
{
	let request = Request::new_with_str_and_init(url, req_opts)?;

	request
		.headers()
		.set("Authorization", format!("Bearer {}", bearer_header).as_str())?;

	request.headers().set("Content-Type", "application/json")?;

	let window = web_sys::window().unwrap();
	let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
	let resp: Response = resp_value.dyn_into().unwrap();
	let text = JsFuture::from(resp.text()?).await?;
	let server_output = match text.as_string() {
		Some(v) => v,
		None => return Err(JsValue::from_str("String parsing failed")),
	};

	if resp.status() >= 400 {
		//handle server errs
		return Err(JsValue::from_str(server_output.as_str()));
	}

	Ok(server_output)
}
