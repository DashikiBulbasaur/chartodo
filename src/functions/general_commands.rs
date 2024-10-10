use super::general_helpers::*;
use crate::functions::{
    deadline_tasks::deadline_helpers::*, regular_tasks::regular_helpers::*,
    repeating_tasks::repeating_helpers::*,
};
use comfy_table::*;
use modifiers::UTF8_ROUND_CORNERS;
use presets::UTF8_FULL;
use std::io::Write;

pub fn list() {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    deadline_tasks_create_dir_and_file_if_needed();
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();
    let mut table = Table::new();

    // open file and parse
    let regular_tasks = open_regular_tasks_and_return_tasks_struct();
    let deadline_tasks = open_deadline_tasks_and_return_tasks_struct();
    let repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // get strings to print
    let (regular_todo, regular_done) = regular_tasks_list(regular_tasks);
    let (deadline_todo, deadline_done) = deadline_tasks_list(deadline_tasks);
    let (repeating_todo, repeating_done) = repeating_tasks_list(repeating_tasks);

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("CHARTODO").add_attribute(Attribute::Bold),
            Cell::new("DEADLINES").add_attribute(Attribute::Bold),
            Cell::new("REPEATING").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            format!("{}", regular_todo),
            format!("{}", deadline_todo),
            format!("{}", repeating_todo),
        ])
        .add_row(vec![
            format!("{}", regular_done),
            format!("{}", deadline_done),
            format!("{}", repeating_done),
        ]);

    writeln!(writer, "{table}").expect("writeln failed");
}

pub fn clear_all_lists() -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    deadline_tasks_create_dir_and_file_if_needed();
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if all lists are empty
    if regular_tasks.todo.is_empty()
        && regular_tasks.done.is_empty()
        && deadline_tasks.todo.is_empty()
        && deadline_tasks.done.is_empty()
        && repeating_tasks.todo.is_empty()
        && repeating_tasks.done.is_empty()
    {
        writeln!(writer, "ERROR: All of the lists are currently empty.").expect("writeln failed");

        // error = true
        return true;
    }

    // clear all lists
    regular_tasks.todo.clear();
    regular_tasks.done.clear();
    deadline_tasks.todo.clear();
    deadline_tasks.done.clear();
    repeating_tasks.todo.clear();
    repeating_tasks.done.clear();

    // write changes to files
    write_changes_to_new_regular_tasks(regular_tasks);
    write_changes_to_new_deadline_tasks(deadline_tasks);
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

pub fn clear_regular_tasks() -> bool {
    // housekeeping
    regular_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut regular_tasks = open_regular_tasks_and_return_tasks_struct();

    // check if all lists are empty
    if regular_tasks.todo.is_empty() && regular_tasks.done.is_empty() {
        writeln!(writer, "ERROR: The regular task lists are currently empty.")
            .expect("writeln failed");

        // error = true
        return true;
    }

    // clear all lists
    regular_tasks.todo.clear();
    regular_tasks.done.clear();

    // write changes to file
    write_changes_to_new_regular_tasks(regular_tasks);

    // error = false
    false
}

pub fn clear_deadline_tasks() -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if all lists are empty
    if deadline_tasks.todo.is_empty() && deadline_tasks.done.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline task lists are currently empty."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // clear all lists
    deadline_tasks.todo.clear();
    deadline_tasks.done.clear();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn clear_repeating_tasks() -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if all lists are empty
    if repeating_tasks.todo.is_empty() && repeating_tasks.done.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating task lists are currently empty."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // clear all lists
    repeating_tasks.todo.clear();
    repeating_tasks.done.clear();

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

// cargo test general_commands_unit_tests -- --test-threads=1
#[cfg(test)]
mod general_commands_unit_tests {
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
    fn clearall_all_lists_are_empty() {
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
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks).
            context(
                "during testing: the fresh data to put in the new deadline_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);
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

        let error_should_be_true = clear_all_lists();
        assert!(error_should_be_true);
    }

    #[test]
    fn clearall_is_correct() {
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
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks).
            context(
                "during testing: the fresh data to put in the new deadline_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
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

        // this should be the content of the files
        // write fresh to regular tasks so content is known
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
        // write fresh to deadline tasks so content is known
        let deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks).
            context(
                "during testing: the fresh data to put in the new deadline_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        // write fresh to repeating tasks so content is known
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

        let error_should_be_false = clear_all_lists();
        assert!(!error_should_be_false);

        let read_regular_tasks = open_regular_tasks_and_return_tasks_struct();
        let read_deadline_tasks = open_deadline_tasks_and_return_tasks_struct();
        let read_repeating_tasks = open_repeating_tasks_and_return_tasks_struct();
        assert_eq!(read_regular_tasks, regular_tasks);
        assert_eq!(read_deadline_tasks, deadline_tasks);
        assert_eq!(read_repeating_tasks, repeating_tasks);
    }

    #[test]
    fn clearregular_lists_are_empty() {
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

        let error_should_be_true = clear_regular_tasks();
        assert!(error_should_be_true);
    }

    #[test]
    fn clearregular_is_correct() {
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

        let error_should_be_false = clear_regular_tasks();
        assert!(!error_should_be_false);

        let read_regular_tasks = open_regular_tasks_and_return_tasks_struct();
        assert_eq!(read_regular_tasks, regular_tasks);
    }

    #[test]
    fn cleardeadline_lists_are_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks).
            context(
                "during testing: the fresh data to put in the new deadline_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        let error_should_be_true = clear_deadline_tasks();
        assert!(error_should_be_true);
    }

    #[test]
    fn cleardeadline_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks).
            context(
                "during testing: the fresh data to put in the new deadline_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        let error_should_be_false = clear_deadline_tasks();
        assert!(!error_should_be_false);

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks).
            context(
                "during testing: the fresh data to put in the new deadline_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        let read_deadline_tasks = open_deadline_tasks_and_return_tasks_struct();
        assert_eq!(read_deadline_tasks, deadline_tasks);
    }

    #[test]
    fn clearrepeating_lists_are_empty() {
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

        let error_should_be_true = clear_repeating_tasks();
        assert!(error_should_be_true);
    }

    #[test]
    fn clearrepeating_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
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

        let error_should_be_false = clear_repeating_tasks();
        assert!(!error_should_be_false);

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

        let read_repeating_tasks = open_repeating_tasks_and_return_tasks_struct();
        assert_eq!(read_repeating_tasks, repeating_tasks);
    }

    #[test]
    fn zzzz_rename_regular_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, remove the modified original and rename copy to original

        std::fs::remove_file(path_to_regular_tasks())
            .context("failed delete modified regular_tasks.json after running tests")
            .expect("failed to delete regular_tasks.json after regular_helpers unit tests");

        std::fs::rename(regular_tasks_copy_path(), path_to_regular_tasks())
            .context("failed to rename regular_tasks_copy to regular_tasks")
            .expect("failed to rename regular_tasks_copy to regular_tasks after tests were done");
    }

    #[test]
    fn zzzz_rename_deadline_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, remove the modified original and rename copy to original

        std::fs::remove_file(path_to_deadline_tasks())
            .context("failed delete modified deadline_tasks.json after running tests")
            .expect("failed to delete deadline_tasks.json after deadline_helpers unit tests");

        std::fs::rename(deadline_tasks_copy_path(), path_to_deadline_tasks())
            .context("failed to rename deadline_tasks_copy to deadline_tasks")
            .expect("failed to rename deadline_tasks_copy to deadline_tasks after tests were done");
    }

    #[test]
    fn zzzz_rename_repeating_copy_to_original() {
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
