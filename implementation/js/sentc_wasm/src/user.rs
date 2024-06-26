use alloc::string::{String, ToString};

use sentc_crypto::{user, util_req_full};
use wasm_bindgen::prelude::*;

use crate::group;

#[wasm_bindgen]
pub struct GeneratedRegisterData
{
	identifier: String,
	password: String,
}

#[wasm_bindgen]
impl GeneratedRegisterData
{
	pub fn get_identifier(&self) -> String
	{
		self.identifier.clone()
	}

	pub fn get_password(&self) -> String
	{
		self.password.clone()
	}
}

#[wasm_bindgen]
pub struct DeviceKeyData
{
	private_key: String, //Base64 exported keys
	public_key: String,
	sign_key: String,
	verify_key: String,
	exported_public_key: String,
	exported_verify_key: String,
}

impl From<sentc_crypto::entities::user::DeviceKeyDataExport> for DeviceKeyData
{
	fn from(key: sentc_crypto::entities::user::DeviceKeyDataExport) -> Self
	{
		Self {
			private_key: key.private_key,
			public_key: key.public_key,
			sign_key: key.sign_key,
			verify_key: key.verify_key,
			exported_public_key: key.exported_public_key,
			exported_verify_key: key.exported_verify_key,
		}
	}
}

#[wasm_bindgen]
pub struct UserKeyData
{
	private_key: String,
	public_key: String,
	group_key: String,
	time: String,
	group_key_id: String,
	sign_key: String,
	verify_key: String,
	exported_public_key: String,
	exported_public_key_sig_key_id: Option<String>,
	exported_verify_key: String,
}

impl From<sentc_crypto::entities::user::UserKeyDataExport> for UserKeyData
{
	fn from(value: sentc_crypto::entities::user::UserKeyDataExport) -> Self
	{
		Self {
			private_key: value.private_key,
			public_key: value.public_key,
			group_key: value.group_key,
			time: value.time.to_string(),
			group_key_id: value.group_key_id,
			sign_key: value.sign_key,
			verify_key: value.verify_key,
			exported_public_key: value.exported_public_key,
			exported_public_key_sig_key_id: value.exported_public_key_sig_key_id,
			exported_verify_key: value.exported_verify_key,
		}
	}
}

#[wasm_bindgen]
impl UserKeyData
{
	pub fn get_private_key(&self) -> String
	{
		self.private_key.clone()
	}

	pub fn get_public_key(&self) -> String
	{
		self.public_key.clone()
	}

	pub fn get_group_key(&self) -> String
	{
		self.group_key.clone()
	}

	pub fn get_time(&self) -> String
	{
		self.time.clone()
	}

	pub fn get_group_key_id(&self) -> String
	{
		self.group_key_id.clone()
	}

	pub fn get_sign_key(&self) -> String
	{
		self.sign_key.clone()
	}

	pub fn get_verify_key(&self) -> String
	{
		self.verify_key.clone()
	}

	pub fn get_exported_public_key(&self) -> String
	{
		self.exported_public_key.clone()
	}

	pub fn get_exported_public_key_sig_key_id(&self) -> Option<String>
	{
		self.exported_public_key_sig_key_id.clone()
	}

	pub fn get_exported_verify_key(&self) -> String
	{
		self.exported_verify_key.clone()
	}
}

#[wasm_bindgen]
pub struct PrepareLoginOtpOutput
{
	master_key: String,
	auth_key: String,
}

impl From<util_req_full::user::PrepareLoginOtpOutput> for PrepareLoginOtpOutput
{
	fn from(value: util_req_full::user::PrepareLoginOtpOutput) -> Self
	{
		Self {
			master_key: value.master_key,
			auth_key: value.auth_key,
		}
	}
}

#[wasm_bindgen]
pub struct UserLoginOut
{
	user_data: Option<UserData>,

	mfa: Option<PrepareLoginOtpOutput>,
}

impl From<util_req_full::user::PreLoginOutExport> for UserLoginOut
{
	fn from(value: util_req_full::user::PreLoginOutExport) -> Self
	{
		match value {
			util_req_full::user::PreLoginOutExport::Direct(d) => {
				Self {
					mfa: None,
					user_data: Some(d.into()),
				}
			},
			util_req_full::user::PreLoginOutExport::Otp(d) => {
				Self {
					user_data: None,
					mfa: Some(d.into()),
				}
			},
		}
	}
}

#[wasm_bindgen]
impl UserLoginOut
{
	pub fn get_user_keys(&self) -> JsValue
	{
		self.user_data.as_ref().map(|o| o.user_keys.clone()).into()
	}

