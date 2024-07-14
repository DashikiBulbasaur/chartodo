use super::deadline_helpers::*;
use std::io::Write;

pub fn deadline_tasks_rmdone(done_remove: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if deadline_tasks.done.is_empty() {
        return writeln!(writer, "The deadline done list is currently empty.")
            .expect("writeln failed");
    }

    // filter for viable items
    let mut dones_to_remove: Vec<usize> = vec![];
    done_remove.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= deadline_tasks.done.len()
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
    if dones_to_remove.len() >= deadline_tasks.done.len() {
        return writeln!(
            writer,
            "You might as well do deadline-cleardone since you want to remove all of the items."
        )
        .expect("writeln failed");
    }

    // remove each item one by one
    dones_to_remove.iter().for_each(|position| {
        deadline_tasks.done.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_not_done(not_done: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if deadline_tasks.done.is_empty() {
        return writeln!(writer, "The deadline done list is currently empty.")
            .expect("writeln failed");
    }

    // filter for viable items
    let mut not_dones: Vec<usize> = vec![];
    not_done.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= deadline_tasks.done.len()
        {
            not_dones.push(item.parse().unwrap());
        }
    });
    drop(not_done);

    // reverse sort
    not_dones.sort();
    not_dones.reverse();
    not_dones.dedup();

    // check if user wants to remove all done items to todo
    if not_dones.len() >= deadline_tasks.done.len() {
        return writeln!(writer, "You might as well do deadline-notdoneall since you want to reverse all deadline done items.").expect("writeln failed");
    }

    // check if todo list would overflow
    if not_dones.len() + deadline_tasks.todo.len() > 15 {
        return writeln!(writer, "You want to move too many deadline done items back to deadline todo; doing so would exceed the deadline todo list's length. Try deleting some deadline todo items first.").expect("writeln failed");
    }

    // reverse dones one by one
    not_dones.iter().for_each(|position| {
        deadline_tasks
            .todo
            .push(deadline_tasks.done.get(*position - 1).unwrap().clone());
        deadline_tasks.done.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_clear_done() {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if deadline_tasks.done.is_empty() {
        return writeln!(writer, "The deadline done list is currently empty.")
            .expect("writeln failed");
    }

    // clear done list
    deadline_tasks.done.clear();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_notdoneall() {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if deadline_tasks.done.is_empty() {
        return writeln!(writer, "The deadline done list is currently empty.")
            .expect("writeln failed");
    }

    // check if todo list would overflow
    if deadline_tasks.done.len() + deadline_tasks.todo.len() > 15 {
        return writeln!(writer, "Reversing all deadline done items to todo would exceed the deadline todo list's maximum len. Please remove some deadline todo items first.").expect("writeln failed");
    }

    // reverse all done items
    deadline_tasks
        .done
        .iter()
        .for_each(|item| deadline_tasks.todo.push(item.clone()));
    deadline_tasks.done.clear();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}
