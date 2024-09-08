use super::deadline_helpers::*;
use std::io::Write;

pub fn deadline_tasks_rmdone(mut done_remove: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if deadline_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The deadline done list is currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable items
    for i in (0..done_remove.len()).rev() {
        if done_remove.get(i).unwrap().parse::<usize>().is_err()
        && done_remove.get(i).unwrap().is_empty() // this will never trigger smh
        && done_remove.get(i).unwrap().parse::<usize>().unwrap() == 0
        && done_remove.get(i).unwrap().parse::<usize>().unwrap() > deadline_tasks.done.len()
        {
            done_remove.swap_remove(i);
        }
    }

    if done_remove.is_empty() {
        writeln!(writer, "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the regular todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    done_remove.sort();
    done_remove.dedup();

    // check if user wants to remove all of the items
    if done_remove.len() >= deadline_tasks.done.len() && deadline_tasks.done.len() > 5 {
        writeln!(
            writer,
            "WARNING: You might as well do deadline-cleardone since you want to remove all of the items."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // remove each item one by one
    done_remove.iter().rev().for_each(|position| {
        deadline_tasks
            .done
            .remove(position.parse::<usize>().unwrap() - 1);
    });

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_not_done(mut not_done: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if deadline_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The deadline done list is currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable items
    for i in (0..not_done.len()).rev() {
        if not_done.get(i).unwrap().parse::<usize>().is_err()
        && not_done.get(i).unwrap().is_empty() // this will never trigger smh
        && not_done.get(i).unwrap().parse::<usize>().unwrap() == 0
        && not_done.get(i).unwrap().parse::<usize>().unwrap() > deadline_tasks.done.len()
        {
            not_done.swap_remove(i);
        }
    }

    if not_done.is_empty() {
        writeln!(writer, "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the regular todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    not_done.sort();
    not_done.dedup();

    // check if user wants to remove all done items to todo
    if not_done.len() >= deadline_tasks.done.len() {
        writeln!(writer, "WARNING: You might as well do deadline-notdoneall since you want to reverse all deadline done items.").expect("writeln failed");

        // error = true
        return true;
    }

    // reverse dones one by one
    not_done.iter().rev().for_each(|position| {
        deadline_tasks.todo.push(
            deadline_tasks
                .done
                .get(position.parse::<usize>().unwrap() - 1)
                .unwrap()
                .to_owned(),
        );
        deadline_tasks
            .done
            .remove(position.parse::<usize>().unwrap() - 1);
    });

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_clear_done() -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if deadline_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The deadline done list is currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
    }

    // clear done list
    deadline_tasks.done.clear();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_notdoneall() -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if deadline_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The deadline done list is currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
    }

    // reverse all done items
    deadline_tasks
        .done
        .iter()
        .for_each(|item| deadline_tasks.todo.push(item.clone()));
    deadline_tasks.done.clear();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}
