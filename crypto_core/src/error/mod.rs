#[derive(Debug)]
pub enum Error
{
	DecryptionFailedCiphertextShort,

	KeyCreationFailed,

	EncryptionFailed,
	EncryptionFailedRng,
	DecryptionFailed,

	PwHashFailed,
	PwSplitFailedLeft,
	PwSplitFailedRight,
	HashAuthKeyFailed,

	KeyDecryptFailed,

	SignKeyCreateFailed,
	InitSignFailed,
	DataToSignTooShort,
	InitVerifyFailed,

	AlgNotFound,
	DecodeSaltFailed,
	DerivedKeyWrongFormat,

	LoginServerOutputWrong,
	KeyRotationServerOutputWrong,

	DecodePrivateKeyFailed,

	ImportingPrivateKeyFailed,
	ImportingSignKeyFailed,
	ImportSymmetricKeyFailed,
	ImportPublicKeyFailed,

	ExportingPublicKeyFailed,
	ImportingKeyFromPemFailed,

	JsonToStringFailed,
	JsonParseFailed,
}