	pub fn get_device_private_key(&self) -> Option<String>
	{
		self.user_data
			.as_ref()
			.map(|o| o.device_keys.private_key.clone())
	}

	pub fn get_device_public_key(&self) -> Option<String>
	{
		self.user_data
			.as_ref()
			.map(|o| o.device_keys.public_key.clone())
	}

	pub fn get_device_sign_key(&self) -> Option<String>
	{
		self.user_data
			.as_ref()
			.map(|o| o.device_keys.sign_key.clone())
	}

	pub fn get_device_verify_key(&self) -> Option<String>
	{
		self.user_data
			.as_ref()
			.map(|o| o.device_keys.verify_key.clone())
	}

	pub fn get_device_exported_public_key(&self) -> Option<String>
	{
		self.user_data
			.as_ref()
			.map(|o| o.device_keys.exported_public_key.clone())
	}

	pub fn get_device_exported_verify_key(&self) -> Option<String>
	{
		self.user_data
			.as_ref()
			.map(|o| o.device_keys.exported_verify_key.clone())
	}

	pub fn get_jwt(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.jwt.clone())
	}

	pub fn get_refresh_token(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.refresh_token.clone())
	}

	pub fn get_id(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.user_id.clone())
	}

	pub fn get_device_id(&self) -> Option<String>
	{
		self.user_data.as_ref().map(|o| o.device_id.clone())
	}

	pub fn get_hmac_keys(&self) -> JsValue
	{
		self.user_data.as_ref().map(|o| o.hmac_keys.clone()).into()
	}

	pub fn get_mfa_master_key(&self) -> Option<String>
	{
		self.mfa.as_ref().map(|o| o.master_key.clone())
	}

	pub fn get_mfa_auth_key(&self) -> Option<String>
	{
		self.mfa.as_ref().map(|o| o.auth_key.clone())
	}
}

#[wasm_bindgen]
pub struct UserData
{
	device_keys: DeviceKeyData,
	user_keys: JsValue,

	jwt: String,
	refresh_token: String,
	user_id: String,
	device_id: String,
	hmac_keys: JsValue,
}

impl From<sentc_crypto::entities::user::UserDataExport> for UserData
{
	fn from(data: sentc_crypto::entities::user::UserDataExport) -> Self
	{
		Self {
			device_keys: data.device_keys.into(),
			user_keys: JsValue::from_serde(&data.user_keys).unwrap(),
			jwt: data.jwt,
			refresh_token: data.refresh_token,
			user_id: data.user_id,
			device_id: data.device_id,
			hmac_keys: JsValue::from_serde(&data.hmac_keys).unwrap(),
		}
	}
}

#[wasm_bindgen]
impl UserData
{
	pub fn get_user_keys(&self) -> JsValue
	{
		self.user_keys.clone()
	}

	pub fn get_device_private_key(&self) -> String
	{
		self.device_keys.private_key.clone()
	}

	pub fn get_device_public_key(&self) -> String
	{
		self.device_keys.public_key.clone()
	}

	pub fn get_device_sign_key(&self) -> String
	{
		self.device_keys.sign_key.clone()
	}

	pub fn get_device_verify_key(&self) -> String
	{
		self.device_keys.verify_key.clone()
	}

	pub fn get_device_exported_public_key(&self) -> String
	{
		self.device_keys.exported_public_key.clone()
	}

	pub fn get_device_exported_verify_key(&self) -> String
	{
		self.device_keys.exported_verify_key.clone()
	}

	pub fn get_jwt(&self) -> String
	{
		self.jwt.clone()
	}

	pub fn get_refresh_token(&self) -> String
	{
		self.refresh_token.clone()
	}

	pub fn get_id(&self) -> String
	{
		self.user_id.clone()
	}

	pub fn get_device_id(&self) -> String
	{
		self.device_id.clone()
	}

	pub fn get_hmac_keys(&self) -> JsValue
	{
		self.hmac_keys.clone()
	}
}

#[wasm_bindgen]
pub struct UserPublicData
{
	public_key: String,
	public_key_id: String,
	verify_key: String,
	verify_key_id: String,
}

#[wasm_bindgen]
impl UserPublicData
{
	pub fn get_verify_key(&self) -> String
	{
		self.verify_key.clone()
	}

	pub fn get_public_key(&self) -> String
	{
		self.public_key.clone()
	}

	pub fn get_verify_key_id(&self) -> String
	{
		self.verify_key_id.clone()
	}

