use anyhow::Context;
use assert_cmd::prelude::*;
use chartodo::functions::{
    deadline_tasks::deadline_helpers::*, json_file_structs::*, regular_tasks::regular_helpers::*,
    repeating_tasks::repeating_helpers::*,
};
use predicates::prelude::*;
use std::{path::PathBuf, process::Command};

// cargo test --test general_commands_tests -- --test-threads=1

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

fn path_to_repeating_tasks() -> PathBuf {
    // get the data dir XDG spec and return it with path to regular_tasks.json
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

        if deadline_path.contains(linux_path) {
            got_deadline_tasks_path = true;
        } else if deadline_path.contains(windows_path) {
            got_deadline_tasks_path = true;
        } else if deadline_path.contains(mac_path) {
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

        if deadline_tasks_copy_path.contains(linux_path) {
            got_deadline_tasks_copy_path = true;
        } else if deadline_tasks_copy_path.contains(windows_path) {
            got_deadline_tasks_copy_path = true;
        } else if deadline_tasks_copy_path.contains(mac_path) {
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
}

mod general_commands_list {
    use super::*;

    #[test]
    fn list_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("list").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn list_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("l").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn list_prints_correctly() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "regular",
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
                        "task": "deadline",
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
                        "task": "repeating",
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
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("list");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular"))
            .stdout(predicate::str::contains("1: deadline"))
            .stdout(predicate::str::contains("1: repeating"))
            .stdout(predicate::str::contains("CHARTODO"))
            .stdout(predicate::str::contains("DEADLINE"))
            .stdout(predicate::str::contains("REPEATING"));

        Ok(())
    }

    #[test]
    fn list_abrev_prints_correctly() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "regular",
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
                        "task": "deadline",
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
                        "task": "repeating",
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
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("l");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: regular"))
            .stdout(predicate::str::contains("1: deadline"))
            .stdout(predicate::str::contains("1: repeating"))
            .stdout(predicate::str::contains("CHARTODO"))
            .stdout(predicate::str::contains("DEADLINE"))
            .stdout(predicate::str::contains("REPEATING"));

        Ok(())
    }
}

mod general_commands_clearall {
    use super::*;

    #[test]
    fn clearall_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn clearall_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn clearall_all_lists_empty() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: All of the lists are currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn clearall_abrev_all_lists_empty() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: All of the lists are currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn clearall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "regular",
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
                        "task": "regular-done",
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
                        "task": "deadline",
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
                        "task": "deadline-done",
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
                        "task": "repeating",
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
                        "task": "repeating-done",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: regular"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: regular-done"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-done"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-done"))
            .is_err());

        Ok(())
    }

    #[test]
    fn clearall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "regular",
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
                        "task": "regular-done",
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
                        "task": "deadline",
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
                        "task": "deadline-done",
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
                        "task": "repeating",
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
                        "task": "repeating-done",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: regular"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: regular-done"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-done"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-done"))
            .is_err());

        Ok(())
    }
}

mod general_commands_clearregular {
    use super::*;

    #[test]
    fn clearregular_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall-regular").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn clearregular_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca-r").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn clearregular_all_lists_empty() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall-regular");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular task lists are currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn clearregular_abrev_all_lists_empty() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca-r");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The regular task lists are currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn clearregular_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "regular",
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
                        "task": "regular-done",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall-regular");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: regular"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: regular-done"))
            .is_err());

        Ok(())
    }

    #[test]
    fn clearregular_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to regular tasks so content is known
        let fresh_regular_tasks = r#"
            {
                "todo": [
                    {
                        "task": "regular",
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
                        "task": "regular-done",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca-r");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: regular"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: regular-done"))
            .is_err());

        Ok(())
    }
}

mod general_commands_cleardeadline {
    use super::*;

    #[test]
    fn cleardeadline_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall-deadline").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleardeadline_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca-d").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleardeadline_all_lists_empty() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall-deadline");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline task lists are currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn cleardeadline_abrev_all_lists_empty() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca-d");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline task lists are currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn cleardeadline_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline",
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
                        "task": "deadline-done",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall-deadline");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-done"))
            .is_err());

        Ok(())
    }

    #[test]
    fn cleardeadline_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline",
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
                        "task": "deadline-done",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cr-d");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-done"))
            .is_err());

        Ok(())
    }
}

mod general_commands_clearrepeating {
    use super::*;

