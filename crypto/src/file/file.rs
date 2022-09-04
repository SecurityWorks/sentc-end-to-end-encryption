use alloc::string::{String, ToString};

use sentc_crypto_common::file::BelongsToType;

use crate::file::{done_register_file_internally, prepare_file_name_update_internally, prepare_register_file_internally};
use crate::util::import_sym_key;
use crate::SdkError;

pub fn prepare_register_file(key: &str, belongs_to_id: &str, belongs_to_type: &str, file_name: &str) -> Result<(String, String), String>
{
	let key = import_sym_key(key)?;

	let belongs_to_id = match belongs_to_id {
		"" => None,
		_ => Some(belongs_to_id.to_string()),
	};

	let file_name = match file_name {
		"" => None,
		_ => Some(file_name.to_string()),
	};

	let belongs_to_type: BelongsToType = serde_json::from_str(belongs_to_type).map_err(|e| SdkError::JsonParseFailed(e))?;

	let (server_input, encrypted_file_name) = prepare_register_file_internally(&key, belongs_to_id, belongs_to_type, file_name)?;

	let encrypted_file_name = match encrypted_file_name {
		None => "".to_string(),
		Some(n) => n,
	};

	Ok((server_input, encrypted_file_name))
}

pub fn done_register_file(server_output: &str) -> Result<(String, String), String>
{
	Ok(done_register_file_internally(server_output)?)
}

pub fn prepare_file_name_update(key: &str, file_name: &str) -> Result<String, String>
{
	let key = import_sym_key(key)?;

	let file_name = match file_name {
		"" => None,
		_ => Some(file_name.to_string()),
	};

	Ok(prepare_file_name_update_internally(&key, file_name)?)
}
