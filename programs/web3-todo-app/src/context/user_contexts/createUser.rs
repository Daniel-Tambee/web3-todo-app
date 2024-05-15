use anchor_lang::prelude::*;

use crate::models::user::User;

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    #[account(init, space = std::mem::size_of::<User>() + 8, payer = authority)]
    pub user: Box<Account<'info, User>>,

    pub system_program: Program<'info, System>,
}
