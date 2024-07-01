use digest::Digest;
use openssl::pkey::{HasPrivate, HasPublic, PKey, Private, Public as Op};
use pqcrypto_dilithium::dilithium3::{PublicKey, SecretKey};
use pqcrypto_traits::sign::{PublicKey as PkT, SecretKey as SkT};
use sentc_crypto_core::cryptomat::{Sig, SignK, SignKeyPair, SymKey, VerifyK};
use sentc_crypto_core::{crypto_alg_str_impl, Error};
use sentc_crypto_fips_keys::core::sign::{import_pk, import_sk};

use crate::core::sign::pqc_dilithium::SIG_LENGTH;
use crate::core::sign::{SignKey, Signature, VerifyKey};
use crate::core::{export_pk, export_sk};
use crate::{hybrid_import_export, hybrid_sk_from_bytes};

pub const ED25519_DILITHIUM_HYBRID_REC_OUTPUT: &str = "ED25519_DILITHIUM_REC_3";

pub struct Ed25519DilithiumHybridSig
{
	x: Vec<u8>,
	k: Vec<u8>,
}
crypto_alg_str_impl!(Ed25519DilithiumHybridSig, ED25519_DILITHIUM_HYBRID_REC_OUTPUT);

impl Ed25519DilithiumHybridSig
{
	pub fn get_raw_sig(&self) -> (&[u8], &[u8])
	{
		(&self.x, &self.k)
	}

	pub fn from_bytes_owned(x: Vec<u8>, k: Vec<u8>) -> Self
	{
		Self {
			x,
			k,
		}
	}
}

impl Into<Signature> for Ed25519DilithiumHybridSig
{
	fn into(self) -> Signature
	{
		Signature::Ed25519DilithiumHybrid(self)
	}
}

impl Into<Vec<u8>> for Ed25519DilithiumHybridSig
{
	fn into(self) -> Vec<u8>
	{
		let mut output = Vec::with_capacity(self.x.len() + self.k.len());
		output.extend_from_slice(&self.x);
		output.extend_from_slice(&self.k);

		output
	}
}

impl Sig for Ed25519DilithiumHybridSig {}

pub struct Ed25519DilithiumHybridVerifyKey
{
	x: PKey<Op>,
	k: PublicKey,
}

hybrid_import_export!(Ed25519DilithiumHybridVerifyKey, import_pk, export_pk, PublicKey);
crypto_alg_str_impl!(Ed25519DilithiumHybridVerifyKey, ED25519_DILITHIUM_HYBRID_REC_OUTPUT);

impl Into<VerifyKey> for Ed25519DilithiumHybridVerifyKey
{
	fn into(self) -> VerifyKey
	{
		VerifyKey::Ed25519DilithiumHybrid(self)
	}
}

impl VerifyK for Ed25519DilithiumHybridVerifyKey
{
	type Signature = Ed25519DilithiumHybridSig;

	fn verify<'a>(&self, data_with_sig: &'a [u8]) -> Result<(&'a [u8], bool), Error>
	{
		let (sig, data) = split_sig_and_data(data_with_sig)?;

		//now split the both sig
		let (sig_x, sig_k) = sentc_crypto_core::split_sig_and_data(sig, super::ed25519::SIG_LENGTH)?;

		Ok((data, verify_internally(&self.x, &self.k, sig_x, sig_k, data)?))
	}

	fn verify_only(&self, sig: &Self::Signature, data: &[u8]) -> Result<bool, Error>
	{
		verify_internally(&self.x, &self.k, &sig.x, &sig.k, data)
	}

	fn create_hash<D: Digest>(&self, hasher: &mut D)
	{
		hasher.update(&export_pk(&self.x).unwrap());
		hasher.update(self.k.as_bytes());
	}
}

pub struct Ed25519DilithiumHybridSignK
{
	x: PKey<Private>,
	k: SecretKey,
}

hybrid_import_export!(Ed25519DilithiumHybridSignK, import_sk, export_sk, SecretKey);
hybrid_sk_from_bytes!(Ed25519DilithiumHybridSignK, import_sk, SecretKey);
crypto_alg_str_impl!(Ed25519DilithiumHybridSignK, ED25519_DILITHIUM_HYBRID_REC_OUTPUT);

impl Into<SignKey> for Ed25519DilithiumHybridSignK
{
	fn into(self) -> SignKey
	{
		SignKey::Ed25519DilithiumHybrid(self)
	}
}

impl SignK for Ed25519DilithiumHybridSignK
{
	type Signature = Ed25519DilithiumHybridSig;

	fn encrypt_by_master_key<M: SymKey>(&self, master_key: &M) -> Result<Vec<u8>, Error>
	{
		let k = [&export_sk(&self.x)?, self.k.as_bytes()].concat();

		master_key.encrypt(&k)
	}

	fn sign(&self, data: &[u8]) -> Result<Vec<u8>, Error>
	{
		let (sig_x, sig_k) = sign_internal(&self.x, &self.k, data)?;

		let mut output = Vec::with_capacity(sig_x.len() + sig_k.len() + data.len());
		output.extend_from_slice(&sig_x);
		output.extend_from_slice(&sig_k);
		output.extend_from_slice(data);

		Ok(output)
	}

