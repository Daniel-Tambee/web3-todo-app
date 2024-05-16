use std::borrow::{ Borrow, BorrowMut };

use anchor_lang::{
    context::Context,
    solana_program::program_error::ProgramError,
    Key,
    ToAccountInfo,
};

use crate::{
    context::todo_contexts::{
        createTodo::CreateTodo,
        findTodoById::findTodoById,
        findTodoByWallet::findTodoByWallet,
        findTodoIsDone::FindTodoIsDone,
        findTodoNotDone::FindTodoNotDone,
        markToDoDone::MarkTodoDone,
    },
    models::todo::Todo,
};
pub fn create_todo(_ctx: Context<CreateTodo>, _content: String) -> Result<(), ProgramError> {
    let todo: &mut anchor_lang::prelude::Account<Todo> = _ctx.accounts.todo.as_mut();
    let user: &mut anchor_lang::prelude::Account<crate::models::user::User> = _ctx.accounts.user.as_mut();
    todo.owner = user.authority.key();
    todo.id += user.todos.len() + 1;
    todo.content = _content;
    todo.is_done = false;
    user.todos.push(Todo {
        owner: todo.owner,
        id: todo.id,
        content: todo.content.clone(),
        is_done: todo.is_done,
    });
    Ok(())
}
pub fn find_todo_by_id(_ctx: Context<findTodoById>, _id: usize) -> Result<(), ProgramError> {
    let user: &anchor_lang::prelude::Account<crate::models::user::User> = _ctx.accounts.user.borrow();
    let todo = for item in user.todos.iter() {
        if item.id == _id {
            item.clone();
        }
    };

    Ok(todo)
}
pub fn find_todo_by_wallet(_ctx: Context<findTodoByWallet>) -> Result<Vec<Todo>, ProgramError> {
    let user: &anchor_lang::prelude::Account<crate::models::user::User> = _ctx.accounts.user.borrow();
    let mut result: Vec<Todo> = vec![];
    for todo in user.todos.iter() {
        if todo.owner == user.authority {
            result.push(todo.clone());
        }
    }
    Ok(result)
}
pub fn find_todo_is_done(_ctx: Context<FindTodoIsDone>) -> Result<Vec<Todo>, ProgramError> {
    let user: &anchor_lang::prelude::Account<crate::models::user::User> = _ctx.accounts.user.borrow();
    let mut result: Vec<Todo> = vec![];
    for todo in user.todos.iter() {
        if todo.is_done == true {
            result.push(todo.clone());
        }
    }
    Ok(result)
}
pub fn find_todo_not_done(_ctx: Context<FindTodoNotDone>) -> Result<Vec<Todo>, ProgramError> {
    let user: &anchor_lang::prelude::Account<crate::models::user::User> = _ctx.accounts.user.borrow();
    let mut result: Vec<Todo> = vec![];
    for todo in user.todos.iter() {
        if todo.is_done == false {
            result.push(todo.clone());
        }
    }
    Ok(result)
}
pub fn mark_todo_done(_ctx: Context<MarkTodoDone>, _id: usize) -> Result<(), ProgramError> {
    let user: &mut anchor_lang::prelude::Account<crate::models::user::User> = _ctx.accounts.user.borrow_mut();
    let todo: &mut anchor_lang::prelude::Account<Todo> = _ctx.accounts.todo.borrow_mut();
    todo.is_done = true;
    for todo in user.todos.iter_mut() {
        if todo.id == _id {
            todo.is_done = true;
        }
    }
    Ok(())
}
