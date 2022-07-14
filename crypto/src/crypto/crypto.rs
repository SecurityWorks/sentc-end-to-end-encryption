use alloc::string::String;
use alloc::vec::Vec;

use sentc_crypto_common::crypto::EncryptedHead;
use sentc_crypto_common::user::{UserPublicKeyData, UserVerifyKeyData};
use sentc_crypto_core::Error;

use crate::crypto::{
	decrypt_asymmetric_internally,
	decrypt_raw_asymmetric_internally,
	decrypt_raw_symmetric_internally,
	decrypt_string_asymmetric_internally,
	decrypt_string_symmetric_internally,
	decrypt_symmetric_internally,
	encrypt_asymmetric_internally,
	encrypt_raw_asymmetric_internally,
	encrypt_raw_symmetric_internally,
	encrypt_string_asymmetric_internally,
	encrypt_string_symmetric_internally,
	encrypt_symmetric_internally,
};
use crate::err_to_msg;
use crate::util::{import_private_key, import_sign_key, import_sym_key, SignKeyFormatInt};

fn prepare_sign_key(sign_key: &str) -> Result<Option<SignKeyFormatInt>, Error>
{
	let sign_key = match sign_key {
		"" => None,
		_ => {
			let k = import_sign_key(sign_key)?;

			Some(k)
		},
	};

	Ok(sign_key)
}

fn prepare_verify_key(verify_key_data: &str) -> Result<Option<UserVerifyKeyData>, Error>
{
	let verify_key = match verify_key_data {
		"" => None,
		_ => {
			let k = UserVerifyKeyData::from_string(verify_key_data).map_err(|_| Error::JsonParseFailed)?;

			Some(k)
		},
	};

	Ok(verify_key)
}

