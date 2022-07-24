use alloc::string::String;
use alloc::vec::Vec;

use sentc_crypto_common::group::{GroupKeyServerOutput, GroupServerData, KeyRotationInput};
use sentc_crypto_common::user::UserPublicKeyData;
use sentc_crypto_common::GroupId;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

use crate::group::{
	done_key_rotation_internally,
	get_group_keys_internally,
	key_rotation_internally,
	prepare_create_internally,
	prepare_group_keys_for_new_member_internally,
};
use crate::util::{
	export_private_key_to_string,
	export_public_key_to_string,
	export_sym_key_to_string,
	import_private_key,
	import_public_key,
	import_sym_key,
	import_sym_key_from_format,
	PrivateKeyFormatInt,
	SymKeyFormat,
	SymKeyFormatInt,
};
use crate::{err_to_msg, SdkError};

#[derive(Serialize, Deserialize)]
pub struct GroupKeyData
{
	pub private_group_key: String,
	pub public_group_key: String,
	pub group_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupOutData
{
	pub group_id: String,
	pub keys: Vec<GroupKeyData>,
}

impl GroupOutData
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

#[derive(Serialize, Deserialize)]
pub struct GroupKeys(Vec<SymKeyFormat>);

impl GroupKeys
{
	pub fn from_string(v: &str) -> serde_json::Result<Self>
	{
		from_str::<Self>(v)
	}

	pub fn to_string(&self) -> serde_json::Result<String>
	{
		to_string(self)
	}
}

pub fn prepare_create(creators_public_key: &str, parent_group_id: Option<GroupId>) -> Result<String, String>
{
	let creators_public_key = import_public_key(creators_public_key).map_err(|e| err_to_msg(e))?;

	prepare_create_internally(&creators_public_key, parent_group_id).map_err(|e| err_to_msg(e))
}

pub fn key_rotation(previous_group_key: &str, invoker_public_key: &str) -> Result<String, String>
{
	//the ids comes from the storage of the current impl from the sdk, the group key id comes from get group
	let previous_group_key = import_sym_key(previous_group_key).map_err(|e| err_to_msg(e))?;

	let invoker_public_key = import_public_key(invoker_public_key).map_err(|e| err_to_msg(e))?;

	key_rotation_internally(&previous_group_key, &invoker_public_key).map_err(|e| err_to_msg(e))
}

pub fn done_key_rotation(private_key: &str, public_key: &str, previous_group_key: &str, server_output: &str) -> Result<String, String>
{
	let previous_group_key = import_sym_key(previous_group_key).map_err(|e| err_to_msg(e))?;

	let private_key = import_private_key(private_key).map_err(|e| err_to_msg(e))?;

	let public_key = import_public_key(public_key).map_err(|e| err_to_msg(e))?;

	let server_output = KeyRotationInput::from_string(server_output).map_err(|_| err_to_msg(SdkError::KeyRotationServerOutputWrong))?;

	done_key_rotation_internally(&private_key, &public_key, &previous_group_key, &server_output).map_err(|e| err_to_msg(e))
}

fn get_group_keys(private_key: &PrivateKeyFormatInt, server_output: &GroupKeyServerOutput) -> Result<GroupKeyData, SdkError>
{
	let result = get_group_keys_internally(&private_key, &server_output)?;

	let private_group_key = export_private_key_to_string(result.private_group_key)?;
	let public_group_key = export_public_key_to_string(result.public_group_key)?;
	let group_key = export_sym_key_to_string(result.group_key)?;

	Ok(GroupKeyData {
		private_group_key,
		public_group_key,
		group_key,
	})
}

pub fn get_group_data(private_key: &str, server_output: &str) -> Result<GroupOutData, String>
{
	let server_output = GroupServerData::from_string(server_output).map_err(|_e| err_to_msg(SdkError::JsonParseFailed))?;

	let private_key = import_private_key(private_key).map_err(|e| err_to_msg(e))?;

	//resolve a group key page
	let mut keys = Vec::with_capacity(server_output.keys.len());

	for key in server_output.keys {
		let value = get_group_keys(&private_key, &key).map_err(|e| err_to_msg(e))?;

		keys.push(value);
	}

	Ok(GroupOutData {
		group_id: server_output.group_id,
		keys,
	})
}

pub fn prepare_group_keys_for_new_member(requester_public_key_data: &str, group_keys: &str) -> Result<String, String>
{
	let requester_public_key_data = UserPublicKeyData::from_string(requester_public_key_data).map_err(|_e| err_to_msg(SdkError::JsonParseFailed))?;

	let group_keys = GroupKeys::from_string(group_keys)
		.map_err(|_| err_to_msg(SdkError::JsonParseFailed))?
		.0;

	let mut saved_keys = Vec::with_capacity(group_keys.len());

	//split group key and id
	for group_key in group_keys {
		let key = import_sym_key_from_format(&group_key).map_err(|e| err_to_msg(e))?;

		saved_keys.push(key);
	}

	let split_group_keys = prepare_group_keys_for_new_member_with_ref(&saved_keys);

	prepare_group_keys_for_new_member_internally(&requester_public_key_data, &split_group_keys).map_err(|e| err_to_msg(e))
}

fn prepare_group_keys_for_new_member_with_ref(saved_keys: &Vec<SymKeyFormatInt>) -> Vec<&SymKeyFormatInt>
{
	//this is needed because we need only ref of the group key not the group key itself.
	//but for the non rust version the key is just a string which gets

	let mut split_group_keys = Vec::with_capacity(saved_keys.len());

	for saved_key in saved_keys {
		split_group_keys.push(saved_key);
	}

	split_group_keys
}

#[cfg(test)]
mod test
{
	use alloc::string::ToString;
	use alloc::vec;

