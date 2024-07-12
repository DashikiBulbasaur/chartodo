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
        return writeln!(writer, "You want to add too many todo items. The maximum length of the todo list is only 15. With the current length of the todo list, please only add {} or less", 15 - regular_tasks.todo.len()).expect("writeln failed");
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

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        return writeln!(writer, "The todo list is currently empty so you can't change any todos to done.").expect("writeln failed");
    }

    // filter for viable items
    let mut todos_to_dones: Vec<usize> = vec![];
    todo_to_done.iter().for_each(|item| {
      if item.parse::<usize>().is_ok() 
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= regular_tasks.todo.len() 
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
        regular_tasks.done.push(regular_tasks.todo.get(*position - 1).unwrap().to_owned());
        regular_tasks.todo.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}

pub fn regular_tasks_remove_todo(todo_to_remove: Vec<String>) {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        return writeln!(writer, "The todo list is currently empty. Try adding items to it first before removing any.").expect("writeln failed");
    }

    // filter for viable items
    let mut todos_to_remove: Vec<usize> = vec![];
    todo_to_remove.iter().for_each(|item| {
      if item.parse::<usize>().is_ok() 
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= regular_tasks.todo.len() 
        {
            todos_to_remove.push(item.parse().unwrap()); 
        }
    });
    drop(todo_to_remove);

    // reverse sort 
    todos_to_remove.sort();
    todos_to_remove.reverse();
    todos_to_remove.dedup();

    // check if user wants to remove all of the items
    if todos_to_remove.len() >= regular_tasks.todo.len() {
        return writeln!(writer, "You might as well do cleartodo since you want to remove all of the items.").expect("writeln failed");
    }

    // remove each item one by one
    todos_to_remove.iter().for_each(|position| {
        regular_tasks.todo.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}

pub fn regular_tasks_clear_todo() {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        return writeln!(writer, "The todo list is currently empty. Try adding items to it first before removing any.").expect("writeln failed");
    }

    // clear todo list
    regular_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}

pub fn regular_tasks_change_all_todo_to_done() {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        return writeln!(writer, "The todo list is currently empty, so you can't change any todos to done.").expect("writeln failed");
    }

    // clear done list if it will overflow
    if regular_tasks.todo.len() + regular_tasks.done.len() > 10 {
        regular_tasks.done.clear();
    } 

    // push all todos to done
    regular_tasks.todo.iter().for_each(|item| regular_tasks.done.push(item.clone()));
    regular_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}

pub fn regular_tasks_edit_todo(position_and_new: Vec<String>) {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        return writeln!(writer, "The todo list is currently empty, so there are no todos that can be edited.").expect("writeln failed");
    }

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_and_new.len() != 2 {
        return writeln!(writer, "You must specify the todo's position that will be edited, and what to edit it to. A proper example would be: chartodo edit 4 new-item.").expect("writeln failed");
    }

    // check if position is a valid number
    if position_and_new.first().unwrap().parse::<usize>().is_err() {
        return writeln!(writer, "You must provide a viable position. Try something between 1 and {}", regular_tasks.todo.len()).expect("writeln failed");
    }

    // positions can't be zero
    if position_and_new.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(writer, "Positions can't be zero. They have to be 1 and above.").expect("writeln failed");
    }

    // position not in range of todo list len
    if position_and_new.first().unwrap().parse::<usize>().unwrap() > regular_tasks.todo.len() {
        return writeln!(writer, "Your position exceed's the todo list's length. Try something between 1 and {}", regular_tasks.todo.len()).expect("writeln failed");
    }

    // new item can't be more than 40 chars
    if position_and_new.last().unwrap().len() > 40 {
        return writeln!(writer, "Editing a todo item to be more than 40 characters is not allowed").expect("writeln failed");
    }

    // edit todo item
    let position: usize = position_and_new.first().unwrap().parse().unwrap();
    let new_task: Task = Task {
        task: position_and_new.last().unwrap().to_string(),
        date: None,
        time: None,
        repeat_number: None,
        repeat_unit: None,
        repeat_done: None,
    };
    regular_tasks.todo[position - 1].clone_from(&new_task);
    drop(new_task);

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}
