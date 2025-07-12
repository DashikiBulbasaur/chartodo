use super::deadline_helpers::*;
use crate::functions::{filtering::*, validations::*};

pub fn deadline_tasks_rmdone(done_remove: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if validate_empty_task_list(
        deadline_tasks.done.is_empty(),
        TodoOrDone::Done,
        TaskType::Deadline,
    ) {
        // error = true
        return true;
    }

    // filter for ranged positioning, if any
    let done_remove = ranged_positioning_filter(done_remove, deadline_tasks.done.len());

    // filter for viable items
    let done_remove = positioning_filter(done_remove, deadline_tasks.done.len());

    // check if no valid args
    if validate_valid_args(done_remove.is_empty(), TodoOrDone::Done, TaskType::Deadline) {
        // error = true
        return true;
    }

    // sort and dedup
    let done_remove = sort_and_dedup(done_remove);

    // check if user wants to remove all of the items
    if validate_should_do_all_equivalent(
        done_remove.len(),
        deadline_tasks.done.len(),
        TodoOrDone::Done,
        TaskType::Deadline,
        PositionCommands::RmDone,
    ) {
        // error = true
        return true;
    }

    // remove each item one by one
    done_remove.iter().rev().for_each(|position| {
        deadline_tasks.done.remove(position - 1);
    });

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_not_done(not_done: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if validate_empty_task_list(
        deadline_tasks.done.is_empty(),
        TodoOrDone::Done,
        TaskType::Deadline,
    ) {
        // error = true
        return true;
    }

    // filter for ranged positioning, if any
    let not_done = ranged_positioning_filter(not_done, deadline_tasks.done.len());

    // filter for viable items
    let not_done = positioning_filter(not_done, deadline_tasks.done.len());

    // check for no valid args
    if validate_valid_args(not_done.is_empty(), TodoOrDone::Done, TaskType::Deadline) {
        // error = true
        return true;
    }

    // sort and dedup
    let not_done = sort_and_dedup(not_done);

    // check if user wants to remove all done items to todo
    if validate_should_do_all_equivalent(
        not_done.len(),
        deadline_tasks.done.len(),
        TodoOrDone::Done,
        TaskType::Deadline,
        PositionCommands::NotDone,
    ) {
        // error = true
        return true;
    }

    // reverse dones one by one
    not_done.iter().rev().for_each(|position| {
        deadline_tasks
            .todo
            .push(deadline_tasks.done.get(position - 1).unwrap().to_owned());
        deadline_tasks.done.remove(position - 1);
    });

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_clear_done() -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if validate_empty_task_list(
        deadline_tasks.done.is_empty(),
        TodoOrDone::Done,
        TaskType::Deadline,
    ) {
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

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if the done list is empty
    if validate_empty_task_list(
        deadline_tasks.done.is_empty(),
        TodoOrDone::Done,
        TaskType::Deadline,
    ) {
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

// cargo test deadline_done_unit_tests -- --test-threads=1
#[cfg(test)]
mod deadline_done_unit_tests {
    use super::*;
    use crate::functions::json_file_structs::*;
    use anyhow::Context;
    use std::path::PathBuf;

    // these are taken from deadline_helpers
    fn path_to_deadline_tasks() -> PathBuf {
        // get the data dir XDG spec and return it with path to deadline_tasks.json
        let mut deadline_tasks_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        deadline_tasks_path.push("chartodo/deadline_tasks.json");

        deadline_tasks_path
    }

    fn deadline_tasks_copy_path() -> PathBuf {
        // get the path for deadline_tasks_copy.json, which will be used to hold the original contents
        // of deadline_tasks.json while it's getting modified
        let mut deadline_tasks_copy_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        deadline_tasks_copy_path.push("chartodo/deadline_tasks_copy.json");

        deadline_tasks_copy_path
    }

    // these have been tested in other fns, these are just included here as a sanity check
    #[test]
    fn deadline_tasks_path_is_correct() {
        let linux_path = "/.local/share/chartodo/deadline_tasks.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/deadline_tasks.json";
        let mac_path = "/Library/Application Support/chartodo/deadline_tasks.json";
        let mut got_deadline_tasks_path: bool = false;
        let deadline_path = path_to_deadline_tasks();
        let deadline_path = deadline_path.to_str().unwrap();

        if deadline_path.contains(linux_path)
            | deadline_path.contains(windows_path)
            | deadline_path.contains(mac_path)
        {
            got_deadline_tasks_path = true;
        }

        assert!(got_deadline_tasks_path);
    }

    #[test]
    fn deadline_tasks_copy_path_is_correct() {
        let linux_path = "/.local/share/chartodo/deadline_tasks_copy.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/deadline_tasks_copy.json";
        let mac_path = "/Library/Application Support/chartodo/deadline_tasks_copy.json";
        let mut got_deadline_tasks_copy_path: bool = false;
        let deadline_tasks_copy_path = deadline_tasks_copy_path();
        let deadline_tasks_copy_path = deadline_tasks_copy_path.to_str().unwrap();

        if deadline_tasks_copy_path.contains(linux_path)
            | deadline_tasks_copy_path.contains(windows_path)
            | deadline_tasks_copy_path.contains(mac_path)
        {
            got_deadline_tasks_copy_path = true;
        }

        assert!(got_deadline_tasks_copy_path);
    }

    #[test]
    fn aaaa_deadline_tasks_clone_file() {
        // name is aaaa so it's done first
        // since we will be modifying the original file to run a test, the original data must be
        // preserved first
        std::fs::File::create(deadline_tasks_copy_path())
            .context("failed to create deadline_tasks_copy.json")
            .expect("failed to create a copy during unit test");

        std::fs::copy(path_to_deadline_tasks(), deadline_tasks_copy_path())
            .context("failed to copy deadline_tasks.json to deadline_tasks_copy.json")
            .expect("failed to copy original file to copy file during unit test");
    }

    #[test]
    fn deadline_tasks_rmdone_done_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = deadline_tasks_rmdone(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_rmdone_no_valid_args() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that there are no valid args
        let arguments = vec![String::from("-11"), String::from("0"), String::from("2")];
        let error_should_be_true = deadline_tasks_rmdone(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_rmdone_should_do_deadlinecleardone() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

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
        let error_should_be_true = deadline_tasks_rmdone(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_rmdone_should_do_deadlinecleardone_range() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that there are no valid args
        let arguments = vec![String::from("1-6"), String::from("1")];
        let error_should_be_true = deadline_tasks_rmdone(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_rmdone_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hello",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1")];
        let error_should_be_false = deadline_tasks_rmdone(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_rmdone_multiple_args_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "welcome",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("3")];
        let error_should_be_false = deadline_tasks_rmdone(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hello",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_rmdone_multiple_args_is_correct_range() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "welcome",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("2-3")];
        let error_should_be_false = deadline_tasks_rmdone(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_notdone_done_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = deadline_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_notdone_no_valid_args() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that there are no valid args
        let arguments = vec![String::from("-11"), String::from("0"), String::from("2")];
        let error_should_be_true = deadline_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_notdone_should_do_deadlinenotdoneall() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

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
        let error_should_be_true = deadline_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_notdone_should_do_deadlinenotdoneall_range() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that there are no valid args
        let arguments = vec![String::from("1-6"), String::from("1")];
        let error_should_be_true = deadline_tasks_not_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_notdone_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hello",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1")];
        let error_should_be_false = deadline_tasks_not_done(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_notdone_multiple_args_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "welcome",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("3")];
        let error_should_be_false = deadline_tasks_not_done(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "welcome",
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
                ],
                "done": [
                    {
                        "task": "hello",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_notdone_multiple_args_is_correct_range() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "welcome",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("2-3")];
        let error_should_be_false = deadline_tasks_not_done(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "welcome",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_cleardone_done_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let error_should_be_true = deadline_tasks_clear_done();

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_cleardone_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hello",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let error_should_be_false = deadline_tasks_clear_done();
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_notdoneall_done_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let error_should_be_true = deadline_tasks_notdoneall();

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_notdoneall_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "welcome",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let error_should_be_false = deadline_tasks_notdoneall();
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
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
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "welcome",
                        "date": "2025-01-01",
                        "time": "00:00",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn zzzz_rename_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, remove the modified original and rename copy to original

        std::fs::remove_file(path_to_deadline_tasks())
            .context("failed delete modified deadline_tasks.json after running tests")
            .expect("failed to delete deadline_tasks.json after deadline_helpers unit tests");

        std::fs::rename(deadline_tasks_copy_path(), path_to_deadline_tasks())
            .context("failed to rename deadline_tasks_copy to deadline_tasks")
            .expect("failed to rename deadline_tasks_copy to deadline_tasks after tests were done");
    }
}
