use crate::SdkError;

pub type VoidRes = Result<(), SdkError>;

#[allow(clippy::needless_return_with_question_mark)]
pub fn check_kick_user(user_rank: i32, admin_rank: i32) -> VoidRes
{
	if admin_rank > 2 {
		return Err(SdkError::GroupPermission)?;
	}

	if admin_rank > user_rank {
		//user has a higher rank
		return Err(SdkError::GroupUserKickRank)?;
	}

	Ok(())
}

#[allow(clippy::needless_return_with_question_mark)]
pub fn check_group_delete(admin_rank: i32) -> VoidRes
{
	if admin_rank > 1 {
		return Err(SdkError::GroupPermission)?;
	}

	Ok(())
}

#[allow(clippy::needless_return_with_question_mark)]
pub fn check_delete_user_rank(admin_rank: i32) -> VoidRes
{
	if admin_rank > 2 {
		return Err(SdkError::GroupPermission)?;
	}

	Ok(())
}

#[allow(clippy::needless_return_with_question_mark)]
pub fn check_get_join_reqs(admin_rank: i32) -> VoidRes
{
	if admin_rank > 2 {
		return Err(SdkError::GroupPermission)?;
	}

	Ok(())
}

#[allow(clippy::needless_return_with_question_mark)]
pub fn check_make_invite_req(admin_rank: i32) -> VoidRes
{
	if admin_rank > 2 {
		return Err(SdkError::GroupPermission)?;
	}

	Ok(())
}

#[allow(clippy::needless_return_with_question_mark)]
pub fn check_create_sub_group(admin_rank: i32) -> VoidRes
{
	if admin_rank > 1 {
		return Err(SdkError::GroupPermission)?;
	}

	Ok(())
}

#[allow(clippy::needless_return_with_question_mark)]
pub fn check_sent_join_req_list(admin_rank: i32) -> VoidRes
{
	if admin_rank > 1 {
		return Err(SdkError::GroupPermission)?;
	}

	Ok(())
}
