use anchor_lang::prelude::*;

use crate::models::user::User;

#[derive(Accounts)]
pub struct finduserById<'info> {
    #[account(signer)]
    pub authority: Signer<'info>,

    #[account(has_one = authority)]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}
