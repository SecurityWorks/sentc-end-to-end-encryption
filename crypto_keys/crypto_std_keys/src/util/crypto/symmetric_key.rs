use alloc::string::{String, ToString};
use alloc::vec::Vec;

use base64ct::{Base64, Encoding};
use sentc_crypto_common::crypto::EncryptedHead;
use sentc_crypto_common::user::UserVerifyKeyData;
use sentc_crypto_core::cryptomat::SymKey;
use sentc_crypto_utils::cryptomat::{SignKWrapper, SymKeyCrypto};
use sentc_crypto_utils::error::SdkUtilError;

use crate::util::{SignKey, SymmetricKey, VerifyKey};

impl SymKeyCrypto for SymmetricKey
{
	type SignKey = SignKey;
	type VerifyKey = VerifyKey;

	fn encrypt_raw(&self, data: &[u8], sign_key: Option<&Self::SignKey>) -> Result<(EncryptedHead, Vec<u8>), SdkUtilError>
	{
		let encrypted = self.key.encrypt(data)?;

		self.finish_raw_encrypt(encrypted, sign_key)
	}

	fn encrypt_raw_with_aad(&self, data: &[u8], aad: &[u8], sign_key: Option<&Self::SignKey>) -> Result<(EncryptedHead, Vec<u8>), SdkUtilError>
	{
		let encrypted = self.key.encrypt_with_aad(data, aad)?;

		self.finish_raw_encrypt(encrypted, sign_key)
	}

	fn decrypt_raw(&self, encrypted_data: &[u8], head: &EncryptedHead, verify_key: Option<&UserVerifyKeyData>) -> Result<Vec<u8>, SdkUtilError>
	{
		let data_to_decrypt = Self::prepare_decrypt(encrypted_data, head, verify_key)?;

		Ok(self.key.decrypt(data_to_decrypt)?)
	}

	fn decrypt_raw_with_aad(
		&self,
		encrypted_data: &[u8],
		aad: &[u8],
		head: &EncryptedHead,
		verify_key: Option<&UserVerifyKeyData>,
	) -> Result<Vec<u8>, SdkUtilError>
	{
		let data_to_decrypt = Self::prepare_decrypt(encrypted_data, head, verify_key)?;

		Ok(self.key.decrypt_with_aad(data_to_decrypt, aad)?)
	}

	fn encrypt_string(&self, data: &str, sign_key: Option<&Self::SignKey>) -> Result<String, SdkUtilError>
	{
		let encrypted = self.encrypt(data.as_bytes(), sign_key)?;

		Ok(Base64::encode_string(&encrypted))
	}

	fn encrypt_string_with_aad(&self, data: &str, aad: &str, sign_key: Option<&Self::SignKey>) -> Result<String, SdkUtilError>
	{
		let encrypted = self.encrypt_with_aad(data.as_bytes(), aad.as_bytes(), sign_key)?;

		Ok(Base64::encode_string(&encrypted))
	}

	fn decrypt_string(&self, encrypted_data_with_head: &str, verify_key: Option<&UserVerifyKeyData>) -> Result<String, SdkUtilError>
	{
		let encrypted = Base64::decode_vec(encrypted_data_with_head).map_err(|_| SdkUtilError::DecodeEncryptedDataFailed)?;

		let decrypted = self.decrypt(&encrypted, verify_key)?;

		String::from_utf8(decrypted).map_err(|_| SdkUtilError::DecodeEncryptedDataFailed)
	}

	fn decrypt_string_with_aad(
		&self,
		encrypted_data_with_head: &str,
		aad: &str,
		verify_key: Option<&UserVerifyKeyData>,
	) -> Result<String, SdkUtilError>
	{
		let encrypted = Base64::decode_vec(encrypted_data_with_head).map_err(|_| SdkUtilError::DecodeEncryptedDataFailed)?;

		let decrypted = self.decrypt_with_aad(&encrypted, aad.as_bytes(), verify_key)?;

		String::from_utf8(decrypted).map_err(|_| SdkUtilError::DecodeEncryptedDataFailed)
	}
}

impl SymmetricKey
{
	fn finish_raw_encrypt(&self, encrypted: Vec<u8>, sign_key: Option<&impl SignKWrapper>) -> Result<(EncryptedHead, Vec<u8>), SdkUtilError>
	{
		match sign_key {
			None => {
				Ok((
					EncryptedHead {
						id: self.key_id.to_string(),
						sign: None,
					},
					encrypted,
				))
			},
			Some(sk) => {
				let (sign_head, data_with_sign) = sk.sign_with_head(&encrypted)?;

				Ok((
					EncryptedHead {
						id: self.key_id.to_string(),
						sign: Some(sign_head),
					},
					data_with_sign,
				))
			},
		}
	}
}
