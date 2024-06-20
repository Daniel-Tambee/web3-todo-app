pub mod context;
pub mod controllers;
pub mod models;
use anchor_lang::prelude::*;
declare_id!("9VELjY4xnvMWThciCAuMDZNiDYSCd3341BD7PmU1UxwC");

#[program]
pub mod web3_todo_app {
    pub use super::*;

    use controllers::{
        todo::{
            create_todo as create_todo_controller,
            find_todo_by_id as find_todo_by_id_controller,
            find_todo_by_wallet as find_todo_by_wallet_controller,
            find_todo_is_done as find_todo_is_done_controller,
        },
        user::{ create_user, find_user_by_id, find_user_todos },
    };

    pub use context::todo_contexts::{
        createTodo::CreateTodo,
        findTodoById::findTodoById,
        findTodoByWallet::findTodoByWallet,
        findTodoIsDone::FindTodoIsDone,
        findTodoNotDone::FindTodoNotDone,
    };

}
