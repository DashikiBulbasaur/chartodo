use super::regular_helpers::*;
use crate::functions::json_file_structs::*;
use std::io::Write;

pub fn regular_tasks_add_todo(add_todo: Vec<String>) -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // add todos
    // note that I could compare the len of regular_tasks before and after this operation,
    // but checking using bool is I think simpler and more performant
    let mut tasks_added = false;
    add_todo.iter().for_each(|item| {
        // check if the task is under max allowed len (100)
        if item.as_str().len() <= 100 {
            // a task was added. could I just skip the if and do tasks_added = true and would it matter?
            // note that I don't think it would change much and this might be better
            if !tasks_added {
                tasks_added = true;
            }

            let new_task = Task {
                task: item.to_string(),
                date: None,
                time: None,
                repeat_number: None,
                repeat_unit: None,
                repeat_done: None,
                repeat_original_date: None,
                repeat_original_time: None,
            };
            regular_tasks.todo.push(new_task);
        }
    });

    if !tasks_added {
        writeln!(writer, "ERROR: All of the regular task items you wanted to add exceeded the max character len of 100. This error is just to notify you that none were added. The max-character-len is imposed so that users don't accidentally create infinite-length items. You can open an issue on github and request the max-character-len to be increased.").expect("writeln failed");

        // error = true
        return true;
    }

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_change_todo_to_done(mut todo_to_done: Vec<String>) -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The regular todo list is currently empty so you can't change any todos to done."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable items
    for i in (0..todo_to_done.len()).rev() {
        if todo_to_done.get(i).unwrap().parse::<usize>().is_err()
            || todo_to_done.get(i).unwrap().is_empty() // this will never trigger smh
            || todo_to_done.get(i).unwrap().parse::<usize>().unwrap() == 0
            || todo_to_done.get(i).unwrap().parse::<usize>().unwrap() > regular_tasks.todo.len()
        {
            todo_to_done.swap_remove(i);
        }
    }

    // check if none of the args were valid
    if todo_to_done.is_empty() {
        writeln!(writer, "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the regular todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    todo_to_done.sort();
    todo_to_done.dedup();

    // check if the user basically specified the entire list
    if todo_to_done.len() >= regular_tasks.todo.len() && regular_tasks.todo.len() > 5 {
        writeln!(
            writer,
            "WARNING: you've specified marking the entire regular todo list as done. You should do chartodo doneall."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // if changing todos to done means the done list overflows, clear done list
    if todo_to_done.len() + regular_tasks.done.len() > 30 {
        regular_tasks.done.clear();
    }

    // change todos to dones one by one. no idea if the parse slows down the process significantly
    // rev is done so that removing by position doesn't become invalid
    todo_to_done.iter().rev().for_each(|position| {
        regular_tasks.done.push(
            regular_tasks
                .todo
                .get(position.parse::<usize>().unwrap() - 1)
                .unwrap()
                .to_owned(),
        );
        regular_tasks
            .todo
            .remove(position.parse::<usize>().unwrap() - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_remove_todo(mut todo_to_remove: Vec<String>) -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The regular todo list is currently empty, so you can't remove any items. Try adding to it first before removing any."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable items
    for i in (0..todo_to_remove.len()).rev() {
        if todo_to_remove.get(i).unwrap().parse::<usize>().is_err()
            || todo_to_remove.get(i).unwrap().is_empty() // this will never trigger smh
            || todo_to_remove.get(i).unwrap().parse::<usize>().unwrap() == 0
            || todo_to_remove.get(i).unwrap().parse::<usize>().unwrap() > regular_tasks.todo.len()
        {
            todo_to_remove.swap_remove(i);
        }
    }

    // check if all args were invalid
    if todo_to_remove.is_empty() {
        writeln!(writer, "ERROR: none of the positions you gave were valid -- they were all either negative, zero, or exceeded the regular todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    todo_to_remove.sort();
    todo_to_remove.dedup();

    // check if user wants to remove all of the items
    if todo_to_remove.len() >= regular_tasks.todo.len() && regular_tasks.todo.len() > 5 {
        writeln!(
            writer,
            "WARNING: You specified removing the entire regular todo list. You should instead do chartodo cleartodo."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // remove each item one by one
    todo_to_remove.iter().rev().for_each(|position| {
        regular_tasks
            .todo
            .remove(position.parse::<usize>().unwrap() - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_clear_todo() -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The regular todo list is currently empty. Try adding items to it first before removing any."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // clear todo list
    regular_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_change_all_todo_to_done() -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The regular todo list is currently empty, so you can't change any todos to done."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // clear done list if it will overflow
    if regular_tasks.todo.len() + regular_tasks.done.len() > 30 {
        regular_tasks.done.clear();
    }

    // push all todos to done
    regular_tasks
        .todo
        .iter()
        .for_each(|item| regular_tasks.done.push(item.clone()));
    regular_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_edit_todo(position_and_new: Vec<String>) -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if regular_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The regular todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_and_new.len() != 2 {
        writeln!(writer, "ERROR: You must specify the regular todo's position that will be edited, and what to edit it to. There should be 2 arguments after 'chartodo edit'. You provided {} arguments. A proper example would be: chartodo edit 4 new-item.", position_and_new.len()).expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if position_and_new.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: To edit a regular task item, you must provide a viable position. Try something between 1 and {}",
            regular_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if position_and_new.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if position_and_new.first().unwrap().parse::<usize>().unwrap() > regular_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: The position you specified exceeds the regular todo list's current length. Try something between 1 and {}",
            regular_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // new regular task len must be <= 100
    if position_and_new.last().unwrap().len() > 100 {
        writeln!(writer, "ERROR: Editing a regular task to be longer than 100 characters is not allowed. This is to impose a standard so that users can't accidentally create infinite-length tasks. You can open an issue on github and request for the max-character-len to be increased.").expect("writeln failed");

        // error = true
        return true;
    }

    // edit todo item
    let position: usize = position_and_new.first().unwrap().parse().unwrap();
    // is this unwrap proper. i feel like it is, there should be no instances where it isn't accessible
    regular_tasks.todo.get_mut(position - 1).unwrap().task =
        position_and_new.last().unwrap().to_string();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}
