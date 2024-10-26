use alloc::format;
use alloc::string::{String, ToString};

use sentc_crypto_core::Error;

#[derive(Debug)]
pub enum SdkUtilError
{
	Base(Error),
	JsonToStringFailed,
	JsonParseFailed(serde_json::Error),
	JsonParse,

	ServerErr(u32, String),

	ExportingPublicKeyFailed,
	DecodeSaltFailed,
	ImportingPrivateKeyFailed,
	ImportPublicKeyFailed,
	DecodePublicKeyFailed,
	AlgNotFound,
	ImportingKeyFromPemFailed,
	DecodeRandomValueFailed,
	ImportSymmetricKeyFailed,
	ImportingSignKeyFailed,
	ImportVerifyKeyFailed,
	ImportAuthMasterKeyFailed,
	ImportKeyFailed,

	DerivedKeyWrongFormat,
	InvalidJwt,
	InvalidJwtFormat,

	DecryptingLoginChallengeFailed,

	#[cfg(any(feature = "rustls", feature = "wasm"))]
	RequestErr(String),
	#[cfg(any(feature = "rustls", feature = "wasm"))]
	ResponseErrText,
	#[cfg(any(feature = "rustls", feature = "wasm"))]
	ResponseErrBytes,

	#[cfg(feature = "encryption")]
	SigFoundNotKey,
	#[cfg(feature = "encryption")]
	VerifyFailed,
	#[cfg(feature = "encryption")]
	DecodeEncryptedDataFailed,
	#[cfg(feature = "encryption")]
	SearchableEncryptionDataNotFound,
	#[cfg(feature = "encryption")]
	SearchableEncryptionDataTooLong,
}

/**
To convert the core error to sdk error
 */
impl From<Error> for SdkUtilError
{
	fn from(e: Error) -> Self
	{
		SdkUtilError::Base(e)
	}
}

impl From<serde_json::Error> for SdkUtilError
{
	fn from(value: serde_json::Error) -> Self
	{
		Self::JsonParseFailed(value)
	}
}

impl From<SdkUtilError> for String
{
	fn from(e: SdkUtilError) -> Self
	{
		err_to_msg(e)
	}
}