	use base64ct::{Base64, Encoding};
	use sentc_crypto_common::group::{CreateData, DoneKeyRotationData, GroupKeysForNewMemberServerInput, KeyRotationData};
	use sentc_crypto_core::crypto::encrypt_asymmetric as encrypt_asymmetric_core;
	use sentc_crypto_core::SymKey;

	use super::*;
	use crate::group::test_fn::create_group;
	use crate::user::test_fn::create_user;

	#[test]
	fn test_create_group()
	{
		//create a rust dummy user
		let user = create_user();

		let group = prepare_create(&user.public_key.as_str(), None).unwrap();
		let group = CreateData::from_string(group.as_str()).unwrap();

		let pk = import_public_key(user.public_key.as_str()).unwrap();

		assert_eq!(group.creator_public_key_id, pk.key_id);
	}

	#[test]
	fn test_create_and_get_group()
	{
		//test here only basic functions, if function panics. the key test is done in crypto mod

		let user = create_user();

		let (data, _) = create_group(&user);

		assert_eq!(data.group_id, "123".to_string());
	}

	#[test]
	fn test_prepare_group_keys_for_new_member()
	{
		let user = create_user();

		let user1 = create_user();

		let group_create = prepare_create(user.public_key.as_str(), None).unwrap();
		let group_create = CreateData::from_string(group_create.as_str()).unwrap();

		let group_server_output_user_0 = GroupKeyServerOutput {
			encrypted_group_key: group_create.encrypted_group_key.to_string(),
			group_key_alg: group_create.group_key_alg.to_string(),
			group_key_id: "123".to_string(),
			encrypted_private_group_key: group_create.encrypted_private_group_key.to_string(),
			public_group_key: group_create.public_group_key.to_string(),
			keypair_encrypt_alg: group_create.keypair_encrypt_alg.to_string(),
			key_pair_id: "123".to_string(),
			user_public_key_id: "123".to_string(),
		};

		let group_server_output_user_0 = GroupServerData {
			group_id: "123".to_string(),
			parent_group_id: None,
			keys: vec![group_server_output_user_0],
			keys_page: 0,
			key_update: false,
		};

		let group_data_user_0 = get_group_data(
			user.private_key.as_str(),
			group_server_output_user_0.to_string().unwrap().as_str(),
		)
		.unwrap();
		let group_key_user_0 = group_data_user_0.keys[0].group_key.as_str();

		let group_keys = GroupKeys(vec![SymKeyFormat::from_string(group_key_user_0).unwrap()]);

		//prepare the keys for user 1
		let out = prepare_group_keys_for_new_member(
			user1.exported_public_key.as_str(),
			group_keys.to_string().unwrap().as_str(),
		)
		.unwrap();
		let out = GroupKeysForNewMemberServerInput::from_string(out.as_str()).unwrap();
		let out_group_1 = &out.0[0]; //this group only got one key

		let group_server_output_user_1 = GroupKeyServerOutput {
			encrypted_group_key: out_group_1.encrypted_group_key.to_string(),
			group_key_alg: out_group_1.alg.to_string(),
			group_key_id: out_group_1.key_id.to_string(),
			encrypted_private_group_key: group_create.encrypted_private_group_key,
			public_group_key: group_create.public_group_key,
			keypair_encrypt_alg: group_create.keypair_encrypt_alg,
			key_pair_id: "123".to_string(),
			user_public_key_id: "123".to_string(),
		};

		let group_server_output_user_1 = GroupServerData {
			group_id: "123".to_string(),
			parent_group_id: None,
			keys: vec![group_server_output_user_1],
			keys_page: 0,
			key_update: false,
		};

		let group_data_user_1 = get_group_data(
			user1.private_key.as_str(),
			group_server_output_user_1.to_string().unwrap().as_str(),
		)
		.unwrap();

		let group_key_0 = import_sym_key(group_data_user_0.keys[0].group_key.as_str()).unwrap();
		let group_key_1 = import_sym_key(group_data_user_1.keys[0].group_key.as_str()).unwrap();

		assert_eq!(group_key_0.key_id, group_key_1.key_id);

		match (&group_key_0.key, &group_key_1.key) {
			(SymKey::Aes(k0), SymKey::Aes(k1)) => {
				assert_eq!(*k0, *k1);
			},
		}
	}