	fn sign_only<D: AsRef<[u8]>>(&self, data: D) -> Result<Self::Signature, Error>
	{
		let (x, k) = sign_internal(&self.x, &self.k, data.as_ref())?;

		Ok(Ed25519DilithiumHybridSig {
			x,
			k,
		})
	}
}

impl SignKeyPair for Ed25519DilithiumHybridSignK
{
	type SignKey = Self;
	type VerifyKey = Ed25519DilithiumHybridVerifyKey;

	fn generate_key_pair() -> Result<(Self::SignKey, Self::VerifyKey), Error>
	{
		let (xvk, xsk) = sentc_crypto_fips_keys::core::sign::generate_key_pair()?;
		let (pk, sk) = super::pqc_dilithium::generate_key_pair();

		Ok((
			Ed25519DilithiumHybridSignK {
				x: xsk,
				k: sk,
			},
			Ed25519DilithiumHybridVerifyKey {
				x: xvk,
				k: pk,
			},
		))
	}
}

pub(crate) fn split_sig_and_data(data_with_sig: &[u8]) -> Result<(&[u8], &[u8]), Error>
{
	sentc_crypto_core::split_sig_and_data(data_with_sig, super::ed25519::SIG_LENGTH + SIG_LENGTH)
}

//__________________________________________________________________________________________________
//internally function

fn sign_internal<T: HasPrivate>(x: &PKey<T>, k: &SecretKey, data: &[u8]) -> Result<(Vec<u8>, Vec<u8>), Error>
{
	let sig_x = sentc_crypto_fips_keys::core::sign::sign_internally(x, data)?;

	let sig_k = super::pqc_dilithium::sign_internally(k, &[data, &sig_x].concat());

	Ok((sig_x, sig_k))
}

fn verify_internally<T: HasPublic>(x: &PKey<T>, k: &PublicKey, sig_x: &[u8], sig_k: &[u8], data: &[u8]) -> Result<bool, Error>
{
	let res = super::pqc_dilithium::verify_internally(k, sig_k, &[data, sig_x].concat())?;

	if !res {
		return Ok(res);
	}

	sentc_crypto_fips_keys::core::sign::verify_internally(x, sig_x, data)
}

#[cfg(test)]
mod test
{
	use sentc_crypto_core::user::safety_number;
	use sentc_crypto_core::Error::DataToSignTooShort;

	use super::*;
	use crate::core::sign::ed25519::SIG_LENGTH;

	#[test]
	fn test_generate_keypair()
	{
		let _ = Ed25519DilithiumHybridSignK::generate_key_pair().unwrap();
	}

	#[test]
	fn test_sign_and_verify()
	{
		let (sk, vk) = Ed25519DilithiumHybridSignK::generate_key_pair().unwrap();

		let text = "Hello world üöäéèßê°";

		let data_with_sig = sk.sign(text.as_bytes()).unwrap();

		let (data, check) = vk.verify(&data_with_sig).unwrap();

		assert!(check);
		assert_eq!(data, text.as_bytes());
	}

	#[test]
	fn test_wrong_verify()
	{
		let (_sk, vk) = Ed25519DilithiumHybridSignK::generate_key_pair().unwrap();
		let (sk, _vk) = Ed25519DilithiumHybridSignK::generate_key_pair().unwrap();

		let text = "Hello world üöäéèßê°";

		let data_with_sig = sk.sign(text.as_bytes()).unwrap();

		let (data, check) = vk.verify(&data_with_sig).unwrap();

		assert!(!check);
		assert_eq!(data, text.as_bytes());
	}

	#[test]
	fn test_too_short_sig_bytes()
	{
		let (sk, vk) = Ed25519DilithiumHybridSignK::generate_key_pair().unwrap();
		let text = "Hello world üöäéèßê°";

		let data_with_sig = sk.sign(text.as_bytes()).unwrap();

		let data_with_sig = &data_with_sig[..31];

		let check_result = vk.verify(data_with_sig);

		assert!(matches!(check_result, Err(DataToSignTooShort)));
	}

	#[test]
	fn test_wrong_sig_bytes()
	{
		let (sk, vk) = Ed25519DilithiumHybridSignK::generate_key_pair().unwrap();
		let text = "Hello world üöäéèßê°";

		let data_with_sig = sk.sign(text.as_bytes()).unwrap();

		let data_with_sig = &data_with_sig[..crate::core::sign::pqc_dilithium::SIG_LENGTH + SIG_LENGTH + 2];

		let (_data, check) = vk.verify(data_with_sig).unwrap();

		assert!(!check);
	}

	#[test]
	fn test_safety_number()
	{
		let (_sk, vk) = Ed25519DilithiumHybridSignK::generate_key_pair().unwrap();

		let number = safety_number(&vk, "123", None, None);

		assert_eq!(number.len(), 32);
	}

	#[test]
	fn test_combined_safety_number()
	{
		let (_, vk) = Ed25519DilithiumHybridSignK::generate_key_pair().unwrap();
		let (_, vk1) = Ed25519DilithiumHybridSignK::generate_key_pair().unwrap();

		let number = safety_number(&vk, "123", Some(&vk1), Some("321"));

		assert_eq!(number.len(), 32);

		//test the other way around

		let number_2 = safety_number(&vk1, "321", Some(&vk), Some("123"));

		assert_eq!(number_2.len(), 32);

		assert_ne!(number, number_2);
	}
}
