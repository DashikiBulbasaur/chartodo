use super::repeating_helpers::*;
use std::io::Write;

pub fn repeating_tasks_not_done(mut not_done: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if repeating_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The repeating done list is currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable items
    for i in (0..not_done.len()).rev() {
        if not_done.get(i).unwrap().parse::<usize>().is_err()
        || not_done.get(i).unwrap().is_empty() // this will never trigger smh
        || not_done.get(i).unwrap().parse::<usize>().unwrap() == 0
        || not_done.get(i).unwrap().parse::<usize>().unwrap() > repeating_tasks.done.len()
        {
            not_done.swap_remove(i);
        }
    }

    // no valid args
    if not_done.is_empty() {
        writeln!(writer, "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    not_done.sort();
    not_done.dedup();

    // check if user wants to move all done items to todo
    if not_done.len() >= repeating_tasks.done.len() && repeating_tasks.done.len() > 5 {
        writeln!(writer, "WARNING: You specified an entire done list that's relatively long. You should do repeating-notdoneall.").expect("writeln failed");

        // error = true
        return true;
    }

    // before pushing to todo, change each repeat_done field in each specified done to false
    not_done.iter().for_each(|position| {
        repeating_tasks
            .done
            .get_mut(position.parse::<usize>().unwrap() - 1)
            .unwrap()
            .repeat_done = Some(false);
    });

    // reverse dones one by one
    not_done.iter().rev().for_each(|position| {
        repeating_tasks.todo.push(
            repeating_tasks
                .done
                .get(position.parse::<usize>().unwrap() - 1)
                .unwrap()
                .clone(),
        );
        repeating_tasks
            .done
            .remove(position.parse::<usize>().unwrap() - 1);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

pub fn repeating_tasks_rmdone(mut done_remove: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if repeating_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The repeating done list is currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable items
    for i in (0..done_remove.len()).rev() {
        if done_remove.get(i).unwrap().parse::<usize>().is_err()
        || done_remove.get(i).unwrap().is_empty() // this will never trigger smh
        || done_remove.get(i).unwrap().parse::<usize>().unwrap() == 0
        || done_remove.get(i).unwrap().parse::<usize>().unwrap() > repeating_tasks.done.len()
        {
            done_remove.swap_remove(i);
        }
    }

    // no valid args
    if done_remove.is_empty() {
        writeln!(writer, "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    done_remove.sort();
    done_remove.dedup();

    // check if user wants to remove all of the items
    if done_remove.len() >= repeating_tasks.done.len() && repeating_tasks.done.len() > 5 {
        writeln!(
            writer,
            "WARNING: You want to remove all of the finished tasks in a relatively long repeating done list. You should do repeating-cleardone."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // remove each item one by one
    done_remove.iter().rev().for_each(|position| {
        repeating_tasks
            .done
            .remove(position.parse::<usize>().unwrap() - 1);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

pub fn repeating_tasks_not_done_all() -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if repeating_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The repeating done list is currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
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

    // error = false
    false
}

pub fn repeating_tasks_clear_done() -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if repeating_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The repeating done list is currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
    }

    // clear done list
    repeating_tasks.done.clear();

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

// cargo test repeating_done_unit_tests -- --test-threads=1
#[cfg(test)]
mod repeating_done_unit_tests {
    use super::*;
    use crate::functions::json_file_structs::*;
    use anyhow::Context;
    use std::path::PathBuf;

    // these are taken from repeating_helpers
    fn path_to_repeating_tasks() -> PathBuf {
        // get the data dir XDG spec and return it with path to repeating_tasks.json
        let mut repeating_tasks_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        repeating_tasks_path.push("chartodo/repeating_tasks.json");

        repeating_tasks_path
    }

    fn repeating_tasks_copy_path() -> PathBuf {
        // get the path for repeating_tasks_copy.json, which will be used to hold the original contents
        // of repeating_tasks.json while it's getting modified
        let mut repeating_tasks_copy_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        repeating_tasks_copy_path.push("chartodo/repeating_tasks_copy.json");

        repeating_tasks_copy_path
    }

    // these have been tested in other fns, these are just included here as a sanity check
    #[test]
    fn repeating_tasks_path_is_correct() {
        let linux_path = "/.local/share/chartodo/repeating_tasks.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/repeating_tasks.json";
        let mac_path = "/Library/Application Support/chartodo/repeating_tasks.json";
        let mut got_repeating_tasks_path: bool = false;
        let repeating_path = path_to_repeating_tasks();
        let repeating_path = repeating_path.to_str().unwrap();

        if repeating_path.contains(linux_path) {
            got_repeating_tasks_path = true;
        } else if repeating_path.contains(windows_path) {
            got_repeating_tasks_path = true;
        } else if repeating_path.contains(mac_path) {
            got_repeating_tasks_path = true;
        }

        assert!(got_repeating_tasks_path);
    }

    #[test]
    fn repeating_tasks_copy_path_is_correct() {
        let linux_path = "/.local/share/chartodo/repeating_tasks_copy.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/repeating_tasks_copy.json";
        let mac_path = "/Library/Application Support/chartodo/repeating_tasks_copy.json";
        let mut got_repeating_tasks_copy_path: bool = false;
        let repeating_tasks_copy_path = repeating_tasks_copy_path();
        let repeating_tasks_copy_path = repeating_tasks_copy_path.to_str().unwrap();

        if repeating_tasks_copy_path.contains(linux_path) {
            got_repeating_tasks_copy_path = true;
        } else if repeating_tasks_copy_path.contains(windows_path) {
            got_repeating_tasks_copy_path = true;
        } else if repeating_tasks_copy_path.contains(mac_path) {
            got_repeating_tasks_copy_path = true;
        }

        assert!(got_repeating_tasks_copy_path);
    }

    #[test]
    fn aaaa_repeating_tasks_clone_file() {
        // name is aaaa so it's done first
        // since we will be modifying the original file to run a test, the original data must be
        // preserved first
        std::fs::File::create(repeating_tasks_copy_path())
            .context("failed to create repeating_tasks_copy.json")
            .expect("failed to create a copy during unit test");

        std::fs::copy(path_to_repeating_tasks(), repeating_tasks_copy_path())
            .context("failed to copy repeating_tasks.json to repeating_tasks_copy.json")
            .expect("failed to copy original file to copy file during unit test");
    }

    #[test]
    fn repeating_tasks_notdone_done_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = repeating_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_notdone_no_valid_args() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that there are no valid args
        let arguments = vec![String::from("-11"), String::from("0"), String::from("2")];
        let error_should_be_true = repeating_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_notdone_should_do_notdoneall() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that there are no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
            String::from("1"),
        ];
        let error_should_be_true = repeating_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_notdone_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": true,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![String::from("1"), String::from("2")];
        let error_should_be_false = repeating_tasks_not_done(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_notdone_multiple_args_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("4"),
            String::from("1"),
        ];
        let error_should_be_false = repeating_tasks_not_done(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_rmdone_done_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = repeating_tasks_rmdone(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_rmdone_no_valid_args() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that there are no valid args
        let arguments = vec![String::from("-11"), String::from("0"), String::from("2")];
        let error_should_be_true = repeating_tasks_rmdone(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_rmdone_should_do_cleardone() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that there are no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
            String::from("1"),
        ];
        let error_should_be_true = repeating_tasks_rmdone(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_rmdone_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": true,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![String::from("1"), String::from("2")];
        let error_should_be_false = repeating_tasks_rmdone(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_rmdone_multiple_args_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("4"),
            String::from("1"),
        ];
        let error_should_be_false = repeating_tasks_rmdone(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_notdoneall_done_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let error_should_be_true = repeating_tasks_not_done_all();

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_notdoneall_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let error_should_be_false = repeating_tasks_not_done_all();
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_cleardone_done_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let error_should_be_true = repeating_tasks_clear_done();

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_cleardone_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let error_should_be_false = repeating_tasks_clear_done();
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn zzzz_rename_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, remove the modified original and rename copy to original

        std::fs::remove_file(path_to_repeating_tasks())
            .context("failed delete modified repeating_tasks.json after running tests")
            .expect("failed to delete repeating_tasks.json after repeating_helpers unit tests");

        std::fs::rename(repeating_tasks_copy_path(), path_to_repeating_tasks())
            .context("failed to rename repeating_tasks_copy to repeating_tasks")
            .expect(
                "failed to rename repeating_tasks_copy to repeating_tasks after tests were done",
            );
    }
}
