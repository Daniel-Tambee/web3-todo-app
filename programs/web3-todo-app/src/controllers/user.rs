use std::borrow::BorrowMut;

use anchor_lang::{ context::Context, solana_program::program_error::ProgramError };

use crate::{ context::user_contexts::createUser::CreateUser, models::{ todo::*, user::* } };

pub fn create_user(_ctx: Context<CreateUser>) -> Result<(), ProgramError> {
    let user = _ctx.accounts.user.as_mut();
    Ok(())
}
pub fn find_user_todos() -> Result<Vec<Todo>, ProgramError> {
    todo!()
}
pub fn find_user_by_id() -> Result<(), ProgramError> {
    todo!()
}
