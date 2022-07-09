use alloc::string::String;
use alloc::vec::Vec;

use sentc_crypto_common::group::{GroupKeyServerOutput, GroupServerData, KeyRotationInput};
use sentc_crypto_common::user::UserPublicKeyData;
use sentc_crypto_core::Error;

use crate::group::{
	done_key_rotation_internally,
	get_group_keys_internally,
	key_rotation_internally,
	prepare_create_internally,
	prepare_group_keys_for_new_member_internally,
	GroupKeyData,
};
use crate::util::{PrivateKeyFormat, PrivateKeyFormatInt, PublicKeyFormat, SymKeyFormat};

pub struct GroupOutData
{
	pub keys: Vec<GroupKeyData>,
	pub group_id: String,
}

pub fn prepare_create(creators_public_key: &PublicKeyFormat) -> Result<String, Error>
{
	prepare_create_internally(&creators_public_key)
}

pub fn key_rotation(previous_group_key: &SymKeyFormat, invoker_public_key: &PublicKeyFormat) -> Result<String, Error>
{
	key_rotation_internally(&previous_group_key, &invoker_public_key)
}

pub fn done_key_rotation(
	private_key: &PrivateKeyFormat,
	public_key: &PublicKeyFormat,
	previous_group_key: &SymKeyFormat,
	server_output: &KeyRotationInput,
) -> Result<String, Error>
{
	done_key_rotation_internally(&private_key, &public_key, &previous_group_key, server_output)
}

fn get_group_keys(private_key: &PrivateKeyFormatInt, server_output: &GroupKeyServerOutput) -> Result<GroupKeyData, Error>
{
	get_group_keys_internally(private_key, server_output)
}

pub fn get_group_data(private_key: &PrivateKeyFormat, server_output: &GroupServerData) -> Result<GroupOutData, Error>
{
	let mut keys = Vec::with_capacity(server_output.keys.len());

	for key in &server_output.keys {
		keys.push(get_group_keys(private_key, key)?);
	}

	Ok(GroupOutData {
		keys,
		group_id: server_output.group_id.clone(),
	})
}

pub fn prepare_group_keys_for_new_member(requester_public_key_data: &UserPublicKeyData, group_keys: &[&SymKeyFormat]) -> Result<String, Error>
{
	prepare_group_keys_for_new_member_internally(requester_public_key_data, group_keys)
}

#[cfg(test)]
mod test
{
	use alloc::string::ToString;
	use alloc::vec;

	use sentc_crypto_common::group::{CreateData, GroupKeysForNewMemberServerInput};
	use sentc_crypto_core::SymKey;

	use super::*;
	use crate::group::test_fn::create_group;
	use crate::user::test_fn::create_user;

	#[test]
	fn test_create_group()
	{
		//create a rust dummy user
		let (user, _public_key, _verify_key) = create_user();

		let group = prepare_create(&user.public_key).unwrap();
		let group = CreateData::from_string(group.as_bytes()).unwrap();

		assert_eq!(group.creator_public_key_id, user.public_key.key_id);
	}

	#[test]
	fn test_create_and_get_group()
	{
		//test here only basic functions, if function panics. the key test is done in crypto mod

		let (user, _public_key, _verify_key) = create_user();

		let data = create_group(&user);

		assert_eq!(data.group_id, "123".to_string());
	}

	#[test]
	fn test_prepare_group_keys_for_new_member()
	{
		let (user, _public_key, _verify_key) = create_user();
		let (user1, public_key1, _verify_key1) = create_user();

		let group_create = prepare_create(&user.public_key).unwrap();
		let group_create = CreateData::from_string(group_create.as_bytes()).unwrap();

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
			keys: vec![group_server_output_user_0],
			keys_page: 0,
		};

		let group_data_user_0 = get_group_data(&user.private_key, &group_server_output_user_0).unwrap();

		//prepare the keys for user 1
		let out = prepare_group_keys_for_new_member(&public_key1, &[&group_data_user_0.keys[0].group_key]).unwrap();
		let out = GroupKeysForNewMemberServerInput::from_string(out.as_bytes()).unwrap();
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
			keys: vec![group_server_output_user_1],
			keys_page: 0,
		};

		let group_data_user_1 = get_group_data(&user1.private_key, &group_server_output_user_1).unwrap();

		assert_eq!(group_data_user_0.keys[0].group_key.key_id, group_data_user_1.keys[0].group_key.key_id);

		match (&group_data_user_0.keys[0].group_key.key, &group_data_user_1.keys[0].group_key.key) {
			(SymKey::Aes(k0), SymKey::Aes(k1)) => {
				assert_eq!(*k0, *k1);
			},
		}
	}
}