    #[test]
    fn clearrepeating_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall-repeating").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn clearrepeating_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca-rp").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn clearrepeating_all_lists_empty() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall-repeating");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating task lists are currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn clearrepeating_abrev_all_lists_empty() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca-rp");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating task lists are currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn clearrepeating_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating",
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
                        "task": "repeating-done",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("clearall-repeating");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-done"))
            .is_err());

        Ok(())
    }

    #[test]
    fn clearrepeating_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating",
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
                        "task": "repeating-done",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ca-rp");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating"))
            .is_err());
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-done"))
            .is_err());

        Ok(())
    }
}

mod main_help {
    use super::*;

    #[test]
    fn help_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("help").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn help_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("h").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn help_prints_correctly() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("help");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("
    CHARTODO is a simple command-line-interface (CLI) todo list application

    Commands:
        If a command says it has chaining, it means you can include multiple separate tasks or positions
        Date format is always in year-month-day, e.g., 2099-12-25
        Time format is always in a 24-hour format, e.g., 13:58. Note that there is no space between hour and minute
        Only the following time units are allowed in repeating tasks: minutes, hours, days, weeks, months, years

        help | h
            show help
            example: chartodo help
        list | l
            show the todo list
            example: chartodo list
        clearall | ca
            clear everything (TODO, DEADLINE, REPEATING)
            example: chartodo ca
        clearall-regular | ca-r
            clear all regular todo and done tasks
            example: chartodo ca-r
        clearall-deadline | ca-d
            clear all deadline todo and done tasks
            example: chartodo ca-d
        clearall-repeating | ca-rp
            clear all repeating todo and done tasks
            example: chartodo ca-rp

        TODO:
            add | a
                add an item to the todo list. To add a multi-word item, replace space with something like -
                format: chartodo add [task]
                example: chartodo add new-item
                example: chartodo add 1st-item 2nd-item 3rd-item
            done | d
                change a todo item to done, using numbered positions to specify which one(s). Has chaining
                format: chartodo done [position]
                example: chartodo done 3
                example: chartodo d 5 1 3 2
            notdone | nd
                reverses a done item back to a todo item using numbered positions. Has chaining
                format: chartodo notdone [position]
                example: chartodo nd 3
            rmtodo | rmt
                remove a todo item from the list using numbered positions. Has chaining
                format: chartodo rmtodo [position]
                example: chartodo rmt 4 1 5
            rmdone | rmd
                removes a done item using numbered positions. Has chaining
                format: chartodo rmdone [position]
                example: chartodo rmd 4
            doneall | da
                change all todo items to done
                format: chartodo doneall
            notdoneall | nda
                reverses all done items back to todo
                format: chartodo notdoneall
            cleartodo | ct
                clear the todo list
                format: chartodo cleartodo
            cleardone | cd
                clear the done list
                format: chartodo cleardone
            clearboth | cb
                clear both todo and done lists
                format: chartodo clearboth
            edit | e
                changes a todo item, with its position specified, to what you want
                format: chartoo edit [position] [new task]
                example: chartodo edit 3 change-item-to-this

        DEADLINE:
            deadline-add | dl-a
                adds a task with a day and time limit. Has chaining
                format: chartodo deadline-add [deadline task] [ending date] [ending time]
                example: chartodo dl-a go-on-a-run 2099-01-01 08:00
                example: chartodo dl-a go-shopping 2030-12-01 13:00 go-bowling 2030-12-01 15:30
            deadline-addonlydate | dl-aod
                adds a deadline task. only the date is specified and time defaults to 00:00. Has chaining
                format: chartodo deadline-addonlydate [deadline task] [ending date]
                example: chartodo dl-aod midnight 2099-12-12
            deadline-addonlytime | dl-aot
                adds a deadline task. only the time is specified and date defaults to current date. Has chaining
                format: chartodo deadline-addonlytime [deadline task] [ending time]
                example: chartodo dl-aot homework-due-today 23:59
            deadline-done | dl-d
                mark one/several deadline task(s) as done using numbered positions. Has chaining
                format: chartodo deadline-done [position]
                example: chartodo dl-d 1
                example: chartodo dl-d 1 2 3 4 5
            deadline-notdone | dl-nd
                reverses a deadline done item back to todo using numbered positions. Has chaining
                format: chartodo deadline-notdone [position]
                example: chartodo dl-nd 5 4 1
            deadline-rmtodo | dl-rmt
                remove one or several todo item(s) using numbered positions. Has chaining
                format: chartodo deadline-rmtodo [position]
                example: chartodo dl-rmt 1
            deadline-rmdone | dl-rmd
                removes a deadline done item using numbered position. Has chaining
                format: chartodo deadline-rmdone [position]
                example: chartodo dl-rmd 3 2 1
            deadline-doneall | dl-da
                mark the entire deadline todo list as done
                format: chartodo deadline-doneall
            deadline-notdoneall | dl-nda
                reverses all deadline done items back to todo
                format: chartodo deadline-notdoneall
            deadline-cleartodo | deadline-ct
                clear the deadline todo list
                format: chartodo deadline-cleartodo
            deadline-cleardone | dl-cd
                clears the deadline done list
                format: chartodo deadline-cleardone
            deadline-clearboth | dl-cb
                clears both of the deadline todo and done lists
                format: chartodo deadline-clearboth
            deadline-editall | dl-ea
                edit all the parameters of a deadline todo task
                format: chartodo deadline-editall [position] [new deadline task] [new ending date] [new ending time]
                example: chartodo dl-ea 1 new-item 2150-01-01 00:00
            deadline-edittask | dl-eta
                edit the task parameter of a deadline todo task
                format: chartodo deadline-edittask [position] [new deadline task]
                example: chartodo dl-eta 1 new-item
            deadline-editdate | dl-ed
                edit the date parameter of a deadline todo task
                format: chartodo deadline-editdate [position] [new ending date]
                example: chartodo dl-ed 1 2150-01-1
            deadline-edittime | dl-eti
                edit the time parameter of a deadline todo task
                format: chartodo deadline-edittime [position] [new ending time]
                example: chartodo dl-eti 1 23:59
            deadline-editdatetime | dl-edt
                edit the date and time parameter of a deadline todo task
                format: chartodo deadline-editdatetime [position] [new ending date] [new ending time]
                example: chartodo dl-edt 1 2100-01-01 13:00

        REPEATING:
        note: Only the following time units are allowed in repeating tasks: minutes, hours, days, weeks, months, years
            repeating-add | rp-a
                add a repeating task with a set interval. the task starts from your current date and time. Has chaining
                format: chartodo repeating-add [repeating task] [interval] [time unit]
                example: chartodo rp-a gym 2 days
                example: chartood rp-a gym 2 days mow 1 week
            repeating-addstart | rp-as
                add a repeating task that starts on your specified datetime. Has chaining
                format: chartodo repeating-addstart [repeating task] [interval] [time unit] [starting date] [starting time]
                example: chartodo rp-as task 3 days 2099-01-01 00:00
            repeating-addend | rp-ae
                add a repeating task that ends on your specified datetime. Has chaining
                format: chartodo repeating-addend [repeating task] [interval] [time unit] [ending date] [ending time]
                example: chartodo rp-ae task 3 days 2099-01-01 00:00
            repeating-done | rp-d
                mark repeating todos as done. Has chaining
                format: chartodo repeating-done [position]
                example: chartodo rp-d 1
                example: chartodo rp-d 1 2 3 4 5
            repeating-reset | repeating-donereset | rp-r | rp-dr
                reset the starting datetime of a repeating task to your current date and time. Has chaining
                    functionally, this can also be used to mark a repeating task as 'done' but
                    immediately start the interval again with your current date and time
                format: chartodo repeating-reset [position]
                example: chartodo rp-dr 1
            repeating-notdone | rp-nd
                reverse repeating dones back to todo. Has chaining
                format: chartodo repeating-notdone [position]
                example: chartodo rp-nd 1
            repeating-rmtodo | rp-rmt
                remove a repeating todo task. Has chaining
                format: chartodo repeating-rmtodo [position]
                example: chartodo rp-rmt 1
            repeating-rmdone | rp-rmd
                remove one/several repeating done task(s). Has chaining
                format: chartodo repeating-rmdone [position]
                example: chartodo rp-rmd 1
            repeating-start | rp-s
                show the starting datetime of one or more repeating tasks. Has chaining
                format: chartodo repeating-start [position]
                example: chartodo rp-s 1
            repeating-doneall | rp-da
                mark all repeating tasks as done
                format: chartodo repeating-doneall
            repeating-notdoneall | rp-nda
                reverse all finished repeating tasks back to todo
                format: chartodo repeating-notdoneall
            repeating-cleartodo | rp-ct
                delete all of the repeating todo tasks
                format: chartodo repeating-cleartodo
            repeating-cleardone | rp-cd
                delete all of the finished repeating tasks
                format: chartodo repeating-cleardone
            repeating-clearboth | rp-cb
                clear the repeating todo and done lists
                format: chearotod repeating-clearboth
            repeating-resetall | repeating-doneresetall | rp-ra | rp-dra
                resets the starting datetime of all repeating tasks to your current date and time
                format: chartodo repeating-resetall
            repeating-startall | rp-sa
                show the starting datetime of all repeating tasks
                format: chartodo repeating-startall
            repeating-editall | rp-ea
                edit all the parameters of a repeating task: task, interval, time unit, and starting/ending datetime
                You must specify whether it's the starting or ending datetime using keywords 'start' or 'end'
                format: chartodo repeating-editall [position] [new repeating task] [interval] [time unit] start/end [date] [time]
                example: chartodo rp-ea 1 new-repeating-task 3 days start 2000-01-01 00:00
                example: chartodo rp-ea 1 new-repeating-task 3 days end 2100-01-01 00:00
            repeating-edittask | rp-eta
                edit the task parameter of a repeating task
                format: chartodo repeating-edittask [position] [new repeating task]
                example: chartodo rp-eta 1 new-task
            repeating-editinterval | rp-ei
                edit the interval of a repeating task
                format: chartodo repeating-editinterval [position] [interval]
                example: chartodo rp-ei 1 3
            repeating-editunit | rp-eu
                edit the time unit of a repeating task
                format: chartodo repeating-editunit [position] [time unit]
                example: chartodo rp-eu 1 weeks
            repeating-editintervalunit | rp-eiu
                edit the interval and time unit of a repeating task
                format: chartodo repeating-editintervalunit [position] [interval] [time unit]
                example: chartodo rp-eiu 1 3 days
            repeating-editstart | rp-es
                edit the starting datetime of a repeating task
                format: chartodo repeating-editstart [position] [starting date] [starting time]
                example: chartodo rp-es 2100-12-24 13:08
            repeating-editend | rp-ee
                edit the ending datetime of a repeating task
                format: chartodo repeating-editend [position] [ending date] [ending time]
                example: chartodo rp-ee 2100-12-24 13:08
"));

        Ok(())
    }

