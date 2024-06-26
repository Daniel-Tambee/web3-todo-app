use super::todo::Todo;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[account]
pub struct User {
    pub authority: Pubkey,
    pub id: usize,
    pub count: usize,
    pub todos:  Vec<Todo>,
}
