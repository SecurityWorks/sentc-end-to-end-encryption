pub(crate) mod group;

#[cfg(feature = "export")]
mod group_export;
mod group_rank_check;

pub use self::group::Group;
#[cfg(not(feature = "export"))]
pub use self::group::*;
#[cfg(feature = "export")]
pub use self::group_export::*;
pub use self::group_rank_check::*;

#[cfg(test)]
pub(crate) mod test_fn
{
	#[cfg(feature = "export")]
	use alloc::string::String;
	use alloc::string::ToString;
	use alloc::vec;
	use alloc::vec::Vec;

	use sentc_crypto_common::group::{CreateData, GroupHmacData, GroupKeyServerOutput, GroupServerData, GroupSortableData, GroupUserAccessBy};
	use sentc_crypto_common::ServerOutput;

	use super::*;
	use crate::entities::group::{GroupKeyData, GroupOutData};
	#[cfg(feature = "export")]
	use crate::entities::group::{GroupKeyDataExport, GroupOutDataExport};
	use crate::user::test_fn::TestUserKeyDataInt;

	#[cfg(feature = "std_keys")]
	pub type TestGroup = crate::keys::std::StdGroup;
	#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
	pub type TestGroup = crate::keys::fips::FipsGroup;
	#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
	pub type TestGroup = crate::keys::rec::RecGroup;

	#[cfg(feature = "std_keys")]
	pub type TestPublicKey = sentc_crypto_std_keys::util::PublicKey;
	#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
	pub type TestPublicKey = sentc_crypto_fips_keys::util::PublicKey;
	#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
	pub type TestPublicKey = sentc_crypto_rec_keys::util::PublicKey;

	#[cfg(feature = "std_keys")]
	pub type TestSymmetricKey = sentc_crypto_std_keys::util::SymmetricKey;
	#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
	pub type TestSymmetricKey = sentc_crypto_fips_keys::util::SymmetricKey;
	#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
	pub type TestSymmetricKey = sentc_crypto_rec_keys::util::SymmetricKey;

	#[cfg(feature = "std_keys")]
	pub type TestSecretKey = sentc_crypto_std_keys::util::SecretKey;
	#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
	pub type TestSecretKey = sentc_crypto_fips_keys::util::SecretKey;
	#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
	pub type TestSecretKey = sentc_crypto_rec_keys::util::SecretKey;

	#[cfg(feature = "std_keys")]
	pub type TestHmacKey = sentc_crypto_std_keys::util::HmacKey;
	#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
	pub type TestHmacKey = sentc_crypto_fips_keys::util::HmacKey;
	#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
	pub type TestHmacKey = sentc_crypto_rec_keys::util::HmacKey;

	#[cfg(feature = "std_keys")]
	pub type TestSortableKey = sentc_crypto_std_keys::util::SortableKey;
	#[cfg(all(feature = "fips_keys", not(feature = "std_keys")))]
	pub type TestSortableKey = sentc_crypto_fips_keys::util::SortableKey;
	#[cfg(all(feature = "rec_keys", not(feature = "std_keys")))]
	pub type TestSortableKey = sentc_crypto_rec_keys::util::SortableKey;

	pub(crate) fn create_group(
		user: &TestUserKeyDataInt,
	) -> (
		GroupOutData,
		Vec<GroupKeyData<TestSymmetricKey, TestSecretKey, TestPublicKey>>,
		GroupServerData,
		Vec<TestHmacKey>,
		Vec<TestSortableKey>,
	)
	{
		let group = TestGroup::prepare_create(&user.public_key, Some(&user.sign_key), "".to_string()).unwrap();
		let group = CreateData::from_string(group.as_str()).unwrap();

		let group_server_output = GroupKeyServerOutput {
			encrypted_group_key: group.encrypted_group_key,
			group_key_alg: group.group_key_alg,
			group_key_id: "123".to_string(),
			encrypted_private_group_key: group.encrypted_private_group_key,
			public_group_key: group.public_group_key,
			keypair_encrypt_alg: group.keypair_encrypt_alg,
			key_pair_id: "123".to_string(),
			user_public_key_id: "123".to_string(),
			time: 0,
			signed_by_user_id: group.signed_by_user_id,
			signed_by_user_sign_key_id: group.signed_by_user_sign_key_id,
			group_key_sig: group.group_key_sig,
			encrypted_sign_key: None,
			verify_key: None,
			keypair_sign_alg: None,
			keypair_sign_id: None,
			public_key_sig: None,
			public_key_sig_key_id: None,
		};

		let group_server_output = GroupServerData {
			group_id: "123".to_string(),
			parent_group_id: None,
			keys: vec![group_server_output],
			hmac_keys: vec![GroupHmacData {
				id: "123".to_string(),
				encrypted_hmac_encryption_key_id: "".to_string(),
				encrypted_hmac_key: group.encrypted_hmac_key,
				encrypted_hmac_alg: group.encrypted_hmac_alg,
				time: 0,
			}],
			sortable_keys: vec![GroupSortableData {
				id: "123".to_string(),
				encrypted_sortable_key: group.encrypted_sortable_key,
				encrypted_sortable_alg: group.encrypted_sortable_alg,
				encrypted_sortable_encryption_key_id: "".to_string(),
				time: 0,
			}],
			key_update: false,
			rank: 0,
			created_time: 0,
			joined_time: 0,
			access_by: GroupUserAccessBy::User,
			is_connected_group: false,
		};

		//to avoid the clone trait on the real type
		let group_ser_str = group_server_output.to_string().unwrap();

		let server_output = ServerOutput {
			status: true,
			err_msg: None,
			err_code: None,
			result: Some(group_server_output),
		};

		let out = group::get_group_data(server_output.to_string().unwrap().as_str()).unwrap();

		let group_keys: Vec<_> = out
			.keys
			.into_iter()
			.map(|k| TestGroup::decrypt_group_keys(&user.private_key, k, Some(&user.exported_verify_key)).unwrap())
			.collect();

		let hmac_keys = out
			.hmac_keys
			.into_iter()
			.map(|k| TestGroup::decrypt_group_hmac_key(&group_keys[0].group_key, k).unwrap())
			.collect();

		let sortable_keys = out
			.sortable_keys
			.into_iter()
			.map(|k| TestGroup::decrypt_group_sortable_key(&group_keys[0].group_key, k).unwrap())
			.collect();

		(
			GroupOutData {
				keys: vec![],
				hmac_keys: vec![],
				sortable_keys: vec![],
				parent_group_id: out.parent_group_id,
				key_update: out.key_update,
				created_time: out.created_time,
				joined_time: out.joined_time,
				rank: out.rank,
				group_id: out.group_id,
				access_by_group_as_member: out.access_by_group_as_member,
				access_by_parent_group: out.access_by_parent_group,
				is_connected_group: out.is_connected_group,
			},
			group_keys,
			GroupServerData::from_string(group_ser_str.as_str()).unwrap(),
			hmac_keys,
			sortable_keys,
		)
	}