	pub fn get_public_key_id(&self) -> String
	{
		self.public_key_id.clone()
	}
}

#[wasm_bindgen]
pub struct UserPublicKeyData
{
	public_key: String,
	public_key_id: String,
	public_key_sig_key_id: Option<String>,
}

#[wasm_bindgen]
impl UserPublicKeyData
{
	pub fn get_public_key(&self) -> String
	{
		self.public_key.clone()
	}

	pub fn get_public_key_id(&self) -> String
	{
		self.public_key_id.clone()
	}

	pub fn get_public_key_sig_key_id(&self) -> Option<String>
	{
		self.public_key_sig_key_id.clone()
	}
}

#[wasm_bindgen]
pub struct UserVerifyKeyData
{
	verify_key: String,
	verify_key_id: String,
}

#[wasm_bindgen]
impl UserVerifyKeyData
{
	pub fn get_verify_key(&self) -> String
	{
		self.verify_key.clone()
	}

	pub fn get_verify_key_id(&self) -> String
	{
		self.verify_key_id.clone()
	}
}

#[wasm_bindgen]
pub struct PrepareLoginOutput
{
	auth_key: String,
	master_key_encryption_key: String,
}

#[wasm_bindgen]
impl PrepareLoginOutput
{
	pub fn get_auth_key(&self) -> String
	{
		self.auth_key.clone()
	}

	pub fn get_master_key_encryption_key(&self) -> String
	{
		self.master_key_encryption_key.clone()
	}
}

#[wasm_bindgen]
pub struct UserInitServerOutput
{
	jwt: String,
	invites: JsValue,
}

#[wasm_bindgen]
impl UserInitServerOutput
{
	pub fn get_jwt(&self) -> String
	{
		self.jwt.clone()
	}

	pub fn get_invites(&self) -> JsValue
	{
		self.invites.clone()
	}
}

/**
# Check if the identifier is available

but without making a request
*/
#[wasm_bindgen]
pub fn prepare_check_user_identifier_available(user_identifier: &str) -> Result<String, JsValue>
{
	Ok(user::prepare_check_user_identifier_available(user_identifier)?)
}

/**
# Validates the response if the identifier is available

but without making a request
 */
#[wasm_bindgen]
pub fn done_check_user_identifier_available(server_output: &str) -> Result<bool, JsValue>
{
	Ok(user::done_check_user_identifier_available(server_output)?)
}

#[wasm_bindgen]
pub fn generate_user_register_data() -> Result<GeneratedRegisterData, JsValue>
{
	let (identifier, password) = user::generate_user_register_data()?;

	Ok(GeneratedRegisterData {
		identifier,
		password,
	})
}

/**
# Get the user input from the user client

This is used when the register endpoint should only be called from the backend and not the clients.

For full register see register()
*/
#[wasm_bindgen]
pub fn prepare_register(user_identifier: &str, password: &str) -> Result<String, JsValue>
{
	Ok(user::register(user_identifier, password)?)
}

/**
# Validates the response of register

Returns the new user id
*/
#[wasm_bindgen]
pub fn done_register(server_output: &str) -> Result<String, JsValue>
{
	Ok(user::done_register(server_output)?)
}

/**
# Register a new user for the app

Do the full req incl. req.
No checking about spamming and just return the user id.
*/
#[wasm_bindgen]
pub async fn register(base_url: String, auth_token: String, user_identifier: String, password: String) -> Result<String, JsValue>
{
	Ok(util_req_full::user::register(base_url, &auth_token, &user_identifier, &password).await?)
}

#[wasm_bindgen]
pub fn prepare_register_device_start(device_identifier: &str, password: &str) -> Result<String, JsValue>
{
	Ok(user::prepare_register_device_start(device_identifier, password)?)
}

#[wasm_bindgen]
pub fn done_register_device_start(server_output: &str) -> Result<(), JsValue>
{
	Ok(user::done_register_device_start(server_output)?)
}

#[wasm_bindgen]
pub async fn register_device_start(base_url: String, auth_token: String, device_identifier: String, password: String) -> Result<String, JsValue>
{
	Ok(util_req_full::user::register_device_start(base_url, &auth_token, &device_identifier, &password).await?)
}

#[wasm_bindgen]
pub struct PreRegisterDeviceData
{
	input: String,
	exported_public_key: String,
}

#[wasm_bindgen]
impl PreRegisterDeviceData
{
	pub fn get_input(&self) -> String
	{
		self.input.clone()
	}

