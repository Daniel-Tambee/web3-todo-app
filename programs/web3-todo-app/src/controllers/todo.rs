use anchor_lang::{context::Context, solana_program::program_error::ProgramError};

use crate::{
    context::todo_contexts::{
        createTodo::CreateTodo, findTodoById::findTodoById, findTodoByWallet::findTodoByWallet,
        findTodoIsDone::FindTodoIsDone, markToDoDone::MarkTodoDone,
    },
    models::todo::Todo,
};

pub fn create_todo(_ctx: Context<CreateTodo>, _content: String) -> Result<Todo, ProgramError> {
    todo!()
}
pub fn find_todo_by_id(_ctx: Context<findTodoById>, _id: usize) -> Result<Todo, ProgramError> {
    todo!()
}
pub fn find_todo_by_wallet(_ctx: Context<findTodoByWallet>) -> Result<Vec<Todo>, ProgramError> {
    todo!()
}
pub fn find_todo_is_done(_ctx: Context<FindTodoIsDone>) -> Result<Vec<Todo>, ProgramError> {
    todo!()
}
pub fn mark_todo_done(_ctx: Context<MarkTodoDone>, _id: usize) -> Result<Todo, ProgramError> {
    todo!()
}
