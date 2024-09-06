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
    let mut index: usize = 0;
    // i can't do an iter for each loop since i can't return from inside a closure
    while index < add_todo.len() {
        if add_todo.get(index).unwrap().len() > 100 {
            writeln!(writer, "ERROR: task '{}' was {} characters long, which is over the task max-character-len of 100. A max-character-len is imposed so that users don't accidentally create extremely-long items. You can open an issue on github and request the max-character-len to be increased.", add_todo.get(index).unwrap(), add_todo.get(index).unwrap().len()).expect("writeln failed");

            // error = true
            return true;
        }

        let new_task = Task {
            task: add_todo.get(index).unwrap().to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: None,
            repeat_original_date: None,
            repeat_original_time: None,
        };
        regular_tasks.todo.push(new_task);

        index += 1;
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
    // rev cuz i want the indices to be viable after swap removing
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
            "WARNING: you've specified marking the entire regular todo list as done. You should do chartodo doneall.")
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
        writeln!(writer, "ERROR: You must specify the regular todo's position that will be edited, and what to edit it to. There should be 2 arguments after 'chartodo edit'. You provided {} argument(s). A proper example would be: chartodo edit 4 new-item.", position_and_new.len()).expect("writeln failed");

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
        writeln!(writer, "ERROR: Editing a regular task to be longer than 100 characters is not allowed. This is to impose a standard so that users can't accidentally create extremely-long tasks. You can open an issue on github and request for the max-character-len to be increased.").expect("writeln failed");

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

// cargo test regular_todo_unit_tests -- --test-threads=1
#[cfg(test)]
mod regular_todo_unit_tests {
    use super::*;
    use anyhow::Context;
    use std::path::PathBuf;

    // these are taken from regular_helpers
    fn path_to_regular_tasks() -> PathBuf {
        // get the data dir XDG spec and return it with path to regular_tasks.json
        let mut regular_tasks_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        regular_tasks_path.push("chartodo/regular_tasks.json");

        regular_tasks_path
    }

    fn regular_tasks_copy_path() -> PathBuf {
        // get the path for regular_tasks_copy.json, which will be used to hold the original contents
        // of regular_tasks.json while it's getting modified
        let mut regular_tasks_copy_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        regular_tasks_copy_path.push("chartodo/regular_tasks_copy.json");

        regular_tasks_copy_path
    }

    // these have been tested in other fns, these are just included here as a sanity check
    #[test]
    fn regular_tasks_path_is_correct() {
        let linux_path = "/.local/share/chartodo/regular_tasks.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/regular_tasks.json";
        let mac_path = "/Library/Application Support/chartodo/regular_tasks.json";
        let mut got_regular_tasks_path: bool = false;
        let regular_path = path_to_regular_tasks();
        let regular_path = regular_path.to_str().unwrap();

        if regular_path.contains(linux_path) {
            got_regular_tasks_path = true;
        } else if regular_path.contains(windows_path) {
            got_regular_tasks_path = true;
        } else if regular_path.contains(mac_path) {
            got_regular_tasks_path = true;
        }

        assert!(got_regular_tasks_path);
    }

    #[test]
    fn regular_tasks_copy_path_is_correct() {
        let linux_path = "/.local/share/chartodo/regular_tasks_copy.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/regular_tasks_copy.json";
        let mac_path = "/Library/Application Support/chartodo/regular_tasks_copy.json";
        let mut got_regular_tasks_copy_path: bool = false;
        let regular_tasks_copy_path = regular_tasks_copy_path();
        let regular_tasks_copy_path = regular_tasks_copy_path.to_str().unwrap();

        if regular_tasks_copy_path.contains(linux_path) {
            got_regular_tasks_copy_path = true;
        } else if regular_tasks_copy_path.contains(windows_path) {
            got_regular_tasks_copy_path = true;
        } else if regular_tasks_copy_path.contains(mac_path) {
            got_regular_tasks_copy_path = true;
        }

        assert!(got_regular_tasks_copy_path);
    }

    // note that i've thought about testing whether or not the copy file was correct
    // it'd be a bit redundant, but it's something I could do as an extra sanity check
    #[test]
    fn aaaa_regular_tasks_clone_file() {
        // name is aaaa so it's done first
        // since we will be modifying the original file to run a test, the original data must be
        // preserved first
        std::fs::File::create(regular_tasks_copy_path())
            .context("failed to create regular_tasks_copy.json")
            .expect("failed to create a copy during unit test");

        std::fs::copy(path_to_regular_tasks(), regular_tasks_copy_path())
            .context("failed to copy regular_tasks.json to regular_tasks_copy.json")
            .expect("failed to copy original file to copy file during unit test");
    }

