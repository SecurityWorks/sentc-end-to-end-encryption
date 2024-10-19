//! # Handle user
//!
//! these functions are used for decoding and encoding the internally values for and from the other implementations
//! we can't work with the enums from the core user mod directly because they must be encoded to base64
//!
//! If rust feature is enabled the rust functions are used. The return is no longer just a json string but rust structs and enums to work with

use alloc::string::String;

use sentc_crypto_common::user::DoneLoginServerReturn;

use crate::SdkError;

pub(crate) mod user;
#[cfg(feature = "export")]
mod user_export;

pub use self::user::User;
#[cfg(not(feature = "export"))]
pub use self::user::*;
#[cfg(feature = "export")]
pub use self::user_export::*;

pub fn check_done_login(server_output: &str) -> Result<DoneLoginServerReturn, SdkError>
{
	Ok(sentc_crypto_utils::user::check_done_login(server_output)?)
}

pub fn prepare_validate_mfa(auth_key: String, device_identifier: String, token: String) -> Result<String, SdkError>
{
	Ok(sentc_crypto_utils::user::prepare_validate_mfa(
		auth_key,
		device_identifier,
		token,
	)?)
}

#[cfg(test)]
pub(crate) mod test_fn
{
	use alloc::string::ToString;
	use alloc::vec;

	use sentc_crypto_common::group::{GroupHmacData, GroupKeyServerOutput};
	use sentc_crypto_common::user::{
		DoneLoginServerKeysOutput,
		DoneLoginServerOutput,
		KeyDerivedData,
		PrepareLoginSaltServerOutput,
		RegisterData,
		VerifyLoginInput,
		VerifyLoginOutput,
	};
	use sentc_crypto_common::ServerOutput;

	use super::*;
	#[cfg(feature = "export")]
	use crate::entities::user::UserDataExport;
	use crate::util;
	use crate::util::server::generate_salt_from_base64_to_string;

	#[cfg(feature = "std_keys")]
	pub type TestUser = crate::keys::std::StdUser;
	#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
	pub type TestUser = crate::keys::fips::FipsUser;
	#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
	pub type TestUser = crate::keys::rec::RecUser;

	#[cfg(feature = "std_keys")]
	pub type TestUserDataInt = crate::keys::std::StdUserDataInt;
	#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
	pub type TestUserDataInt = crate::keys::fips::FipsUserDataInt;
	#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
	pub type TestUserDataInt = crate::keys::rec::RecUserDataInt;

	#[cfg(feature = "std_keys")]
	pub type TestUserKeyDataInt = crate::keys::std::StdUserKeyDataInt;
	#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
	pub type TestUserKeyDataInt = crate::keys::fips::FipsUserKeyDataInt;
	#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
	pub type TestUserKeyDataInt = crate::keys::rec::RecUserKeyDataInt;

	pub(crate) fn simulate_server_prepare_login(derived: &KeyDerivedData) -> String
	{
		//and now try to log in
		//normally the salt gets calc by the api
		#[cfg(feature = "std_keys")]
		let salt_string = generate_salt_from_base64_to_string::<sentc_crypto_std_keys::core::ClientRandomValue>(
			&derived.client_random_value,
			&derived.derived_alg,
			"",
		)
		.unwrap();

		#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
		let salt_string = generate_salt_from_base64_to_string::<sentc_crypto_fips_keys::core::pw_hash::ClientRandomValue>(
			&derived.client_random_value,
			&derived.derived_alg,
			"",
		)
		.unwrap();

		#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
		let salt_string = generate_salt_from_base64_to_string::<sentc_crypto_rec_keys::core::pw_hash::ClientRandomValue>(
			&derived.client_random_value,
			&derived.derived_alg,
			"",
		)
		.unwrap();

		ServerOutput {
			status: true,
			err_msg: None,
			err_code: None,
			result: Some(PrepareLoginSaltServerOutput {
				salt_string,
				derived_encryption_key_alg: derived.derived_alg.clone(),
			}),
		}
		.to_string()
		.unwrap()
	}

