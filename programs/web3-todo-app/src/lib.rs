pub mod context;
pub mod controllers;
pub mod models;
use anchor_lang::prelude::*;
declare_id!("9VELjY4xnvMWThciCAuMDZNiDYSCd3341BD7PmU1UxwC");

#[program]
pub mod web3_todo_app {
    use super::*;
    use controllers::{
        todo::{ create_todo, find_todo_by_id, find_todo_by_wallet, find_todo_is_done },
        user::{ create_user, find_user_by_id, find_user_todos },
    };
}
