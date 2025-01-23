use anyhow::Context;
use assert_cmd::prelude::*;
use chartodo::functions::{json_file_structs::*, regular_tasks::regular_helpers::*};
use predicates::prelude::*;
use std::{path::PathBuf, process::Command};

// cargo test --test regular_todo_tests -- --test-threads=1

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

mod aaa_do_this_first {
    use super::*;

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

        if regular_path.contains(linux_path)
            | regular_path.contains(windows_path)
            | regular_path.contains(mac_path)
        {
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

        if regular_tasks_copy_path.contains(linux_path)
            | regular_tasks_copy_path.contains(windows_path)
            | regular_tasks_copy_path.contains(mac_path)
        {
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
}

mod regular_todo_add {
    use super::*;

    #[test]
    fn regular_todo_adding_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Did not provide the todo item(s) \
            to be added. Good example: chartodo add new-item, or chartodo add item next-item \
            one-more-item. If you have questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_adding_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Did not provide the todo item(s) \
            to be added. Good example: chartodo a new-item, or chartodo a item next-item \
            one-more-item. If you have questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_adding_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("regular-todo-item");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular-todo-item"));

        Ok(())
    }

    #[test]
    fn regular_todo_adding_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("regular-todo-item");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular-todo-item"));

        Ok(())
    }

    #[test]
    fn regular_todo_adding_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("regular-todo-item").arg("hello");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular-todo-item"))
            .stdout(predicate::str::contains("2: hello"));

        Ok(())
    }

    #[test]
    fn regular_todo_adding_multiple_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("a").arg("regular-todo-item").arg("hello");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular-todo-item"))
            .stdout(predicate::str::contains("2: hello"));

        Ok(())
    }
}

mod regular_todo_done {
    use super::*;

    #[test]
    fn regular_todo_done_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Did not provide the todo item(s) \
            to be changed to done. Good example: chartodo done 3, or chartodo done 3 4 5. If \
            you have questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_done_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Did not provide the todo item(s) \
            to be changed to done. Good example: chartodo d 3, or chartodo d 3 4 5. If you \
            have questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_done_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty so you can't change any todos to done.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_done_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty so you can't change any todos to done.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_done_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("a").arg("2").arg("0");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you \
            provided were viable -- they were all either negative, zero, or exceeded the \
            regular todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_done_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("a").arg("2").arg("0");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you \
            provided were viable -- they were all either negative, zero, or exceeded the \
            regular todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_done_should_do_doneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done")
            .arg("1")
            .arg("2")
            .arg("1")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: you've specified marking \
            the entire regular todo list as done. You should do chartodo doneall.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_done_abrev_should_do_doneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d")
            .arg("1")
            .arg("2")
            .arg("1")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: you've specified marking \
            the entire regular todo list as done. You should do chartodo doneall.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_done_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"));
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: this-is-the-todo-list"));

        Ok(())
    }

    #[test]
    fn regular_todo_done_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"));
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: this-is-the-todo-list"));

        Ok(())
    }

    #[test]
    fn regular_todo_done_multiple_args_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                ],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("1").arg("3");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("2: this-is-the-todo-list"))
            .stdout(predicate::str::contains("1: hello"));

        Ok(())
    }

    #[test]
    fn regular_todo_done_abrev_multiple_args_is_correct() -> Result<(), Box<dyn std::error::Error>>
    {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                ],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("1").arg("3");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"));

        Ok(())
    }
}

mod regular_todo_rmtodo {
    use super::*;

    #[test]
    fn regular_todo_rmtodo_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Did not provide the todo item(s) \
            to be removed. Good example: chartodo rmtodo 3, or chartodo rmtodo 3 4 5. If you \
            have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Did not provide the todo item(s) \
            to be removed. Good example: chartodo rmt 3, or chartodo rmt 3 4 5. If you have \
            more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty, so you can't remove any items. Try adding to it first before \
            removing any.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty, so you can't remove any items. Try adding to it first before \
            removing any.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("a").arg("2").arg("0");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: none of the positions you \
            gave were valid -- they were all either negative, zero, or exceeded the regular \
            todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("a").arg("2").arg("0");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: none of the positions you \
            gave were valid -- they were all either negative, zero, or exceeded the regular \
            todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_should_do_doneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo")
            .arg("1")
            .arg("2")
            .arg("1")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You specified removing \
            the entire regular todo list. You should instead do chartodo cleartodo.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_abrev_should_do_doneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": null,
                        "time": null,
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt")
            .arg("1")
            .arg("2")
            .arg("1")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You specified removing \
            the entire regular todo list. You should instead do chartodo cleartodo.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"));

