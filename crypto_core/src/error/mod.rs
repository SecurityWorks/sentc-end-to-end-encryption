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

	DecodePrivateKeyFailed,

	ImportingPrivateKeyFailed,
	ImportingSignKeyFailed,

	ExportingPublicKeyFailed,
	ImportingPublicKeyFailed,

	JsonToStringFailed,
	JsonParseFailed,
}