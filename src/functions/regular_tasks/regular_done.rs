use super::regular_helpers::*;
use std::io::Write;

pub fn regular_tasks_remove_done(mut done_to_remove: Vec<String>) -> bool {
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
    for i in (0..done_to_remove.len()).rev() {
        if done_to_remove.get(i).unwrap().parse::<usize>().is_err()
            || done_to_remove.get(i).unwrap().is_empty() // this will never trigger smh
            || done_to_remove.get(i).unwrap().parse::<usize>().unwrap() == 0
            || done_to_remove.get(i).unwrap().parse::<usize>().unwrap() > regular_tasks.todo.len()
        {
            done_to_remove.swap_remove(i);
        }
    }

    // check if all args were invalid
    if done_to_remove.is_empty() {
        writeln!(writer, "ERROR: None of the positions you gave were valid -- they were all either negatize, zero, or exceeded the regular done list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    done_to_remove.sort();
    done_to_remove.dedup();

    // check if user wants to remove all of the items
    if done_to_remove.len() >= regular_tasks.done.len() && regular_tasks.done.len() > 5 {
        writeln!(writer, "WARNING: You've specified removing the entire regular done list. You should do chartodo cleardone.").expect("writeln failed");

        // error = true
        return true;
    }

    // remove each item one by one
    done_to_remove.iter().rev().for_each(|position| {
        regular_tasks
            .done
            .remove(position.parse::<usize>().unwrap() - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_not_done(mut done_to_todo: Vec<String>) -> bool {
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
    for i in (0..done_to_todo.len()).rev() {
        if done_to_todo.get(i).unwrap().parse::<usize>().is_err()
            || done_to_todo.get(i).unwrap().is_empty() // this will never trigger smh
            || done_to_todo.get(i).unwrap().parse::<usize>().unwrap() == 0
            || done_to_todo.get(i).unwrap().parse::<usize>().unwrap() > regular_tasks.todo.len()
        {
            done_to_todo.swap_remove(i);
        }
    }

    // check if all args were invalid
    if done_to_todo.is_empty() {
        writeln!(writer, "ERROR: None of the positions you gave were valid -- they were all either negative, zero, or exceeded the regular done list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    done_to_todo.sort();
    done_to_todo.dedup();

    // check if user wants to remove all done items to todo
    if done_to_todo.len() >= regular_tasks.done.len() && regular_tasks.done.len() > 5 {
        writeln!(
            writer,
            "WARNING: you've specified reversing the entire regular done list back to todo. You should do chartodo notdoneall."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // reverse dones one by one
    done_to_todo.iter().rev().for_each(|position| {
        regular_tasks.todo.push(
            regular_tasks
                .done
                .get(position.parse::<usize>().unwrap() - 1)
                .unwrap()
                .clone(),
        );
        regular_tasks
            .done
            .remove(position.parse::<usize>().unwrap() - 1);
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
