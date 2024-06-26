use alloc::string::String;

use sentc_crypto_std_keys::core::PwHasherGetter;
use sentc_crypto_std_keys::util::{SecretKey, SignKey};
use sentc_crypto_utils::handle_general_server_response;
use sentc_crypto_utils::http::{auth_req, non_auth_req, HttpMethod};

use crate::StdUserPreVerifyLogin;

#[cfg(feature = "export")]
mod non_rust;
#[cfg(not(feature = "export"))]
mod rust;

#[cfg(feature = "export")]
pub(crate) use self::non_rust::{
	BoolRes,
	DeviceListRes,
	InitRes,
	LoginRes,
	OtpRecoveryKeyRes,
	PreLoginRes,
	RegisterOtpRes,
	RegisterRawOtpRes,
	Res,
	VoidRes,
};
#[cfg(feature = "export")]
pub use self::non_rust::{PreLoginOut, PrepareLoginOtpOutput};
#[cfg(not(feature = "export"))]
pub(crate) use self::rust::{
	BoolRes,
	DeviceListRes,
	InitRes,
	LoginRes,
	OtpRecoveryKeyRes,
	PreLoginRes,
	RegisterOtpRes,
	RegisterRawOtpRes,
	Res,
	VoidRes,
};
#[cfg(not(feature = "export"))]
pub use self::rust::{PreLoginOut, PrepareLoginOtpOutput};

//Register
pub async fn check_user_identifier_available(base_url: String, auth_token: &str, user_identifier: &str) -> BoolRes
{
	let server_input = crate::user::prepare_check_user_identifier_available(user_identifier)?;

	let url = base_url + "/api/v1/exists";

	let res = non_auth_req(HttpMethod::POST, &url, auth_token, Some(server_input)).await?;
	let out = crate::user::done_check_user_identifier_available(&res)?;

	Ok(out)
}

pub async fn register(base_url: String, auth_token: &str, user_identifier: &str, password: &str) -> Res
{
	let register_input = crate::user::register(user_identifier, password)?;

	let url = base_url + "/api/v1/register_light";

	let res = non_auth_req(HttpMethod::POST, &url, auth_token, Some(register_input)).await?;

	let out = crate::user::done_register(&res)?;

	Ok(out)
}

pub async fn register_device_start(base_url: String, auth_token: &str, device_identifier: &str, password: &str) -> Res
{
	let url = base_url + "/api/v1/user/prepare_register_device";

	let input = crate::user::register(device_identifier, password)?;

	let res = non_auth_req(HttpMethod::POST, &url, auth_token, Some(input)).await?;

	//check the server output
	crate::user::done_register_device_start(&res)?;

	Ok(res)
}

pub async fn register_device(base_url: String, auth_token: &str, jwt: &str, server_output: &str) -> VoidRes
{
	let url = base_url + "/api/v1/user/done_register_device_light";

	let input = crate::user::prepare_register_device(server_output)?;

	let res = auth_req(HttpMethod::PUT, &url, auth_token, Some(input), jwt).await?;

	handle_general_server_response(&res)?;

	Ok(())
}

//__________________________________________________________________________________________________
//Login

async fn verify_login(base_url: String, auth_token: &str, pre_verify: StdUserPreVerifyLogin) -> LoginRes
{
	let url = base_url + "/api/v1/verify_login_light";
	let server_out = non_auth_req(HttpMethod::POST, url.as_str(), auth_token, Some(pre_verify.challenge)).await?;

	let keys = crate::user::verify_login(
		&server_out,
		pre_verify.user_id,
		pre_verify.device_id,
		pre_verify.device_keys,
	)?;

	Ok(keys)
}

pub async fn login(base_url: String, auth_token: &str, user_identifier: &str, password: &str) -> PreLoginRes
{
	let pre_login =
		sentc_crypto_utils::full::user::login::<SecretKey, SignKey, PwHasherGetter>(base_url.clone(), auth_token, user_identifier, password).await?;

	match pre_login {
		sentc_crypto_utils::full::user::PreLoginOut::Direct(d) => {
			let out = verify_login(base_url, auth_token, d).await?;

			Ok(PreLoginOut::Direct(out))
		},
		sentc_crypto_utils::full::user::PreLoginOut::Otp(d) => {
			//Otp means the user enables otp, so use done_otp_login fn with the user token before verify,
			// DoneLoginServerOutput is not returned at this point

			//export the data needed for this fn

			#[cfg(feature = "export")]
			{
				let master_key: sentc_crypto_std_keys::util::MasterKeyFormat = d.master_key.into();

				Ok(PreLoginOut::Otp(PrepareLoginOtpOutput {
					master_key: master_key.to_string()?,
					auth_key: d.auth_key,
				}))
			}

			#[cfg(not(feature = "export"))]
			{
				Ok(PreLoginOut::Otp(PrepareLoginOtpOutput {
					master_key: d.master_key,
					auth_key: d.auth_key,
				}))
			}
		},
	}
}

pub async fn mfa_login(
	base_url: String,
	auth_token: &str,
	#[cfg(feature = "export")] master_key_encryption: &str,
	#[cfg(not(feature = "export"))] master_key_encryption: &sentc_crypto_std_keys::core::DeriveMasterKeyForAuth,
	auth_key: String,
	user_identifier: String,
	token: String,
	recovery: bool,
) -> LoginRes
{
	#[cfg(feature = "export")]
	let master_key_encryption: &sentc_crypto_std_keys::core::DeriveMasterKeyForAuth = {
		let master_key_encryption: sentc_crypto_std_keys::util::MasterKeyFormat = master_key_encryption.parse()?;

		&master_key_encryption.try_into()?
	};

	let keys = sentc_crypto_utils::full::user::mfa_login::<SecretKey, SignKey>(
		base_url.clone(),
		auth_token,
		master_key_encryption,
		auth_key,
		user_identifier,
		token,
		recovery,
	)
	.await?;

	verify_login(base_url, auth_token, keys).await
}

