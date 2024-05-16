use anchor_lang::prelude::*;

use crate::models::{todo::Todo, user::User};

#[derive(Accounts)]
pub struct MarkTodoDone<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub todo: Account<'info, Todo>,
    #[account(mut,has_one = authority)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}
