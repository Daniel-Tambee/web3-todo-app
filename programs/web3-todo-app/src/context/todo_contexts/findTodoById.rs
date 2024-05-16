use anchor_lang::prelude::*;

use crate::models::{ todo::Todo, user::User };

#[derive(Accounts)]
pub struct findTodoById<'info> {
    pub authority: Signer<'info>,
    pub todo: Box<Account<'info, Todo>>,
    pub user: Box<Account<'info, User>>,
    pub system_program: Program<'info, System>,
}
