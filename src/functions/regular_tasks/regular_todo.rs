use super::regular_helpers::*;
use crate::functions::json_file_structs::*;
use std::io::Write;

pub fn regular_tasks_add_todo(add_todo: Vec<String>) {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // filter for viable items only
    let mut add_todos = vec![];
    add_todo.iter().for_each(|item| {
        if item.len() <= 40 {
            add_todos.push(item.to_string());
        }
    });
    drop(add_todo);

    // check if user wants to add too many todo items
    if add_todos.len() + regular_tasks.todo.len() >= 15 {
        return writeln!(writer, "You want to add too many todo items. The maximum length of the todo list is only 15. With the current length of the todo list, please only {} or less", 15 - regular_tasks.todo.len()).expect("writeln failed");
    }

    // add todos
    add_todos.iter().for_each(|item| {
        let item = Task {
            task: item.to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: None,
        };
        regular_tasks.todo.push(item);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}

pub fn regular_tasks_change_todo_to_done(todo_to_done: Vec<String>) {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // filter for viable items
    let mut todos_to_dones: Vec<usize> = vec![];
    todo_to_done.iter().for_each(|item| {
      if item.parse::<usize>().is_ok() 
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() < regular_tasks.todo.len() 
        {
            todos_to_dones.push(item.parse().unwrap()); 
        }
    });
    drop(todo_to_done);

    // reverse sort the positions
    todos_to_dones.sort();
    todos_to_dones.reverse();
    todos_to_dones.dedup();

    // check if the user basically specified the entire list
    if todos_to_dones.len() >= regular_tasks.todo.len() {
        return writeln!(writer, "You've specified the entire list. Might as well do chartodo doneall").expect("writeln failed");
    }

    // if changing todos to done means the done list overflows, clear done list
    if todos_to_dones.len() + regular_tasks.done.len() > 10 {
        regular_tasks.done.clear();
    }

    // change todos to dones one by one
    todos_to_dones.iter().for_each(|position| {
        regular_tasks.done.push(regular_tasks.todo.get(*position).unwrap().clone());
        regular_tasks.todo.remove(*position);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}