	#[test]
	fn test_key_rotation()
	{
		let user = create_user();

		let (data, group_server_out) = create_group(&user);

		let rotation_out = key_rotation(data.keys[0].group_key.as_str(), user.public_key.as_str()).unwrap();
		let rotation_out = KeyRotationData::from_string(rotation_out.as_str()).unwrap();

		//get the new group key directly because for the invoker the key is already encrypted by the own public key
		let server_key_output_direct = GroupKeyServerOutput {
			encrypted_group_key: rotation_out.encrypted_group_key_by_user.to_string(),
			group_key_alg: group_server_out.keys[0].group_key_alg.to_string(),
			group_key_id: group_server_out.keys[0].group_key_id.to_string(),
			encrypted_private_group_key: rotation_out.encrypted_private_group_key.to_string(),
			public_group_key: rotation_out.public_group_key.to_string(),
			keypair_encrypt_alg: rotation_out.keypair_encrypt_alg.to_string(),
			key_pair_id: "new_key_id_from_server".to_string(),
			user_public_key_id: "abc".to_string(),
		};

		let new_group_key_direct = get_group_keys(
			&import_private_key(user.private_key.as_str()).unwrap(),
			&server_key_output_direct,
		)
		.unwrap();

		//simulate server key rotation encrypt. encrypt the ephemeral_key (encrypted by the previous room key) with the public keys of all users
		let encrypted_ephemeral_key = Base64::decode_vec(rotation_out.encrypted_ephemeral_key.as_str()).unwrap();
		let encrypted_ephemeral_key_by_group_key_and_public_key = encrypt_asymmetric_core(
			&import_public_key(user.public_key.as_str()).unwrap().key,
			&encrypted_ephemeral_key,
		)
		.unwrap();

		let server_output = KeyRotationInput {
			encrypted_ephemeral_key_by_group_key_and_public_key: Base64::encode_string(&encrypted_ephemeral_key_by_group_key_and_public_key),
			encrypted_group_key_by_ephemeral: rotation_out.encrypted_group_key_by_ephemeral.to_string(),
			ephemeral_alg: rotation_out.ephemeral_alg.to_string(),
			previous_group_key_id: rotation_out.previous_group_key_id.to_string(),
		};

		let done_key_rotation = done_key_rotation(
			user.private_key.as_str(),
			user.public_key.as_str(),
			data.keys[0].group_key.as_str(),
			server_output.to_string().unwrap().as_str(),
		)
		.unwrap();
		let done_key_rotation = DoneKeyRotationData::from_string(done_key_rotation.as_str()).unwrap();

		//get the new group keys
		let server_key_output = GroupKeyServerOutput {
			encrypted_group_key: done_key_rotation.encrypted_new_group_key.to_string(),
			group_key_alg: group_server_out.keys[0].group_key_alg.to_string(),
			group_key_id: group_server_out.keys[0].group_key_id.to_string(),
			encrypted_private_group_key: rotation_out.encrypted_private_group_key.to_string(),
			public_group_key: rotation_out.public_group_key.to_string(),
			keypair_encrypt_alg: rotation_out.keypair_encrypt_alg.to_string(),
			key_pair_id: "new_key_id_from_server".to_string(),
			user_public_key_id: done_key_rotation.public_key_id.to_string(),
		};

		let out = get_group_keys(
			&import_private_key(user.private_key.as_str()).unwrap(),
			&server_key_output,
		)
		.unwrap();

		let old_group_key = import_sym_key(data.keys[0].group_key.to_string().as_str()).unwrap();

		let new_group_key_direct = import_sym_key(new_group_key_direct.group_key.as_str()).unwrap();

		let new_group_key = import_sym_key(out.group_key.as_str()).unwrap();

		//the new group key must be different after key rotation
		match (&old_group_key.key, &new_group_key.key) {
			(SymKey::Aes(k_old), SymKey::Aes(k_new)) => {
				assert_ne!(*k_old, *k_new);
			},
		}

		match (&new_group_key_direct.key, &new_group_key.key) {
			(SymKey::Aes(k_0), SymKey::Aes(k_1)) => {
				//should be the same for all users
				assert_eq!(*k_0, *k_1);
			},
		}
	}
}