pub fn encrypt_raw_symmetric(key: &str, data: &[u8], sign_key: &str) -> Result<(String, Vec<u8>), String>
{
	let key = import_sym_key(key).map_err(|e| err_to_msg(e))?;

	let sign_key = prepare_sign_key(sign_key).map_err(|e| err_to_msg(e))?;

	let (head, encrypted) = match sign_key {
		//in match because we need a valid ref to the sign key format
		None => encrypt_raw_symmetric_internally(&key, data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => encrypt_raw_symmetric_internally(&key, data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	let head = head
		.to_string()
		.map_err(|_e| err_to_msg(Error::JsonToStringFailed))?;

	Ok((head, encrypted))
}

pub fn decrypt_raw_symmetric(key: &str, encrypted_data: &[u8], head: &str, verify_key_data: &str) -> Result<Vec<u8>, String>
{
	let key = import_sym_key(key).map_err(|e| err_to_msg(e))?;

	let verify_key = prepare_verify_key(verify_key_data).map_err(|e| err_to_msg(e))?;

	let head = EncryptedHead::from_string(head).map_err(|_e| err_to_msg(Error::JsonParseFailed))?;

	let decrypted = match verify_key {
		None => decrypt_raw_symmetric_internally(&key, encrypted_data, &head, None).map_err(|e| err_to_msg(e))?,
		Some(k) => decrypt_raw_symmetric_internally(&key, encrypted_data, &head, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(decrypted)
}

pub fn encrypt_raw_asymmetric(reply_public_key_data: &str, data: &[u8], sign_key: &str) -> Result<(String, Vec<u8>), String>
{
	let reply_public_key_data = UserPublicKeyData::from_string(reply_public_key_data).map_err(|_| err_to_msg(Error::JsonParseFailed))?;

	let sign_key = prepare_sign_key(sign_key).map_err(|e| err_to_msg(e))?;

	let (head, encrypted) = match sign_key {
		//in match because we need a valid ref to the sign key format
		None => encrypt_raw_asymmetric_internally(&reply_public_key_data, data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => encrypt_raw_asymmetric_internally(&reply_public_key_data, data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	let head = head
		.to_string()
		.map_err(|_e| err_to_msg(Error::JsonToStringFailed))?;

	Ok((head, encrypted))
}

pub fn decrypt_raw_asymmetric(private_key: &str, encrypted_data: &[u8], head: &str, verify_key_data: &str) -> Result<Vec<u8>, String>
{
	let private_key = import_private_key(private_key).map_err(|e| err_to_msg(e))?;

	let verify_key = prepare_verify_key(verify_key_data).map_err(|e| err_to_msg(e))?;

	let head = EncryptedHead::from_string(head).map_err(|_| err_to_msg(Error::JsonParseFailed))?;

	let decrypted = match verify_key {
		None => decrypt_raw_asymmetric_internally(&private_key, encrypted_data, &head, None).map_err(|e| err_to_msg(e))?,
		Some(k) => decrypt_raw_asymmetric_internally(&private_key, encrypted_data, &head, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(decrypted)
}

pub fn encrypt_symmetric(key: &str, data: &[u8], sign_key: &str) -> Result<Vec<u8>, String>
{
	let key = import_sym_key(key).map_err(|e| err_to_msg(e))?;

	let sign_key = prepare_sign_key(sign_key).map_err(|e| err_to_msg(e))?;

	let encrypted = match sign_key {
		//in match because we need a valid ref to the sign key format
		None => encrypt_symmetric_internally(&key, data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => encrypt_symmetric_internally(&key, data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(encrypted)
}

pub fn decrypt_symmetric(key: &str, encrypted_data: &[u8], verify_key_data: &str) -> Result<Vec<u8>, String>
{
	let key = import_sym_key(key).map_err(|e| err_to_msg(e))?;

	let verify_key = prepare_verify_key(verify_key_data).map_err(|e| err_to_msg(e))?;

	let decrypted = match verify_key {
		None => decrypt_symmetric_internally(&key, encrypted_data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => decrypt_symmetric_internally(&key, encrypted_data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(decrypted)
}

pub fn encrypt_asymmetric(reply_public_key_data: &str, data: &[u8], sign_key: &str) -> Result<Vec<u8>, String>
{
	let reply_public_key_data = UserPublicKeyData::from_string(reply_public_key_data).map_err(|_| err_to_msg(Error::JsonParseFailed))?;

	let sign_key = prepare_sign_key(sign_key).map_err(|e| err_to_msg(e))?;

	let encrypted = match sign_key {
		//in match because we need a valid ref to the sign key format
		None => encrypt_asymmetric_internally(&reply_public_key_data, data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => encrypt_asymmetric_internally(&reply_public_key_data, data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(encrypted)
}

pub fn decrypt_asymmetric(private_key: &str, encrypted_data: &[u8], verify_key_data: &str) -> Result<Vec<u8>, String>
{
	let private_key = import_private_key(private_key).map_err(|e| err_to_msg(e))?;

	let verify_key = prepare_verify_key(verify_key_data).map_err(|e| err_to_msg(e))?;

	let decrypted = match verify_key {
		None => decrypt_asymmetric_internally(&private_key, encrypted_data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => decrypt_asymmetric_internally(&private_key, encrypted_data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(decrypted)
}

pub fn encrypt_string_symmetric(key: &str, data: &[u8], sign_key: &str) -> Result<String, String>
{
	let key = import_sym_key(key).map_err(|e| err_to_msg(e))?;

	let sign_key = prepare_sign_key(sign_key).map_err(|e| err_to_msg(e))?;

	let encrypted = match sign_key {
		//in match because we need a valid ref to the sign key format
		None => encrypt_string_symmetric_internally(&key, data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => encrypt_string_symmetric_internally(&key, data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(encrypted)
}

pub fn decrypt_string_symmetric(key: &str, encrypted_data: &str, verify_key_data: &str) -> Result<Vec<u8>, String>
{
	let key = import_sym_key(key).map_err(|e| err_to_msg(e))?;

	let verify_key = prepare_verify_key(verify_key_data).map_err(|e| err_to_msg(e))?;

	let decrypted = match verify_key {
		None => decrypt_string_symmetric_internally(&key, encrypted_data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => decrypt_string_symmetric_internally(&key, encrypted_data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(decrypted)
}

pub fn encrypt_string_asymmetric(reply_public_key_data: &str, data: &[u8], sign_key: &str) -> Result<String, String>
{
	let reply_public_key_data = UserPublicKeyData::from_string(reply_public_key_data).map_err(|_| err_to_msg(Error::JsonParseFailed))?;

	let sign_key = prepare_sign_key(sign_key).map_err(|e| err_to_msg(e))?;

	let encrypted = match sign_key {
		//in match because we need a valid ref to the sign key format
		None => encrypt_string_asymmetric_internally(&reply_public_key_data, data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => encrypt_string_asymmetric_internally(&reply_public_key_data, data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(encrypted)
}

pub fn decrypt_string_asymmetric(private_key: &str, encrypted_data: &str, verify_key_data: &str) -> Result<Vec<u8>, String>
{
	let private_key = import_private_key(private_key).map_err(|e| err_to_msg(e))?;

	let verify_key = prepare_verify_key(verify_key_data).map_err(|e| err_to_msg(e))?;

	let decrypted = match verify_key {
		None => decrypt_string_asymmetric_internally(&private_key, encrypted_data, None).map_err(|e| err_to_msg(e))?,
		Some(k) => decrypt_string_asymmetric_internally(&private_key, encrypted_data, Some(&k)).map_err(|e| err_to_msg(e))?,
	};

	Ok(decrypted)
}

#[cfg(test)]
mod test
{
	use super::*;
	use crate::group::test_fn::create_group;
	use crate::user::test_fn::create_user;

	#[test]
	fn test_encrypt_decrypt_sym_raw()
	{
		let user = create_user();
		let (group, _) = create_group(&user);
		let group_key = &group.keys[0].group_key;

		let text = "123*+^êéèüöß@€&$";

		let (head, encrypted) = encrypt_raw_symmetric(group_key, text.as_bytes(), "").unwrap();

		let decrypted = decrypt_raw_symmetric(group_key, &encrypted, &head, "").unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_sym_raw_with_sig()
	{
		let user = create_user();

		let (group, _) = create_group(&user);
		let group_key = &group.keys[0].group_key;

		let text = "123*+^êéèüöß@€&$";

		let (head, encrypted) = encrypt_raw_symmetric(group_key, text.as_bytes(), user.sign_key.as_str()).unwrap();

		let decrypted = decrypt_raw_symmetric(group_key, &encrypted, &head, user.exported_verify_key.as_str()).unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_asym_raw()
	{
		let text = "123*+^êéèüöß@€&$";
		let user = create_user();

		let (head, encrypted) = encrypt_raw_asymmetric(user.exported_public_key.as_str(), text.as_bytes(), "").unwrap();

		let decrypted = decrypt_raw_asymmetric(user.private_key.as_str(), &encrypted, &head, "").unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_asym_raw_with_sig()
	{
		let text = "123*+^êéèüöß@€&$";
		let user = create_user();

		let (head, encrypted) = encrypt_raw_asymmetric(user.exported_public_key.as_str(), text.as_bytes(), user.sign_key.as_str()).unwrap();

		let decrypted = decrypt_raw_asymmetric(user.private_key.as_str(), &encrypted, &head, user.exported_verify_key.as_str()).unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_sym()
	{
		let user = create_user();
		let (group, _) = create_group(&user);
		let group_key = &group.keys[0].group_key;

		let text = "123*+^êéèüöß@€&$";

		let encrypted = encrypt_symmetric(group_key, text.as_bytes(), "").unwrap();

		let decrypted = decrypt_symmetric(group_key, &encrypted, "").unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_sym_with_sig()
	{
		let user = create_user();

		let (group, _) = create_group(&user);
		let group_key = &group.keys[0].group_key;

		let text = "123*+^êéèüöß@€&$";

		let encrypted = encrypt_symmetric(group_key, text.as_bytes(), user.sign_key.as_str()).unwrap();

		let decrypted = decrypt_symmetric(group_key, &encrypted, user.exported_verify_key.as_str()).unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_asym()
	{
		let text = "123*+^êéèüöß@€&$";
		let user = create_user();

		let encrypted = encrypt_asymmetric(user.exported_public_key.as_str(), text.as_bytes(), "").unwrap();

		let decrypted = decrypt_asymmetric(user.private_key.as_str(), &encrypted, "").unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_asym_with_sig()
	{
		let text = "123*+^êéèüöß@€&$";
		let user = create_user();

		let encrypted = encrypt_asymmetric(user.exported_public_key.as_str(), text.as_bytes(), user.sign_key.as_str()).unwrap();

		let decrypted = decrypt_asymmetric(user.private_key.as_str(), &encrypted, user.exported_verify_key.as_str()).unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_string_sym()
	{
		let user = create_user();
		let (group, _) = create_group(&user);
		let group_key = &group.keys[0].group_key;

		let text = "123*+^êéèüöß@€&$";

		let encrypted = encrypt_string_symmetric(group_key, text.as_bytes(), "").unwrap();

		let decrypted = decrypt_string_symmetric(group_key, &encrypted, "").unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_string_sym_with_sig()
	{
		let user = create_user();

		let (group, _) = create_group(&user);
		let group_key = &group.keys[0].group_key;

		let text = "123*+^êéèüöß@€&$";

		let encrypted = encrypt_string_symmetric(group_key, text.as_bytes(), user.sign_key.as_str()).unwrap();

		let decrypted = decrypt_string_symmetric(group_key, &encrypted, user.exported_verify_key.as_str()).unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_string_asym()
	{
		let text = "123*+^êéèüöß@€&$";
		let user = create_user();

		let encrypted = encrypt_string_asymmetric(user.exported_public_key.as_str(), text.as_bytes(), "").unwrap();

		let decrypted = decrypt_string_asymmetric(user.private_key.as_str(), &encrypted, "").unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}

	#[test]
	fn test_encrypt_decrypt_string_asym_with_sig()
	{
		let text = "123*+^êéèüöß@€&$";
		let user = create_user();

		let encrypted = encrypt_string_asymmetric(user.exported_public_key.as_str(), text.as_bytes(), user.sign_key.as_str()).unwrap();

		let decrypted = decrypt_string_asymmetric(user.private_key.as_str(), &encrypted, user.exported_verify_key.as_str()).unwrap();

		assert_eq!(text.as_bytes(), decrypted);
	}
}