pub async fn refresh_jwt(base_url: String, auth_token: &str, jwt: &str, refresh_token: String) -> Res
{
	Ok(sentc_crypto_utils::full::user::refresh_jwt(base_url, auth_token, jwt, refresh_token).await?)
}

pub async fn init_user(base_url: String, auth_token: &str, jwt: &str, refresh_token: String) -> InitRes
{
	Ok(sentc_crypto_utils::full::user::init_user(base_url, auth_token, jwt, refresh_token).await?)
}

pub async fn get_user_devices(base_url: String, auth_token: &str, jwt: &str, last_fetched_time: &str, last_fetched_id: &str) -> DeviceListRes
{
	Ok(sentc_crypto_utils::full::user::get_user_devices(base_url, auth_token, jwt, last_fetched_time, last_fetched_id).await?)
}

pub async fn get_fresh_jwt(
	base_url: String,
	auth_token: &str,
	user_identifier: &str,
	password: &str,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> Res
{
	let (_, keys, _) = sentc_crypto_utils::full::user::prepare_user_fresh_jwt::<SecretKey, SignKey, PwHasherGetter>(
		base_url.clone(),
		auth_token,
		user_identifier,
		password,
		mfa_token,
		mfa_recovery,
	)
	.await?;

	let keys = verify_login(base_url, auth_token, keys).await?;

	Ok(keys.jwt)
}

//__________________________________________________________________________________________________

pub async fn change_password(
	base_url: String,
	auth_token: &str,
	user_identifier: &str,
	old_password: &str,
	new_password: &str,
	mfa_token: Option<String>,
	mfa_recovery: Option<bool>,
) -> VoidRes
{
	let (prep_login_out, keys, done_login_out) = sentc_crypto_utils::full::user::prepare_user_fresh_jwt::<SecretKey, SignKey, PwHasherGetter>(
		base_url.clone(),
		auth_token,
		user_identifier,
		old_password,
		mfa_token,
		mfa_recovery,
	)
	.await?;

	let keys = verify_login(base_url.clone(), auth_token, keys).await?;

	Ok(
		sentc_crypto_utils::full::user::done_change_password::<PwHasherGetter>(
			base_url,
			auth_token,
			old_password,
			new_password,
			&keys.jwt,
			&prep_login_out,
			done_login_out,
		)
		.await?,
	)
}

/**
Resets the password of a device of a user.

This req can only be done with the secret token from your backend, not your frontend!
*/
pub async fn reset_password(base_url: String, auth_token: &str, user_identifier: &str, new_password: &str) -> VoidRes
{
	let url = base_url + "/api/v1/user/reset_pw_light";

	let input = crate::user::register(user_identifier, new_password)?;

	let res = non_auth_req(HttpMethod::PUT, url.as_str(), auth_token, Some(input)).await?;

	Ok(handle_general_server_response(res.as_str())?)
}

pub async fn delete(base_url: String, auth_token: &str, fresh_jwt: &str) -> VoidRes
{
	Ok(sentc_crypto_utils::full::user::done_delete(base_url, auth_token, fresh_jwt).await?)
}

/**
# Remove a device from the user group.

This can only be done when the actual device got a fresh jwt,
to make sure that no hacker can remove devices.
 */
pub async fn delete_device(base_url: String, auth_token: &str, fresh_jwt: &str, device_id: &str) -> VoidRes
{
	Ok(sentc_crypto_utils::full::user::done_delete_device(base_url, auth_token, fresh_jwt, device_id).await?)
}

//__________________________________________________________________________________________________

pub async fn update(base_url: String, auth_token: &str, jwt: &str, user_identifier: String) -> VoidRes
{
	Ok(sentc_crypto_utils::full::user::update(base_url, auth_token, jwt, user_identifier).await?)
}

//__________________________________________________________________________________________________
//Otp

pub async fn register_raw_otp(base_url: String, auth_token: &str, fresh_jwt: &str) -> RegisterRawOtpRes
{
	Ok(sentc_crypto_utils::full::user::register_raw_otp(base_url, auth_token, fresh_jwt).await?)
}

pub async fn register_otp(base_url: String, auth_token: &str, issuer: &str, audience: &str, fresh_jwt: &str) -> RegisterOtpRes
{
	Ok(sentc_crypto_utils::full::user::register_otp(base_url, auth_token, fresh_jwt, issuer, audience).await?)
}

pub async fn get_otp_recover_keys(base_url: String, auth_token: &str, fresh_jwt: &str) -> OtpRecoveryKeyRes
{
	Ok(sentc_crypto_utils::full::user::get_otp_recover_keys(base_url, auth_token, fresh_jwt).await?)
}

pub async fn reset_raw_otp(base_url: String, auth_token: &str, fresh_jwt: &str) -> RegisterRawOtpRes
{
	Ok(sentc_crypto_utils::full::user::reset_raw_otp(base_url, auth_token, fresh_jwt).await?)
}

pub async fn reset_otp(base_url: String, auth_token: &str, issuer: &str, audience: &str, fresh_jwt: &str) -> RegisterOtpRes
{
	Ok(sentc_crypto_utils::full::user::reset_otp(base_url, auth_token, fresh_jwt, issuer, audience).await?)
}

pub async fn disable_otp(base_url: String, auth_token: &str, fresh_jwt: &str) -> VoidRes
{
	Ok(sentc_crypto_utils::full::user::disable_otp(base_url, auth_token, fresh_jwt).await?)
}
