use anchor_lang::prelude::*;

use crate::models::{ todo::Todo, user::User };

#[derive(Accounts)]
pub struct CreateTodo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, space = std::mem::size_of::<Todo>() + 8, payer = authority)]
    pub todo: Box<Account<'info, Todo>>,

    #[account(mut,
        has_one = authority
    )]
    pub user: Box<Account<'info, User>>,

    pub system_program: Program<'info, System>,
}
