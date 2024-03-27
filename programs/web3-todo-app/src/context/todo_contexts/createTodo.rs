use anchor_lang::prelude::*;

use crate::models::todo::Todo;

#[derive(Accounts)]
pub struct CreateTodo<'info> {
    pub authority: Signer<'info>,
    pub todo: Box<Account<'info, Todo>>,
    pub system_program: Program<'info, System>,
}
