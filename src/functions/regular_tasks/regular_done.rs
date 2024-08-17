use super::regular_helpers::*;
use std::io::Write;

pub fn regular_tasks_remove_done(done_remove: Vec<String>) -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if regular_tasks.done.is_empty() {
        writeln!(
            writer,
            "ERROR: The regular done list is currently empty, so you can't remove any items."
        )
        .expect("writeln failed");

        // error = true
        return true;
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

    // check if all args were invalid
    if dones_to_remove.is_empty() {
        writeln!(writer, "ERROR: None of the positions you gave were valid -- they were all either negatize, zero, or exceeded the regular done list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // reverse sort
    dones_to_remove.sort();
    dones_to_remove.reverse();
    dones_to_remove.dedup();

    // check if user wants to remove all of the items
    if dones_to_remove.len() >= regular_tasks.done.len() && regular_tasks.done.len() > 10 {
        writeln!(writer, "ERROR: You've specified removing the entire regular done list. You should do chartodo cleardone.").expect("writeln failed");

        // error = true
        return true;
    }

    // remove each item one by one
    dones_to_remove.iter().for_each(|position| {
        regular_tasks.done.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_not_done(done_to_todo: Vec<String>) -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if regular_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The regular done list is currently empty, so you can't reverse any items back to todo.").expect("writeln failed");

        // error = true
        return true;
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

    // check if all args were invalid
    if dones_to_todos.is_empty() {
        writeln!(writer, "ERROR: None of the positions you gave were valid -- they were all either negative, zero, or exceeded the regular done list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // reverse sort
    dones_to_todos.sort();
    dones_to_todos.reverse();
    dones_to_todos.dedup();

    // check if user wants to remove all done items to todo
    if dones_to_todos.len() >= regular_tasks.done.len() && regular_tasks.done.len() > 10 {
        writeln!(
            writer,
            "ERROR: you've specified reversing the entire regular done list back to todo. You should do chartodo notdoneall."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // reverse dones one by one
    dones_to_todos.iter().for_each(|position| {
        regular_tasks
            .todo
            .push(regular_tasks.done.get(*position - 1).unwrap().clone());
        regular_tasks.done.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_clear_done() -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if regular_tasks.done.is_empty() {
        writeln!(
            writer,
            "ERROR: The regular done list is currently empty, so you can't remove any items."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // clear done list
    regular_tasks.done.clear();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_reverse_all_dones() -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if regular_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The regular done list is currently empty, so you can't reverse any items back to todo.").expect("writeln failed");

        // error = true
        return true;
    }

    // reverse all done items
    regular_tasks
        .done
        .iter()
        .for_each(|item| regular_tasks.todo.push(item.clone()));
    regular_tasks.done.clear();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}