    // note that I won't test for successful cases, aka error false, since they print to terminal and modify files as a result of user input,
    // and those can be largely captured by integration tests
    // it's easy to bool check unsuccessful cases since i don't have to also check for modified files afterwards
    // anything that could be checked for correct printing, for cases successful and unsuccessful, can also be done by integration testing

    // successful cases, like unsuccessful cases, can also be bool tested, but i don't wanna do that
    // i'm too lazy
    // i could also check for correct printing for unsuccessful cases here. maybe, maybe not
    #[test]
    fn adding_regular_tasks_todo_is_correct() {
        // this is 101 characters long
        let arguments: Vec<String> = vec![String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")];
        let error_should_be_true = regular_tasks_add_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_adding_fails_correctly() {
        // this is 101 characters long
        let arguments: Vec<String> = vec![String::from("a"), String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), String::from("aa")];
        let error_should_be_true = regular_tasks_add_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_todo_to_done_regular_todo_is_empty() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that regular todo list is correctly identified as empty
        let arguments = vec![String::from("1")];
        let error_should_be_true = regular_tasks_change_todo_to_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_todo_to_done_no_valid_arguments() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this is the todo list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that invalid arguments are in fact invalid
        let arguments = vec![String::from("-1"), String::from("0"), String::from("2")];
        let error_should_be_true = regular_tasks_change_todo_to_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_todo_to_done_should_do_doneall() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "poopy",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "poopy2",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "poopy3",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "poopy4",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "poopy5",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "poopy6",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
            "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that user should in fact do chartodo doneall
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_should_be_true = regular_tasks_change_todo_to_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_remove_todo_regular_todo_is_empty() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that regular todo list is correctly identified as empty
        let arguments = vec![String::from("1")];
        let error_should_be_true = regular_tasks_remove_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_remove_todo_no_valid_arguments() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this is the todo list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that invalid arguments are in fact invalid
        let arguments = vec![String::from("-1"), String::from("0"), String::from("2")];
        let error_should_be_true = regular_tasks_remove_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_remove_todo_should_do_cleartodo() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi2",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },

                    {
                        "task": "hi3",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                
                    {
                        "task": "hi4",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                
                    {
                        "task": "hi5",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi6",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
            "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that user should do cleartodo
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_should_be_true = regular_tasks_remove_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_cleartodo_regular_todo_is_empty() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that regular todo list is correctly identified as empty
        let error_should_be_true = regular_tasks_clear_todo();

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_doneall_regular_todo_is_empty() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that regular todo list is correctly identified as empty
        let error_should_be_true = regular_tasks_change_all_todo_to_done();

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_edit_todo_regular_todo_is_empty() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        let arguments = vec![String::from("1")];

        // check that regular todo list is correctly identified as empty
        let error_should_be_true = regular_tasks_edit_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_edit_todo_wrong_num_of_args() {
        let arguments = vec![String::from("1")];

        // check that a wrong # of args is identified
        let error_should_be_true = regular_tasks_edit_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_edit_todo_position_not_a_num() {
        // the 2nd arg could be invalid, the first arg's invalid state will always be caught first
        let arguments = vec![String::from("a"), String::from("new_task")];

        // check that a wrong # of args is identified
        let error_should_be_true = regular_tasks_edit_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_edit_todo_position_is_zero() {
        // write fresh to regular tasks so content is known
        // not needed to write anything here, but since chronologically/alphabetically the last test
        // wrote an empty todo list, running this with the same content
        // technically registers the wrong error.
        // after this, chronologically/alphabetically, only some edit todo tests are solely concerned with args
        // need writing
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        let arguments = vec![String::from("0"), String::from("new_task")];

        // catch that the position is zero
        let error_should_be_true = regular_tasks_edit_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_edit_todo_position_not_within_len() {
        let arguments = vec![String::from("2"), String::from("new_task")];

        // catch that the position is more than regular todo's len
        let error_should_be_true = regular_tasks_edit_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_edit_todo_task_is_more_than_100_chars() {
        // chronologically/alphabetically, the test before this writes an empty todo list so that needs
        // to be overwritten so that the wrong error isn't triggered
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": [
                    {
                        "task": "this is the done list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        let arguments = vec![String::from("1"), String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")];

        // catch that the task is more than 100 chars long
        let error_should_be_true = regular_tasks_edit_todo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn zzzz_rename_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, remove the modified original and rename copy to original

        std::fs::remove_file(path_to_regular_tasks())
            .context("failed delete modified regular_tasks.json after running tests")
            .expect("failed to delete regular_tasks.json after regular_helpers unit tests");

        std::fs::rename(regular_tasks_copy_path(), path_to_regular_tasks())
            .context("failed to rename regular_tasks_copy to regular_tasks")
            .expect("failed to rename regular_tasks_copy to regular_tasks after tests were done");
    }
}