	pub fn get_public_key(&self) -> String
	{
		self.exported_public_key.clone()
	}
}

#[wasm_bindgen]
pub struct RegisterDeviceData
{
	session_id: String,
	exported_public_key: String,
}

#[wasm_bindgen]
impl RegisterDeviceData
{
	pub fn get_session_id(&self) -> String
	{
		self.session_id.clone()
	}

	pub fn get_public_key(&self) -> String
	{
		self.exported_public_key.clone()
	}
}

#[wasm_bindgen]
pub fn prepare_register_device(server_output: &str, user_keys: &str, key_count: i32) -> Result<PreRegisterDeviceData, JsValue>
{
	let key_session = key_count > 50;

	let (input, exported_public_key) = user::prepare_register_device(server_output, user_keys, key_session)?;

	Ok(PreRegisterDeviceData {
		input,
		exported_public_key,
	})
}

#[wasm_bindgen]
pub async fn register_device(
	base_url: String,
	auth_token: String,
	jwt: String,
	server_output: String,
	key_count: i32,
	user_keys: String,
) -> Result<RegisterDeviceData, JsValue>
{
	let (out, exported_public_key) = util_req_full::user::register_device(
		base_url,
		auth_token.as_str(),
		jwt.as_str(),
		server_output.as_str(),
		key_count,
		user_keys.as_str(),
	)
	.await?;

	let session_id = out.unwrap_or_default();

	Ok(RegisterDeviceData {
		session_id,
		exported_public_key,
	})
}

#[wasm_bindgen]
pub async fn user_device_key_session_upload(
	base_url: String,
	auth_token: String,
	jwt: String,
	session_id: String,
	user_public_key: String,
	group_keys: String,
) -> Result<(), JsValue>
{
	Ok(util_req_full::user::device_key_session(
		base_url,
		auth_token.as_str(),
		jwt.as_str(),
		session_id.as_str(),
		user_public_key.as_str(),
		group_keys.as_str(),
	)
	.await?)
}

/**
# Login the user to this app

Does the login requests. 1. for auth, 2nd to get the keys.

If there are more data in the backend, then it is possible to call it via the jwt what is returned by the done login request.

The other backend can validate the jwt
*/
#[wasm_bindgen]
pub async fn login(base_url: String, auth_token: String, user_identifier: String, password: String) -> Result<UserLoginOut, JsValue>
{
	let data = util_req_full::user::login(
		base_url,
		auth_token.as_str(),
		user_identifier.as_str(),
		password.as_str(),
	)
	.await?;

	Ok(data.into())
}

#[wasm_bindgen]
pub async fn mfa_login(
	base_url: String,
	auth_token: String,
	master_key_encryption: String,
	auth_key: String,
	user_identifier: String,
	token: String,
	recovery: bool,
) -> Result<UserData, JsValue>
{
	let data = util_req_full::user::mfa_login(
		base_url,
		&auth_token,
		&master_key_encryption,
		auth_key,
		user_identifier,
		token,
		recovery,
	)
	.await?;

	Ok(data.into())
}

#[wasm_bindgen]
pub fn done_fetch_user_key(private_key: &str, server_output: &str) -> Result<UserKeyData, JsValue>
{
	let data = user::done_key_fetch(private_key, server_output)?;

	Ok(data.into())
}