        Ok(())
    }

    // note that there is a problem with checking str::contains since I think comfy_table does
    // some shenanigans to print the table that doesn't play nice w/ str::contains
    //
    // thus, concessions with testing have to be made. idk about other alternatives
    #[test]
    fn regular_todo_rmtodo_multiple_args_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                ],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("1").arg("3");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"));

        Ok(())
    }

    #[test]
    fn regular_todo_rmtodo_abrev_multiple_args_is_correct() -> Result<(), Box<dyn std::error::Error>>
    {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                ],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("1").arg("3");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"));

        Ok(())
    }
}

mod regular_todo_cleartodo {
    use super::*;

    #[test]
    fn regular_todo_cleartodo_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cleartodo").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_cleartodo_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ct").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_cleartodo_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cleartodo");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty. Try adding items to it first before removing any.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_cleartodo_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ct");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty. Try adding items to it first before removing any.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_cleartodo_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cleartodo");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: this-is-the-todo-list"))
            .is_err());

        Ok(())
    }

    #[test]
    fn regular_todo_cleartodo_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ct");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("2: hi"))
            .is_err());

        Ok(())
    }
}

mod regular_todo_doneall {
    use super::*;

    #[test]
    fn regular_todo_doneall_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("doneall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_doneall_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("da").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_doneall_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("doneall");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty, so you can't change any todos to done.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_doneall_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("da");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty, so you can't change any todos to done.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_doneall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                ],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        // we must mess up the order so we know doneall was actually successful
        cmd.arg("rmt").arg("1");
        cmd.assert().success();
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("doneall");
        cmd.assert().success();
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("welcome");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"))
            .stdout(predicate::str::contains("2: hello"))
            .stdout(predicate::str::contains("1: welcome"));

        Ok(())
    }

    #[test]
    fn regular_todo_doneall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
                ],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        // we must mess up the order so we know doneall was actually successful
        cmd.arg("rmt").arg("1");
        cmd.assert().success();
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("da");
        cmd.assert().success();
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("add").arg("welcome");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"))
            .stdout(predicate::str::contains("2: hello"))
            .stdout(predicate::str::contains("1: welcome"));

        Ok(())
    }
}

mod regular_todo_edit {
    use super::*;

    #[test]
    fn regular_todo_edit_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Did not provide the todo item to \
            be edited. Good example: chartodo edit 3 abc. If you have more questions, try \
            chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e");
        cmd.assert().failure().stderr(predicate::str::contains(
            "Did not provide the todo item \
            to be edited. Good example: chartodo e 3 abc. If you have more questions, try \
            chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("a");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular todo list is \
            currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You must specify the \
            regular todo's position that will be edited, and what to edit the task \
            to.\n\tThere should be 2 arguments after 'chartodo edit'. You provided 1 \
            argument(s).\n\tFormat: chartodo edit ~position ~task\n\tExample: chartodo edit \
            4 new-item",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You must specify the \
            regular todo's position that will be edited, and what to edit the task \
            to.\n\tThere should be 2 arguments after 'chartodo edit'. You provided 1 \
            argument(s).\n\tFormat: chartodo edit ~position ~task\n\tExample: chartodo edit \
            4 new-item",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("a").arg("a");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: To edit a regular task \
            item, you must provide a viable position. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_abrev_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("a").arg("a");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: To edit a regular task \
            item, you must provide a viable position. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("0").arg("a");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_abrev_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("0").arg("a");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_position_exceeds_len() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("2").arg("a");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The position you specified exceeds the regular todo list's \
            current length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_abrev_position_exceeds_len() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("2").arg("a");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The position you specified exceeds the regular todo list's \
            current length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn regular_todo_edit_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("edit").arg("1").arg("hi");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"));
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: this-is-the-todo-list"))
            .is_err());

        Ok(())
    }

    #[test]
    fn regular_todo_edit_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
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
        let fresh_regular_tasks: Tasks = serde_json::from_str(fresh_regular_tasks)
            .context(
                "during testing: the fresh data to put in the new regular_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_regular_tasks(fresh_regular_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("e").arg("1").arg("hi");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: hi"));
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: this-is-the-todo-list"))
            .is_err());

        Ok(())
    }
}

mod zzz_do_this_last {
    use super::*;

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
