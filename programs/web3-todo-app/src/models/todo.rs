use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;


#[account]
pub struct Todo {
    pub owner: Pubkey,
    pub id: usize,
    pub content: String,
    pub is_done: bool,
}
