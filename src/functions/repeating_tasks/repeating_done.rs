use super::repeating_helpers::*;
use std::io::Write;

pub fn repeating_tasks_not_done(not_done: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if repeating_tasks.done.is_empty() {
        return writeln!(writer, "ERROR: The repeating done list is currently empty.")
            .expect("writeln failed");
    }

    // filter for viable items
    let mut not_dones: Vec<usize> = vec![];
    not_done.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= repeating_tasks.done.len()
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
    if not_dones.len() >= repeating_tasks.done.len() && repeating_tasks.done.len() > 1 {
        return writeln!(writer, "ERROR: You might as well do repeating-notdoneall since you want to reverse all deadline done items.").expect("writeln failed");
    }

    // check if todo list would overflow
    if not_dones.len() + repeating_tasks.todo.len() > 15 {
        return writeln!(writer, "ERROR: You want to move too many repeating done items back to repeating todo; doing so would exceed the repeating todo list's length. Try deleting some repeating todo items first.").expect("writeln failed");
    }

    // before pushing to todo, change each repeat_done field in each specified done to false
    not_dones.iter().for_each(|position| {
        repeating_tasks
            .done
            .get_mut(*position - 1)
            .unwrap()
            .repeat_done = Some(false);
    });

    // reverse dones one by one
    not_dones.iter().for_each(|position| {
        repeating_tasks
            .todo
            .push(repeating_tasks.done.get(*position - 1).unwrap().clone());
        repeating_tasks.done.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

// TODO: for each error message that pops up when the user has specified the entire list without doing ...doneall,
// change it to specify that the len and how much they've specified are below 5.
// reasoning: annoying when trying to do a command and the program says to do ...doneall when there's only one item in the todo/done list

pub fn repeating_tasks_rmdone(done_remove: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if repeating_tasks.done.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating done list is currently empty.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // filter for viable items
    let mut dones_remove: Vec<usize> = vec![];
    done_remove.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= repeating_tasks.done.len()
        {
            dones_remove.push(item.parse().unwrap());
        }
    });
    drop(done_remove);

    // reverse sort
    dones_remove.sort();
    dones_remove.reverse();
    dones_remove.dedup();

    // check if user wants to remove all of the items
    if dones_remove.len() >= repeating_tasks.done.len() && repeating_tasks.done.len() > 5 {
        return writeln!(
            writer,
            "ERROR: You want to remove all of the finished tasks in a relatively long repeating done list. You might as well do repeating-cleardone.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // remove each item one by one
    dones_remove.iter().for_each(|position| {
        repeating_tasks.done.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_not_done_all() {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if repeating_tasks.done.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating done list is currently empty.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // check if todo list would overflow
    if repeating_tasks.done.len() + repeating_tasks.todo.len() > 15 {
        return writeln!(writer, "ERROR: Reversing all deadline done items to todo would exceed the deadline todo list's maximum len. Please remove some deadline todo items first.
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // before pushing, change each repeat_done field to false
    repeating_tasks.done.iter_mut().for_each(|task| {
        task.repeat_done = Some(false);
    });

    // reverse all done items
    repeating_tasks
        .done
        .iter()
        .for_each(|item| repeating_tasks.todo.push(item.clone()));
    repeating_tasks.done.clear();

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_clear_done() {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if repeating_tasks.done.is_empty() {
        return writeln!(
            writer,
            "ERROR: The deadline done list is currently empty.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // clear done list
    repeating_tasks.done.clear();

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}