	#[cfg(feature = "export")]
	pub(crate) fn create_group_export(
		user: &crate::entities::user::UserKeyDataExport,
	) -> (
		GroupOutDataExport,
		Vec<GroupKeyDataExport>,
		GroupServerData,
		Vec<String>,
		Vec<String>,
	)
	{
		let group = group_export::prepare_create(user.public_key.as_str(), Some(&user.sign_key), "".to_string()).unwrap();
		let group = CreateData::from_string(group.as_str()).unwrap();

		let group_server_output = GroupKeyServerOutput {
			encrypted_group_key: group.encrypted_group_key,
			group_key_alg: group.group_key_alg,
			group_key_id: "123".to_string(),
			encrypted_private_group_key: group.encrypted_private_group_key,
			public_group_key: group.public_group_key,
			keypair_encrypt_alg: group.keypair_encrypt_alg,
			key_pair_id: "123".to_string(),
			user_public_key_id: "123".to_string(),
			time: 0,
			signed_by_user_id: group.signed_by_user_id,
			signed_by_user_sign_key_id: group.signed_by_user_sign_key_id,
			group_key_sig: group.group_key_sig,
			encrypted_sign_key: None,
			verify_key: None,
			keypair_sign_alg: None,
			keypair_sign_id: None,
			public_key_sig: None,
			public_key_sig_key_id: None,
		};

		let group_server_output = GroupServerData {
			group_id: "123".to_string(),
			parent_group_id: None,
			keys: vec![group_server_output],
			hmac_keys: vec![GroupHmacData {
				id: "123".to_string(),
				encrypted_hmac_encryption_key_id: "".to_string(),
				encrypted_hmac_key: group.encrypted_hmac_key,
				encrypted_hmac_alg: group.encrypted_hmac_alg,
				time: 0,
			}],
			sortable_keys: vec![GroupSortableData {
				id: "123".to_string(),
				encrypted_sortable_key: group.encrypted_sortable_key,
				encrypted_sortable_alg: group.encrypted_sortable_alg,
				encrypted_sortable_encryption_key_id: "".to_string(),
				time: 0,
			}],
			key_update: false,
			rank: 0,
			created_time: 0,
			joined_time: 0,
			access_by: GroupUserAccessBy::User,
			is_connected_group: false,
		};

		//to avoid the clone trait on the real type
		let group_ser_str = group_server_output.to_string().unwrap();

		let server_output = ServerOutput {
			status: true,
			err_msg: None,
			err_code: None,
			result: Some(group_server_output),
		};

		let group_data = group_export::get_group_data(server_output.to_string().unwrap().as_str()).unwrap();

		let group_keys: Vec<_> = group_data
			.keys
			.iter()
			.map(|k| {
				group_export::decrypt_group_keys(
					user.private_key.as_str(),
					&k.key_data,
					Some(&user.exported_verify_key),
				)
				.unwrap()
			})
			.collect();

		let hmac_keys = group_data
			.hmac_keys
			.iter()
			.map(|k| group_export::decrypt_group_hmac_key(&group_keys[0].group_key, &k.key_data).unwrap())
			.collect();

		let sortable_keys = group_data
			.sortable_keys
			.iter()
			.map(|k| group_export::decrypt_group_sortable_key(&group_keys[0].group_key, &k.key_data).unwrap())
			.collect();

		(
			group_data,
			group_keys,
			GroupServerData::from_string(group_ser_str.as_str()).unwrap(),
			hmac_keys,
			sortable_keys,
		)
	}
}
