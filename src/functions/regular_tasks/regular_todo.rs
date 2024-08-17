use super::regular_helpers::*;
use crate::functions::json_file_structs::*;
use std::io::Write;

pub fn regular_tasks_add_todo(add_todo: Vec<String>) -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // filter for viable items only
    let mut add_todos = vec![];
    add_todo.iter().for_each(|item| {
        if item.len() <= 100 {
            add_todos.push(item.to_string());
        }
    });
    drop(add_todo);

    // check if all args were invalid and notify user
    if add_todos.is_empty() {
        writeln!(writer, "ERROR: All of the regular task items you wanted to add exceeded the max character len of 100. This error is just to notify you that none were added. The max-character-len is imposed so that users don't accidentally create infinite-length items. You can open an issue on github and request the max-character-len to be increased.").expect("writeln failed");

        // error = true
        return true;
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
            repeat_original_date: None,
            repeat_original_time: None,
        };
        regular_tasks.todo.push(item);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_change_todo_to_done(todo_to_done: Vec<String>) -> bool {
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

    // check if none of the args were valid
    if todos_to_dones.is_empty() {
        writeln!(writer, "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the regular todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // reverse sort the positions
    todos_to_dones.sort();
    todos_to_dones.reverse();
    todos_to_dones.dedup();

    // check if the user basically specified the entire list
    if todos_to_dones.len() >= regular_tasks.todo.len() && regular_tasks.todo.len() > 5 {
        writeln!(
            writer,
            "ERROR: you've specified marking the entire regular todo list as done. You should do chartodo doneall."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // if changing todos to done means the done list overflows, clear done list
    if todos_to_dones.len() + regular_tasks.done.len() > 30 {
        regular_tasks.done.clear();
    }

    // change todos to dones one by one
    todos_to_dones.iter().for_each(|position| {
        regular_tasks
            .done
            .push(regular_tasks.todo.get(*position - 1).unwrap().to_owned());
        regular_tasks.todo.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn regular_tasks_remove_todo(todo_to_remove: Vec<String>) -> bool {
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

    // check if all args were invalid
    if todos_to_remove.is_empty() {
        writeln!(writer, "ERROR: none of the positions you gave were valid -- they were all either negative, zero, or exceeded the regular todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // reverse sort
    todos_to_remove.sort();
    todos_to_remove.reverse();
    todos_to_remove.dedup();

    // check if user wants to remove all of the items
    if todos_to_remove.len() >= regular_tasks.todo.len() && regular_tasks.todo.len() > 5 {
        writeln!(
            writer,
            "ERROR: You specified removing the entire regular todo list. You should instead do chartodo cleartodo."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // remove each item one by one
    todos_to_remove.iter().for_each(|position| {
        regular_tasks.todo.remove(*position - 1);
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