	pub(crate) fn simulate_server_done_login(data: RegisterData) -> DoneLoginServerOutput
	{
		let RegisterData {
			device, ..
		} = data;

		#[cfg(feature = "std_keys")]
		let challenge = util::server::encrypt_login_verify_challenge::<sentc_crypto_std_keys::util::SecretKey>(
			&device.derived.public_key,
			&device.derived.keypair_encrypt_alg,
			"abcd",
		)
		.unwrap();

		#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
		let challenge = util::server::encrypt_login_verify_challenge::<sentc_crypto_fips_keys::util::SecretKey>(
			&device.derived.public_key,
			&device.derived.keypair_encrypt_alg,
			"abcd",
		)
		.unwrap();

		#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
		let challenge = util::server::encrypt_login_verify_challenge::<sentc_crypto_rec_keys::util::SecretKey>(
			&device.derived.public_key,
			&device.derived.keypair_encrypt_alg,
			"abcd",
		)
		.unwrap();

		//get the server output back
		let device_keys = DoneLoginServerKeysOutput {
			encrypted_master_key: device.master_key.encrypted_master_key,
			encrypted_private_key: device.derived.encrypted_private_key,
			encrypted_sign_key: device.derived.encrypted_sign_key,
			public_key_string: device.derived.public_key,
			verify_key_string: device.derived.verify_key,
			keypair_encrypt_alg: device.derived.keypair_encrypt_alg,
			keypair_sign_alg: device.derived.keypair_sign_alg,
			keypair_encrypt_id: "abc".to_string(),
			keypair_sign_id: "dfg".to_string(),
			user_id: "abc".to_string(),
			device_id: "abc".to_string(),
			user_group_id: "abc".to_string(),
		};

		DoneLoginServerOutput {
			device_keys,
			challenge,
		}
	}

	pub(crate) fn simulate_verify_login(data: RegisterData, challenge: &str) -> String
	{
		let challenge: VerifyLoginInput = serde_json::from_str(challenge).unwrap();
		assert_eq!(challenge.challenge, "abcd");

		let RegisterData {
			group, ..
		} = data;

		let user_keys = vec![GroupKeyServerOutput {
			encrypted_group_key: group.encrypted_group_key,
			group_key_alg: group.group_key_alg,
			group_key_id: "abc".to_string(),
			encrypted_private_group_key: group.encrypted_private_group_key,
			public_group_key: group.public_group_key,
			keypair_encrypt_alg: group.keypair_encrypt_alg,
			key_pair_id: "".to_string(),
			user_public_key_id: "abc".to_string(),
			time: 0,
			signed_by_user_id: None,
			signed_by_user_sign_key_id: None,
			group_key_sig: None,
			encrypted_sign_key: group.encrypted_sign_key,
			verify_key: group.verify_key,
			keypair_sign_alg: group.keypair_sign_alg,
			keypair_sign_id: Some("abc".to_string()),
			public_key_sig: group.public_key_sig,
			public_key_sig_key_id: Some("abc".to_string()),
		}];

		let hmac_keys = vec![GroupHmacData {
			id: "123".to_string(),
			encrypted_hmac_encryption_key_id: "".to_string(),
			encrypted_hmac_key: group.encrypted_hmac_key,
			encrypted_hmac_alg: group.encrypted_hmac_alg,
			time: 0,
		}];

		let out = VerifyLoginOutput {
			user_keys,
			hmac_keys,
			jwt: "abc".to_string(),
			refresh_token: "abc".to_string(),
		};

		ServerOutput {
			status: true,
			err_msg: None,
			err_code: None,
			result: Some(out),
		}
		.to_string()
		.unwrap()
	}

	pub(crate) fn create_user() -> TestUserDataInt
	{
		let username = "admin";
		let password = "12345";

		let out_string = TestUser::register(username, password).unwrap();

		let out = RegisterData::from_string(out_string.as_str()).unwrap();
		let server_output = simulate_server_prepare_login(&out.device.derived);

		let (_input, auth_key, master_key_encryption_key) = TestUser::prepare_login(username, password, &server_output).unwrap();

		let server_output = simulate_server_done_login(out);

		let done_login = TestUser::done_login(
			&master_key_encryption_key,
			auth_key,
			username.to_string(),
			server_output,
		)
		.unwrap();

		let server_output = simulate_verify_login(RegisterData::from_string(&out_string).unwrap(), &done_login.challenge);

		TestUser::verify_login(
			&server_output,
			done_login.user_id,
			done_login.device_id,
			done_login.device_keys,
		)
		.unwrap()
	}

	#[cfg(feature = "export")]
	pub(crate) fn create_user_export() -> UserDataExport
	{
		let username = "admin";
		let password = "12345";

		let out_string = register(username, password).unwrap();

		let out = RegisterData::from_string(out_string.as_str()).unwrap();
		let server_output = simulate_server_prepare_login(&out.device.derived);

		let (_input, auth_key, master_key_encryption_key) = prepare_login(username, password, &server_output).unwrap();

		let server_output = simulate_server_done_login(out);

		let done_login = done_login(
			&master_key_encryption_key,
			auth_key,
			username.to_string(),
			server_output,
		)
		.unwrap();

		let server_output = simulate_verify_login(RegisterData::from_string(&out_string).unwrap(), &done_login.challenge);

		verify_login(
			&server_output,
			done_login.user_id,
			done_login.device_id,
			done_login.device_keys,
		)
		.unwrap()
	}
}
