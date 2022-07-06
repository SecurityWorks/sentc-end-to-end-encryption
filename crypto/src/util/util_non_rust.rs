use alloc::string::String;

use base64ct::{Base64, Encoding};
use sendclose_crypto_core::{Error, Pk, SignK, Sk, SymKey, VerifyK};
use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_string};

use crate::util::{PrivateKeyFormatInt, PublicKeyFormatInt, SignKeyFormatInt, SymKeyFormatInt, VerifyKeyFormatInt};

#[derive(Serialize, Deserialize)]
pub enum PrivateKeyFormat
{
	Ecies
	{
		key: String, key_id: String
	},
}

impl PrivateKeyFormat
{
	pub fn from_string(v: &[u8]) -> serde_json::Result<Self>
	{
		from_slice::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub enum PublicKeyFormat
{
	Ecies
	{
		key: String, key_id: String
	},
}

impl PublicKeyFormat
{
	pub fn from_string(v: &[u8]) -> serde_json::Result<Self>
	{
		from_slice::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub enum SignKeyFormat
{
	Ed25519
	{
		key: String, key_id: String
	},
}

impl SignKeyFormat
{
	pub fn from_string(v: &[u8]) -> serde_json::Result<Self>
	{
		from_slice::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub enum VerifyKeyFormat
{
	Ed25519
	{
		key: String, key_id: String
	},
}

impl VerifyKeyFormat
{
	pub fn from_string(v: &[u8]) -> serde_json::Result<Self>
	{
		from_slice::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

/**
# Key data to communicate with other ffi programs via Strings

This data must be serialized for exporting and deserialized for import
 */
#[derive(Serialize, Deserialize)]
pub struct KeyData
{
	pub private_key: PrivateKeyFormat,
	pub public_key: PublicKeyFormat,
	pub sign_key: SignKeyFormat,
	pub verify_key: VerifyKeyFormat,
}

impl KeyData
{
	pub fn from_string(v: &[u8]) -> serde_json::Result<Self>
	{
		from_slice::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub enum SymKeyFormat
{
	Aes
	{
		key: String, key_id: String
	},
}

impl SymKeyFormat
{
	pub fn from_string(v: &[u8]) -> serde_json::Result<Self>
	{
		from_slice::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

pub(crate) fn import_private_key(private_key_string: &str) -> Result<PrivateKeyFormatInt, Error>
{
	let private_key_format = PrivateKeyFormat::from_string(private_key_string.as_bytes()).map_err(|_| Error::ImportingPrivateKeyFailed)?;

	import_private_key_from_format(&private_key_format)
}

pub(crate) fn import_private_key_from_format(key: &PrivateKeyFormat) -> Result<PrivateKeyFormatInt, Error>
{
	match key {
		PrivateKeyFormat::Ecies {
			key_id,
			key,
		} => {
			//to bytes via base64
			let bytes = Base64::decode_vec(key.as_str()).map_err(|_| Error::ImportingPrivateKeyFailed)?;

			let private_key: [u8; 32] = bytes
				.try_into()
				.map_err(|_| Error::ImportingPrivateKeyFailed)?;

			Ok(PrivateKeyFormatInt {
				key_id: key_id.clone(),
				key: Sk::Ecies(private_key),
			})
		},
	}
}

pub(crate) fn import_public_key(public_key_string: &str) -> Result<PublicKeyFormatInt, Error>
{
	let public_key_format = PublicKeyFormat::from_string(public_key_string.as_bytes()).map_err(|_| Error::ImportPublicKeyFailed)?;

	import_public_key_from_format(&public_key_format)
}

pub(crate) fn import_public_key_from_format(key: &PublicKeyFormat) -> Result<PublicKeyFormatInt, Error>
{
	match key {
		PublicKeyFormat::Ecies {
			key_id,
			key,
		} => {
			let bytes = Base64::decode_vec(key.as_str()).map_err(|_| Error::ImportPublicKeyFailed)?;

			let key = bytes.try_into().map_err(|_| Error::ImportPublicKeyFailed)?;

			Ok(PublicKeyFormatInt {
				key_id: key_id.clone(),
				key: Pk::Ecies(key),
			})
		},
	}
}

pub(crate) fn import_sign_key(sign_key_string: &str) -> Result<SignKeyFormatInt, Error>
{
	let sign_key_format = SignKeyFormat::from_string(sign_key_string.as_bytes()).map_err(|_| Error::ImportingSignKeyFailed)?;

	import_sign_key_from_format(&sign_key_format)
}

pub(crate) fn import_sign_key_from_format(key: &SignKeyFormat) -> Result<SignKeyFormatInt, Error>
{
	match key {
		SignKeyFormat::Ed25519 {
			key_id,
			key,
		} => {
			//to bytes via base64
			let bytes = Base64::decode_vec(key.as_str()).map_err(|_| Error::ImportingSignKeyFailed)?;

			let sign_key: [u8; 32] = bytes
				.try_into()
				.map_err(|_| Error::ImportingSignKeyFailed)?;

			Ok(SignKeyFormatInt {
				key_id: key_id.clone(),
				key: SignK::Ed25519(sign_key),
			})
		},
	}
}

pub(crate) fn export_private_key(private_key: PrivateKeyFormatInt) -> PrivateKeyFormat
{
	match private_key.key {
		Sk::Ecies(k) => {
			let private_key_string = Base64::encode_string(&k);

			PrivateKeyFormat::Ecies {
				key_id: private_key.key_id,
				key: private_key_string,
			}
		},
	}
}

pub(crate) fn export_public_key(public_key: PublicKeyFormatInt) -> PublicKeyFormat
{
	match public_key.key {
		Pk::Ecies(k) => {
			let public_key_string = Base64::encode_string(&k);

			PublicKeyFormat::Ecies {
				key_id: public_key.key_id,
				key: public_key_string,
			}
		},
	}
}

pub(crate) fn export_sign_key(sign_key: SignKeyFormatInt) -> SignKeyFormat
{
	match sign_key.key {
		SignK::Ed25519(k) => {
			let sign_key_string = Base64::encode_string(&k);

			SignKeyFormat::Ed25519 {
				key_id: sign_key.key_id,
				key: sign_key_string,
			}
		},
	}
}

pub(crate) fn export_verify_key(verify_key: VerifyKeyFormatInt) -> VerifyKeyFormat
{
	match verify_key.key {
		VerifyK::Ed25519(k) => {
			let verify_key_string = Base64::encode_string(&k);

			VerifyKeyFormat::Ed25519 {
				key_id: verify_key.key_id,
				key: verify_key_string,
			}
		},
	}
}

pub(crate) fn import_sym_key(key_string: &str) -> Result<SymKeyFormatInt, Error>
{
	let key_format = SymKeyFormat::from_string(key_string.as_bytes()).map_err(|_| Error::ImportSymmetricKeyFailed)?;

	import_sym_key_from_format(&key_format)
}

pub(crate) fn import_sym_key_from_format(key: &SymKeyFormat) -> Result<SymKeyFormatInt, Error>
{
	match key {
		SymKeyFormat::Aes {
			key,
			key_id,
		} => {
			//to bytes via base64
			let bytes = Base64::decode_vec(key.as_str()).map_err(|_| Error::ImportSymmetricKeyFailed)?;

			let key = bytes
				.try_into()
				.map_err(|_| Error::ImportSymmetricKeyFailed)?;

			Ok(SymKeyFormatInt {
				key_id: key_id.clone(),
				key: SymKey::Aes(key),
			})
		},
	}
}

pub(crate) fn export_sym_key(key: SymKeyFormatInt) -> SymKeyFormat
{
	match key.key {
		SymKey::Aes(k) => {
			let sym_key = Base64::encode_string(&k);

			SymKeyFormat::Aes {
				key_id: key.key_id,
				key: sym_key,
			}
		},
	}
}