#[wasm_bindgen]
pub async fn get_fresh_jwt(
	base_url: String,
	auth_token: String,
	user_identifier: String,
	password: String,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> Result<String, JsValue>
{
	Ok(util_req_full::user::get_fresh_jwt(
		base_url,
		&auth_token,
		&user_identifier,
		&password,
		mfa_token,
		mfa_recovery,
	)
	.await?)
}

#[wasm_bindgen]
pub async fn refresh_jwt(base_url: String, auth_token: String, jwt: String, refresh_token: String) -> Result<String, JsValue>
{
	let out = util_req_full::user::refresh_jwt(base_url, auth_token.as_str(), jwt.as_str(), refresh_token).await?;

	Ok(out)
}

#[wasm_bindgen]
pub async fn init_user(base_url: String, auth_token: String, jwt: String, refresh_token: String) -> Result<UserInitServerOutput, JsValue>
{
	let out = util_req_full::user::init_user(base_url, auth_token.as_str(), jwt.as_str(), refresh_token).await?;

	Ok(UserInitServerOutput {
		jwt: out.jwt,
		invites: JsValue::from_serde(&out.invites).unwrap(),
	})
}

#[wasm_bindgen]
pub async fn reset_password(
	base_url: String,
	auth_token: String,
	jwt: String,
	new_password: String,
	decrypted_private_key: String,
	decrypted_sign_key: String,
) -> Result<(), JsValue>
{
	Ok(util_req_full::user::reset_password(
		base_url,
		auth_token.as_str(),
		jwt.as_str(),
		new_password.as_str(),
		decrypted_private_key.as_str(),
		decrypted_sign_key.as_str(),
	)
	.await?)
}

#[wasm_bindgen]
pub async fn change_password(
	base_url: String,
	auth_token: String,
	user_identifier: String,
	old_password: String,
	new_password: String,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> Result<(), JsValue>
{
	Ok(util_req_full::user::change_password(
		base_url,
		auth_token.as_str(),
		user_identifier.as_str(),
		old_password.as_str(),
		new_password.as_str(),
		mfa_token,
		mfa_recovery,
	)
	.await?)
}

#[wasm_bindgen]
pub async fn delete_user(base_url: String, auth_token: String, fresh_jwt: String) -> Result<(), JsValue>
{
	Ok(util_req_full::user::delete(base_url, auth_token.as_str(), &fresh_jwt).await?)
}

#[wasm_bindgen]
pub async fn delete_device(base_url: String, auth_token: String, fresh_jwt: String, device_id: String) -> Result<(), JsValue>
{
	Ok(util_req_full::user::delete_device(base_url, auth_token.as_str(), &fresh_jwt, device_id.as_str()).await?)
}

#[wasm_bindgen]
pub fn user_prepare_user_identifier_update(user_identifier: String) -> Result<String, JsValue>
{
	Ok(user::prepare_user_identifier_update(user_identifier)?)
}

//__________________________________________________________________________________________________

#[wasm_bindgen]
pub fn user_extract_public_key_data(res: &str) -> Result<UserPublicKeyData, JsValue>
{
	let (public_key, public_key_id, public_key_sig_key_id) = sentc_crypto::util::public::import_public_key_from_string_into_export_string(res)?;

	Ok(UserPublicKeyData {
		public_key,
		public_key_id,
		public_key_sig_key_id,
	})
}

#[wasm_bindgen]
pub fn user_extract_verify_key_data(res: &str) -> Result<String, JsValue>
{
	Ok(sentc_crypto::util::public::import_verify_key_from_string_into_export_string(res)?.0)
}

//__________________________________________________________________________________________________

#[wasm_bindgen]
pub async fn user_key_rotation(
	base_url: String,
	auth_token: String,
	jwt: String,
	public_device_key: String,
	pre_user_key: String,
) -> Result<String, JsValue>
{
	Ok(util_req_full::user::key_rotation(base_url, &auth_token, &jwt, &public_device_key, &pre_user_key).await?)
}

#[wasm_bindgen]
pub async fn user_pre_done_key_rotation(base_url: String, auth_token: String, jwt: String) -> Result<JsValue, JsValue>
{
	let out = util_req_full::user::prepare_done_key_rotation(base_url, auth_token.as_str(), jwt.as_str()).await?;

	Ok(JsValue::from_serde(&out).unwrap())
}

#[wasm_bindgen]
pub fn user_get_done_key_rotation_server_input(server_output: &str) -> Result<group::KeyRotationInput, JsValue>
{
	let out = sentc_crypto::group::get_done_key_rotation_server_input(server_output)?;

	Ok(out.into())
}

#[wasm_bindgen]
pub async fn user_finish_key_rotation(
	base_url: String,
	auth_token: String,
	jwt: String,
	server_output: String,
	pre_group_key: String,
	public_key: String,
	private_key: String,
) -> Result<(), JsValue>
{
	Ok(util_req_full::user::done_key_rotation(
		base_url,
		auth_token.as_str(),
		jwt.as_str(),
		server_output.as_str(),
		pre_group_key.as_str(),
		public_key.as_str(),
		private_key.as_str(),
	)
	.await?)
}

#[wasm_bindgen]
pub fn user_create_safety_number(
	verify_key_1: &str,
	user_id_1: &str,
	verify_key_2: Option<String>,
	user_id_2: Option<String>,
) -> Result<String, JsValue>
{
	Ok(user::create_safety_number(
		verify_key_1,
		user_id_1,
		verify_key_2.as_deref(),
		user_id_2.as_deref(),
	)?)
}

#[wasm_bindgen]
pub fn user_verify_user_public_key(verify_key: &str, public_key: &str) -> Result<bool, JsValue>
{
	Ok(user::verify_user_public_key(verify_key, public_key)?)
}