pub fn err_to_msg(error: SdkUtilError) -> String
{
	match error {
		SdkUtilError::Base(base_error) => {
			match base_error {
				Error::AlgNotFound => out_error("client_1", "The algorithms for this action was not found."),

				Error::DecodePrivateKeyFailed => out_error("client_3", "The private key has a wrong format."),
				Error::DecryptionFailedCiphertextShort => out_error("client_10", "cipher is too short."),
				Error::KeyCreationFailed => {
					out_error(
						"client_11",
						"Can't create a key. This normally happened when the used system has no mechanisms to create random numbers",
					)
				},
				Error::EncryptionFailed => out_error("client_12", "Can't encrypt symmetrically."),
				Error::EncryptionFailedRng => {
					out_error(
						"client_13",
						"Can't create random numbers. This normally happened when the used system has no mechanisms to create random numbers",
					)
				},
				Error::DecryptionFailed => {
					out_error(
						"client_14",
						"Can't decrypt the cipher. This happened when using a wrong key to decrypt",
					)
				},

				Error::PwHashFailed => {
					out_error(
						"client_20",
						"Can't hash the password. This happened when using a wrong algorithm or the output is wrong.",
					)
				},
				Error::PwSplitFailedLeft => out_error("client_21", "Can't hash the password. The input is too short"),
				Error::PwSplitFailedRight => out_error("client_21", "Can't hash the password. The input is too short"),
				Error::HashAuthKeyFailed => out_error("client_22", "Can't hash the password"),

				Error::KeyDecryptFailed => {
					out_error(
						"client_30",
						"Can't decrypt the key. Maybe a wrong master key was used.",
					)
				},
				Error::KeyDecryptionVerifyFailed => {
					out_error(
						"client_31",
						"Can't verify the key. Maybe the signature or the key a corrupted.",
					)
				},
				Error::SignKeyCreateFailed => out_error("client_40", "Can't create a sign key from given bytes"),
				Error::InitSignFailed => out_error("client_41", "Can't create a sign"),
				Error::DataToSignTooShort => out_error("client_42", "This data doesn't contains a sign"),
				Error::InitVerifyFailed => out_error("client_43", "Can't verify the data"),

				Error::HmacAuthFailedLength => out_error("client_50", "Can't auth the hmac"),

				Error::OpeRangeError => out_error("client_60", "Invalid input range"),
				Error::OpeStringToLarge => out_error("client_61", "String is too large to process"),
				Error::OpeHdgInvalidInputs => out_error("client_62", "Invalid inputs"),
			}
		},
		SdkUtilError::AlgNotFound => out_error("client_1", "The algorithms for this action was not found."),
		SdkUtilError::JsonToStringFailed => out_error("client_100", "Cannot create a string from this object"),
		SdkUtilError::JsonParse => out_error("client_102", "Cannot create an object from the input string"),
		SdkUtilError::JsonParseFailed(err) => {
			format!("{{\"status\": {}, \"error_message\": \"{}\"}}", "client_101", err)
		},
		SdkUtilError::ServerErr(code, msg) => {
			out_error(
				(String::from("server_") + code.to_string().as_str()).as_str(),
				msg.as_str(),
			)
		},

		//key decode error (from base64 string to the enum
		SdkUtilError::DerivedKeyWrongFormat => out_error("client_2", "The encrypted key has a wrong format."),
		SdkUtilError::DecodeRandomValueFailed => out_error("client_5", "Can't decode the client random value from registration"),
		//salt decode error (from base64 string to bytes)
		SdkUtilError::DecodeSaltFailed => out_error("client_4", "The salt has a wrong format"),
		SdkUtilError::DecodePublicKeyFailed => out_error("client_8", "Can't decode the public key. Maybe the format is wrong"),

		//import error
		SdkUtilError::ImportingSignKeyFailed => out_error("client_110", "Can't import the sign key"),
		SdkUtilError::ImportingPrivateKeyFailed => out_error("client_111", "Can't import the private key"),
		SdkUtilError::ImportSymmetricKeyFailed => out_error("client_112", "Can't import symmetric key"),
		SdkUtilError::ImportPublicKeyFailed => out_error("client_113", "Can't import public key"),
		SdkUtilError::ImportVerifyKeyFailed => out_error("client_114", "Can't import verify key"),
		SdkUtilError::ImportingKeyFromPemFailed => out_error("client_115", "Can't import this key. It has a wrong format"),
		SdkUtilError::ImportAuthMasterKeyFailed => out_error("client_116", "Can't import auth master key"),
		SdkUtilError::ImportKeyFailed => out_error("client_109", "Can't import the key"),

		//exporting error
		SdkUtilError::ExportingPublicKeyFailed => {
			out_error(
				"client_120",
				"Can't export the public key. It doesn't fit in a pem format",
			)
		},

		SdkUtilError::InvalidJwt => out_error("client_1100", "Jwt is invalid"),
		SdkUtilError::InvalidJwtFormat => out_error("client_1101", "Jwt has a wrong format"),

		SdkUtilError::DecryptingLoginChallengeFailed => out_error("client_1102", "Can't verify login."),

		#[cfg(any(feature = "rustls", feature = "wasm"))]
		SdkUtilError::RequestErr(e) => out_error("client_1000", format!("Can't send the request: {}", e).as_str()),
		#[cfg(any(feature = "rustls", feature = "wasm"))]
		SdkUtilError::ResponseErrText => out_error("client_1002", "Can't decode the response to text"),
		#[cfg(any(feature = "rustls", feature = "wasm"))]
		SdkUtilError::ResponseErrBytes => out_error("client_1003", "Can't get bytes from response"),

		#[cfg(feature = "encryption")]
		SdkUtilError::SigFoundNotKey => {
			out_error(
				"client_20",
				"The verification key can't verify this signature. The signature was signed by another key pair.",
			)
		},
		#[cfg(feature = "encryption")]
		SdkUtilError::VerifyFailed => out_error("client_22", "The verification failed. A wrong verify key was used"),
		#[cfg(feature = "encryption")]
		SdkUtilError::DecodeEncryptedDataFailed => out_error("client_10", "Can't decode the encrypted data"),
		#[cfg(feature = "encryption")]
		SdkUtilError::SearchableEncryptionDataTooLong => {
			out_error(
				"client_300",
				"The input data is too long to hash. The maximal length is 200 characters.",
			)
		},
		#[cfg(feature = "encryption")]
		SdkUtilError::SearchableEncryptionDataNotFound => out_error("client_301", "No data found to hash. Empty Strings are not allowed."),
	}
}

pub fn out_error(code: &str, message: &str) -> String
{
	//create the error in json to communicate with the other implementations, so they can use their own error handling

	format!("{{\"status\": \"{}\", \"error_message\": \"{}\"}}", code, message)
}
