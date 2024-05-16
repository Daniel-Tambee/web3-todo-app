use anchor_lang::prelude::*;

use crate::models::user::User;

#[derive(Accounts)]
pub struct findTodoByWallet<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,has_one = authority)]
    pub user: Box<Account<'info, User>>,
    pub system_program: Program<'info, System>,
}
