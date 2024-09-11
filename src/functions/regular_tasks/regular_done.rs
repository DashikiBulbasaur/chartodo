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
            || done_to_remove.get(i).unwrap().parse::<usize>().unwrap() > regular_tasks.done.len()
        {
            done_to_remove.swap_remove(i);
        }
    }

    // check if all args were invalid
    if done_to_remove.is_empty() {
        writeln!(writer, "ERROR: None of the positions you gave were valid -- they were all either negatize, zero, or exceeded the regular done list's length").expect("writeln failed");

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
            || done_to_todo.get(i).unwrap().parse::<usize>().unwrap() > regular_tasks.done.len()
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

// cargo test regular_done_unit_tests -- --test-threads=1
#[cfg(test)]
mod regular_done_unit_tests {
    use super::*;
    use crate::functions::json_file_structs::*;
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

    #[test]
    fn regular_tasks_remove_done_done_is_empty() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that regular done list is correctly identified as empty
        let arguments = vec![String::from("1")];
        let error_should_be_true = regular_tasks_remove_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_remove_done_no_valid_arguments() {
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

        // check that there are no valid arguments
        let arguments = vec![String::from("2"), String::from("-1"), String::from("0")];
        let error_should_be_true = regular_tasks_remove_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_remove_done_should_do_cleardone() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this is the done list1",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list2",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list3",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list4",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list5",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list6",
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

        // check that user should do notdoneall
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_should_be_true = regular_tasks_remove_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_remove_done_is_correct() {
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

        // perform actions
        let arguments = vec![String::from("1")];
        let error_should_be_false = regular_tasks_remove_done(arguments);
        let read_test_file = open_regular_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let regular_tasks: Tasks = serde_json::from_str(regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, regular_tasks);
    }

    #[test]
    fn regular_tasks_remove_done_multiple_args_is_correct() {
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
                    },
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
                        "task": "hello",
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

        // perform actions. 1 invalid arg + 1 left on purpose
        let arguments = vec![String::from("5"), String::from("1"), String::from("3")];
        let error_should_be_false = regular_tasks_remove_done(arguments);
        let read_test_file = open_regular_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let regular_tasks = r#"
            {
                "todo": [],
                "done": [
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
                ]
            }
        "#;
        let regular_tasks: Tasks = serde_json::from_str(regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, regular_tasks);
    }

    #[test]
    fn regular_tasks_not_done_done_is_empty() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that regular done list is correctly identified as empty
        let arguments = vec![String::from("1")];
        let error_should_be_true = regular_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_not_done_no_valid_arguments() {
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

        // check that there are no valid arguments
        let arguments = vec![String::from("2"), String::from("-1"), String::from("0")];
        let error_should_be_true = regular_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_not_done_should_do_notdoneall() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this is the done list1",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list2",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list3",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list4",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list5",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this is the done list6",
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

        // check that user should do cleardone
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_should_be_true = regular_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_not_done_is_correct() {
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

        // perform actions
        let arguments = vec![String::from("1")];
        let error_should_be_false = regular_tasks_not_done(arguments);
        let read_test_file = open_regular_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let regular_tasks = r#"
            {
                "todo": [
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
                ],
                "done": []
            }
        "#;
        let regular_tasks: Tasks = serde_json::from_str(regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, regular_tasks);
    }

    #[test]
    fn regular_tasks_not_done_multiple_args_is_correct() {
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
                    },
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
                        "task": "hello",
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

        // perform actions. 1 invalid arg + 1 left over
        let arguments = vec![String::from("3"), String::from("-2"), String::from("2")];
        let error_should_be_false = regular_tasks_not_done(arguments);
        let read_test_file = open_regular_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
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
        let regular_tasks: Tasks = serde_json::from_str(regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, regular_tasks);
    }

    #[test]
    fn regular_tasks_cleardone_done_is_empty() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that regular done list is correctly identified as empty
        let error_should_be_true = regular_tasks_clear_done();

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_cleardone_is_correct() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": [
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
                        "task": "hello",
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

        // perform actions
        let error_should_be_false = regular_tasks_clear_done();
        let read_test_file = open_regular_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let regular_tasks: Tasks = serde_json::from_str(regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, regular_tasks);
    }

    #[test]
    fn regular_tasks_notdoneall_done_is_empty() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;

        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // check that regular done list is correctly identified as empty
        let error_should_be_true = regular_tasks_reverse_all_dones();

        assert!(error_should_be_true);
    }

    #[test]
    fn regular_tasks_notdoneall_is_correct() {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": [
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
                        "task": "hello",
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

        // perform actions
        let error_should_be_false = regular_tasks_reverse_all_dones();
        let read_test_file = open_regular_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let regular_tasks = r#"
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
                        "task": "hello",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let regular_tasks: Tasks = serde_json::from_str(regular_tasks).
            context(
                "during testing: the fresh data to put in the new regular_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, regular_tasks);
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