    #[test]
    fn help_abrev_prints_correctly() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("h");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("
    CHARTODO is a simple command-line-interface (CLI) todo list application

    Commands:
        If a command says it has chaining, it means you can include multiple separate tasks or positions
        Date format is always in year-month-day, e.g., 2099-12-25
        Time format is always in a 24-hour format, e.g., 13:58. Note that there is no space between hour and minute
        Only the following time units are allowed in repeating tasks: minutes, hours, days, weeks, months, years

        help | h
            show help
            example: chartodo help
        list | l
            show the todo list
            example: chartodo list
        clearall | ca
            clear everything (TODO, DEADLINE, REPEATING)
            example: chartodo ca
        clearall-regular | ca-r
            clear all regular todo and done tasks
            example: chartodo ca-r
        clearall-deadline | ca-d
            clear all deadline todo and done tasks
            example: chartodo ca-d
        clearall-repeating | ca-rp
            clear all repeating todo and done tasks
            example: chartodo ca-rp

        TODO:
            add | a
                add an item to the todo list. To add a multi-word item, replace space with something like -
                format: chartodo add [task]
                example: chartodo add new-item
                example: chartodo add 1st-item 2nd-item 3rd-item
            done | d
                change a todo item to done, using numbered positions to specify which one(s). Has chaining
                format: chartodo done [position]
                example: chartodo done 3
                example: chartodo d 5 1 3 2
            notdone | nd
                reverses a done item back to a todo item using numbered positions. Has chaining
                format: chartodo notdone [position]
                example: chartodo nd 3
            rmtodo | rmt
                remove a todo item from the list using numbered positions. Has chaining
                format: chartodo rmtodo [position]
                example: chartodo rmt 4 1 5
            rmdone | rmd
                removes a done item using numbered positions. Has chaining
                format: chartodo rmdone [position]
                example: chartodo rmd 4
            doneall | da
                change all todo items to done
                format: chartodo doneall
            notdoneall | nda
                reverses all done items back to todo
                format: chartodo notdoneall
            cleartodo | ct
                clear the todo list
                format: chartodo cleartodo
            cleardone | cd
                clear the done list
                format: chartodo cleardone
            clearboth | cb
                clear both todo and done lists
                format: chartodo clearboth
            edit | e
                changes a todo item, with its position specified, to what you want
                format: chartoo edit [position] [new task]
                example: chartodo edit 3 change-item-to-this

        DEADLINE:
            deadline-add | dl-a
                adds a task with a day and time limit. Has chaining
                format: chartodo deadline-add [deadline task] [ending date] [ending time]
                example: chartodo dl-a go-on-a-run 2099-01-01 08:00
                example: chartodo dl-a go-shopping 2030-12-01 13:00 go-bowling 2030-12-01 15:30
            deadline-addonlydate | dl-aod
                adds a deadline task. only the date is specified and time defaults to 00:00. Has chaining
                format: chartodo deadline-addonlydate [deadline task] [ending date]
                example: chartodo dl-aod midnight 2099-12-12
            deadline-addonlytime | dl-aot
                adds a deadline task. only the time is specified and date defaults to current date. Has chaining
                format: chartodo deadline-addonlytime [deadline task] [ending time]
                example: chartodo dl-aot homework-due-today 23:59
            deadline-done | dl-d
                mark one/several deadline task(s) as done using numbered positions. Has chaining
                format: chartodo deadline-done [position]
                example: chartodo dl-d 1
                example: chartodo dl-d 1 2 3 4 5
            deadline-notdone | dl-nd
                reverses a deadline done item back to todo using numbered positions. Has chaining
                format: chartodo deadline-notdone [position]
                example: chartodo dl-nd 5 4 1
            deadline-rmtodo | dl-rmt
                remove one or several todo item(s) using numbered positions. Has chaining
                format: chartodo deadline-rmtodo [position]
                example: chartodo dl-rmt 1
            deadline-rmdone | dl-rmd
                removes a deadline done item using numbered position. Has chaining
                format: chartodo deadline-rmdone [position]
                example: chartodo dl-rmd 3 2 1
            deadline-doneall | dl-da
                mark the entire deadline todo list as done
                format: chartodo deadline-doneall
            deadline-notdoneall | dl-nda
                reverses all deadline done items back to todo
                format: chartodo deadline-notdoneall
            deadline-cleartodo | deadline-ct
                clear the deadline todo list
                format: chartodo deadline-cleartodo
            deadline-cleardone | dl-cd
                clears the deadline done list
                format: chartodo deadline-cleardone
            deadline-clearboth | dl-cb
                clears both of the deadline todo and done lists
                format: chartodo deadline-clearboth
            deadline-editall | dl-ea
                edit all the parameters of a deadline todo task
                format: chartodo deadline-editall [position] [new deadline task] [new ending date] [new ending time]
                example: chartodo dl-ea 1 new-item 2150-01-01 00:00
            deadline-edittask | dl-eta
                edit the task parameter of a deadline todo task
                format: chartodo deadline-edittask [position] [new deadline task]
                example: chartodo dl-eta 1 new-item
            deadline-editdate | dl-ed
                edit the date parameter of a deadline todo task
                format: chartodo deadline-editdate [position] [new ending date]
                example: chartodo dl-ed 1 2150-01-1
            deadline-edittime | dl-eti
                edit the time parameter of a deadline todo task
                format: chartodo deadline-edittime [position] [new ending time]
                example: chartodo dl-eti 1 23:59
            deadline-editdatetime | dl-edt
                edit the date and time parameter of a deadline todo task
                format: chartodo deadline-editdatetime [position] [new ending date] [new ending time]
                example: chartodo dl-edt 1 2100-01-01 13:00

        REPEATING:
        note: Only the following time units are allowed in repeating tasks: minutes, hours, days, weeks, months, years
            repeating-add | rp-a
                add a repeating task with a set interval. the task starts from your current date and time. Has chaining
                format: chartodo repeating-add [repeating task] [interval] [time unit]
                example: chartodo rp-a gym 2 days
                example: chartood rp-a gym 2 days mow 1 week
            repeating-addstart | rp-as
                add a repeating task that starts on your specified datetime. Has chaining
                format: chartodo repeating-addstart [repeating task] [interval] [time unit] [starting date] [starting time]
                example: chartodo rp-as task 3 days 2099-01-01 00:00
            repeating-addend | rp-ae
                add a repeating task that ends on your specified datetime. Has chaining
                format: chartodo repeating-addend [repeating task] [interval] [time unit] [ending date] [ending time]
                example: chartodo rp-ae task 3 days 2099-01-01 00:00
            repeating-done | rp-d
                mark repeating todos as done. Has chaining
                format: chartodo repeating-done [position]
                example: chartodo rp-d 1
                example: chartodo rp-d 1 2 3 4 5
            repeating-reset | repeating-donereset | rp-r | rp-dr
                reset the starting datetime of a repeating task to your current date and time. Has chaining
                    functionally, this can also be used to mark a repeating task as 'done' but
                    immediately start the interval again with your current date and time
                format: chartodo repeating-reset [position]
                example: chartodo rp-dr 1
            repeating-notdone | rp-nd
                reverse repeating dones back to todo. Has chaining
                format: chartodo repeating-notdone [position]
                example: chartodo rp-nd 1
            repeating-rmtodo | rp-rmt
                remove a repeating todo task. Has chaining
                format: chartodo repeating-rmtodo [position]
                example: chartodo rp-rmt 1
            repeating-rmdone | rp-rmd
                remove one/several repeating done task(s). Has chaining
                format: chartodo repeating-rmdone [position]
                example: chartodo rp-rmd 1
            repeating-start | rp-s
                show the starting datetime of one or more repeating tasks. Has chaining
                format: chartodo repeating-start [position]
                example: chartodo rp-s 1
            repeating-doneall | rp-da
                mark all repeating tasks as done
                format: chartodo repeating-doneall
            repeating-notdoneall | rp-nda
                reverse all finished repeating tasks back to todo
                format: chartodo repeating-notdoneall
            repeating-cleartodo | rp-ct
                delete all of the repeating todo tasks
                format: chartodo repeating-cleartodo
            repeating-cleardone | rp-cd
                delete all of the finished repeating tasks
                format: chartodo repeating-cleardone
            repeating-clearboth | rp-cb
                clear the repeating todo and done lists
                format: chearotod repeating-clearboth
            repeating-resetall | repeating-doneresetall | rp-ra | rp-dra
                resets the starting datetime of all repeating tasks to your current date and time
                format: chartodo repeating-resetall
            repeating-startall | rp-sa
                show the starting datetime of all repeating tasks
                format: chartodo repeating-startall
            repeating-editall | rp-ea
                edit all the parameters of a repeating task: task, interval, time unit, and starting/ending datetime
                You must specify whether it's the starting or ending datetime using keywords 'start' or 'end'
                format: chartodo repeating-editall [position] [new repeating task] [interval] [time unit] start/end [date] [time]
                example: chartodo rp-ea 1 new-repeating-task 3 days start 2000-01-01 00:00
                example: chartodo rp-ea 1 new-repeating-task 3 days end 2100-01-01 00:00
            repeating-edittask | rp-eta
                edit the task parameter of a repeating task
                format: chartodo repeating-edittask [position] [new repeating task]
                example: chartodo rp-eta 1 new-task
            repeating-editinterval | rp-ei
                edit the interval of a repeating task
                format: chartodo repeating-editinterval [position] [interval]
                example: chartodo rp-ei 1 3
            repeating-editunit | rp-eu
                edit the time unit of a repeating task
                format: chartodo repeating-editunit [position] [time unit]
                example: chartodo rp-eu 1 weeks
            repeating-editintervalunit | rp-eiu
                edit the interval and time unit of a repeating task
                format: chartodo repeating-editintervalunit [position] [interval] [time unit]
                example: chartodo rp-eiu 1 3 days
            repeating-editstart | rp-es
                edit the starting datetime of a repeating task
                format: chartodo repeating-editstart [position] [starting date] [starting time]
                example: chartodo rp-es 2100-12-24 13:08
            repeating-editend | rp-ee
                edit the ending datetime of a repeating task
                format: chartodo repeating-editend [position] [ending date] [ending time]
                example: chartodo rp-ee 2100-12-24 13:08
"));

        Ok(())
    }
}

mod main_empty {
    use super::*;

    #[test]
    fn main_no_args_given() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "You must provide a command. Try chartodo help.",
        ));

        Ok(())
    }
}

mod main_wrongcommand {
    use super::*;

    #[test]
    fn main_wrong_command() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("ahahahahahahaha");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }
}

mod zzz_do_this_last {
    use super::*;

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
