use anyhow::Context;
use assert_cmd::prelude::*;
use chartodo::functions::{json_file_structs::*, repeating_tasks::repeating_helpers::*};
use predicates::prelude::*;
use std::{path::PathBuf, process::Command};

// cargo test --test repeating_done_tests -- --test-threads=1

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
    fn repeating_tasks_path_is_correct() {
        let linux_path = "/.local/share/chartodo/repeating_tasks.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/repeating_tasks.json";
        let mac_path = "/Library/Application Support/chartodo/repeating_tasks.json";
        let mut got_repeating_tasks_path: bool = false;
        let repeating_path = path_to_repeating_tasks();
        let repeating_path = repeating_path.to_str().unwrap();

        if repeating_path.contains(linux_path)
            | repeating_path.contains(windows_path)
            | repeating_path.contains(mac_path)
        {
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

        if repeating_tasks_copy_path.contains(linux_path)
            | repeating_tasks_copy_path.contains(windows_path)
            | repeating_tasks_copy_path.contains(mac_path)
        {
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

mod repeating_done_rmdone {
    use super::*;

    #[test]
    fn rmdone_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-rmdone");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-rmdone",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-rmd");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-rmdone",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_empty_done() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-rmdone").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_empty_done() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-rmd").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating",
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
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-rmdone")
            .arg("0")
            .arg("a")
            .arg("2")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were \
            all either negative, zero, or exceeded the repeating done list's length.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating",
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
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-rmd").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were \
            all either negative, zero, or exceeded the repeating done list's length.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_should_do_repeating_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
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
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-rmdone")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You want to remove all of the finished tasks in a \
            relatively long repeating done list. You should do repeating-cleardone.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_should_do_repeating_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "repeating",
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
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-rmd")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You want to remove all of the finished tasks in a \
            relatively long repeating done list. You should do repeating-cleardone.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-19",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-05",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-rmdone").arg("1").arg("1");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-task-1"))
            .is_err());

        Ok(())
    }

    #[test]
    fn rmdone_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-19",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-05",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-rmd").arg("1").arg("1");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-task-1"))
            .is_err());

        Ok(())
    }

    #[test]
    fn rmdone_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task",
                        "date": "2099-01-06",
                        "time": "00:00",
                        "repeat_number": 5,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-3",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-4",
                        "date": "2099-01-03",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-5",
                        "date": "2099-01-02",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "day",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-rmdone")
            .arg("1")
            .arg("1")
            .arg("3")
            .arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("2: repeating-task-2"));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task",
                        "date": "2099-01-06",
                        "time": "00:00",
                        "repeat_number": 5,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-3",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-4",
                        "date": "2099-01-03",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-5",
                        "date": "2099-01-02",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "day",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-rmd").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("2: repeating-task-2"));

        Ok(())
    }
}

mod repeating_done_notdone {
    use super::*;

    #[test]
    fn notdone_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-notdone");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-notdone",
        ));

        Ok(())
    }

    #[test]
    fn notdone_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-nd");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-notdone",
        ));

        Ok(())
    }

    #[test]
    fn notdone_empty_done() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-notdone").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_abrev_empty_done() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-nd").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-notdone")
            .arg("0")
            .arg("a")
            .arg("2")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were \
            all either negative, zero, or exceeded the repeating done list's length.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-nd").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were \
            all either negative, zero, or exceeded the repeating done list's length.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_should_do_repeating_notdoneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-notdone")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You specified an entire done list that's \
            relatively long. You should do repeating-notdoneall.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_abrev_should_do_repeating_notdoneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-nd")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You specified an entire done list that's \
            relatively long. You should do repeating-notdoneall.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-notdone").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-1"))
            .stdout(predicate::str::contains("1: repeating-task-2"));

        Ok(())
    }

    #[test]
    fn notdone_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-nd").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-1"))
            .stdout(predicate::str::contains("1: repeating-task-2"));

        Ok(())
    }

    #[test]
    fn notdone_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-06",
                        "time": "00:00",
                        "repeat_number": 5,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-3",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-4",
                        "date": "2099-01-03",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-5",
                        "date": "2099-01-02",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "day",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-notdone")
            .arg("1")
            .arg("1")
            .arg("3")
            .arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("2: repeating-task-3"))
            .stdout(predicate::str::contains("3: repeating-task-1"));

        Ok(())
    }

    #[test]
    fn notdone_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-06",
                        "time": "00:00",
                        "repeat_number": 5,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-3",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-4",
                        "date": "2099-01-03",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-5",
                        "date": "2099-01-02",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "day",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-nd").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("2: repeating-task-3"))
            .stdout(predicate::str::contains("3: repeating-task-1"));

        Ok(())
    }
}

mod repeating_done_cleardone {
    use super::*;

    #[test]
    fn cleardone_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-cleardone").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleardone_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-cd").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleardone_empty_done() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-cleardone");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn cleardone_abrev_empty_done() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-cd");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn cleardone_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-cleardone");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-task-1"))
            .is_err());

        Ok(())
    }

    #[test]
    fn cleardone_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-cd");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-task-1"))
            .is_err());

        Ok(())
    }
}

mod repeating_done_notdoneall {
    use super::*;

    #[test]
    fn notdoneall_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-notdoneall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn notdoneall_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-nda").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn notdoneall_empty_done() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-notdoneall");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn notdoneall_abrev_empty_done() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-nda");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn notdoneall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-06",
                        "time": "00:00",
                        "repeat_number": 5,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-3",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-4",
                        "date": "2099-01-03",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-5",
                        "date": "2099-01-02",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "day",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-rmd").arg("1");
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-notdoneall");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("2: repeating-task-4"))
            .stdout(predicate::str::contains("3: repeating-task-3"))
            .stdout(predicate::str::contains("4: repeating-task-2"))
            .stdout(predicate::str::contains("5: repeating-task-1"));

        Ok(())
    }

    #[test]
    fn notdoneall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "repeating-task-1",
                        "date": "2099-01-06",
                        "time": "00:00",
                        "repeat_number": 5,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-3",
                        "date": "2099-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-4",
                        "date": "2099-01-03",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-5",
                        "date": "2099-01-02",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "day",
                        "repeat_done": false,
                        "repeat_original_date": "2099-01-01",
                        "repeat_original_time": "00:00"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks)
            .context(
                "during testing: the fresh data to put in the new repeating_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-rmd").arg("1");
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-nda");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("2: repeating-task-4"))
            .stdout(predicate::str::contains("3: repeating-task-3"))
            .stdout(predicate::str::contains("4: repeating-task-2"))
            .stdout(predicate::str::contains("5: repeating-task-1"));

        Ok(())
    }
}

mod zzz_do_this_last {
    use super::*;

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
