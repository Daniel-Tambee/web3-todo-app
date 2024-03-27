pub mod context;
pub mod controllers;
pub mod models;
use anchor_lang::prelude::*;
declare_id!("9VELjY4xnvMWThciCAuMDZNiDYSCd3341BD7PmU1UxwC");

#[program]
pub mod web3_todo_app {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
