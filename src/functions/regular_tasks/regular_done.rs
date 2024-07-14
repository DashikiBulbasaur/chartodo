use super::regular_helpers::*;
use std::io::Write;

pub fn regular_tasks_remove_done(done_remove: Vec<String>) {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if regular_tasks.done.is_empty() {
        return writeln!(writer, "The done list is currently empty.").expect("writeln failed");
    }

    // filter for viable items
    let mut dones_to_remove: Vec<usize> = vec![];
    done_remove.iter().for_each(|item| {
      if item.parse::<usize>().is_ok() 
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= regular_tasks.done.len() 
        {
            dones_to_remove.push(item.parse().unwrap()); 
        }
    });
    drop(done_remove);

    // reverse sort 
    dones_to_remove.sort();
    dones_to_remove.reverse();
    dones_to_remove.dedup();

    // check if user wants to remove all of the items
    if dones_to_remove.len() >= regular_tasks.done.len() {
        return writeln!(writer, "You might as well do cleardone since you want to remove all of the items.").expect("writeln failed");
    }

    // remove each item one by one
    dones_to_remove.iter().for_each(|position| {
        regular_tasks.done.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}

pub fn regular_tasks_not_done(done_to_todo: Vec<String>) {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if regular_tasks.done.is_empty() {
        return writeln!(writer, "The done list is currently empty.").expect("writeln failed");
    }

    // filter for viable items
    let mut dones_to_todos: Vec<usize> = vec![];
    done_to_todo.iter().for_each(|item| {
      if item.parse::<usize>().is_ok() 
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= regular_tasks.done.len() 
        {
            dones_to_todos.push(item.parse().unwrap()); 
        }
    });
    drop(done_to_todo);

    // reverse sort 
    dones_to_todos.sort();
    dones_to_todos.reverse();
    dones_to_todos.dedup();

    // check if user wants to remove all done items to todo
    if dones_to_todos.len() >= regular_tasks.done.len() {
        return writeln!(writer, "You might as well do notdoneall since you want to reverse all done items.").expect("writeln failed");
    }

    // check if todo list would overflow
    if dones_to_todos.len() + regular_tasks.todo.len() > 15 {
        return writeln!(writer, "You want to move too many done items back to todo; doing so would exceed the todo list's length. Try deleting some todo items first.").expect("writeln failed");
    }

    // reverse dones one by one
    dones_to_todos.iter().for_each(|position| {
        regular_tasks.todo.push(regular_tasks.done.get(*position - 1).unwrap().clone());
        regular_tasks.done.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}

pub fn regular_tasks_clear_done() {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if regular_tasks.done.is_empty() {
        return writeln!(writer, "The done list is currently empty.").expect("writeln failed");
    }

    // clear done list
    regular_tasks.done.clear();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}

pub fn regular_tasks_reverse_all_dones() {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if regular_tasks.done.is_empty() {
        return writeln!(writer, "The done list is currently empty.").expect("writeln failed");
    }

    // check if todo list would overflow
    if regular_tasks.done.len() + regular_tasks.todo.len() > 15 {
        return writeln!(writer, "Reversing all done items to todo would exceed the todo list's maximum len. Please remove some todo items first.").expect("writeln failed");
    }

    // reverse all done items
    regular_tasks.done.iter().for_each(|item| regular_tasks.todo.push(item.clone()));
    regular_tasks.done.clear();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);
}