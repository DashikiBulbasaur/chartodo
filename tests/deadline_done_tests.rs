use anyhow::Context;
use assert_cmd::prelude::*;
use chartodo::functions::{deadline_tasks::deadline_helpers::*, json_file_structs::*};
use predicates::prelude::*;
use std::{path::PathBuf, process::Command};

// cargo test --test deadline_done_tests -- --test-threads=1

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

mod aaa_do_this_first {
    use super::*;

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
}

mod deadline_done_rmdone {
    use super::*;

    #[test]
    fn rmdone_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmdone");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-rmdone",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmd");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-rmdone",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_empty_done() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmdone").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline done list is currently empty, so you can't \
            remove any items.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_empty_done() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmd").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline done list is currently empty, so you can't \
            remove any items.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmdone")
            .arg("0")
            .arg("a")
            .arg("2")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were \
            all either negative, zero, or exceeded the deadline done list's length.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmd").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were \
            all either negative, zero, or exceeded the deadline done list's length.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_should_do_deadline_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmdone")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You might as well do deadline-cleardone since you want to \
            remove all of the items.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_should_do_deadline_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmd")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You might as well do deadline-cleardone since you want to \
            remove all of the items.",
        ));

        Ok(())
    }

    #[test]
    fn rmdone_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-1",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmdone").arg("1").arg("1");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-task-1"))
            .is_err());

        Ok(())
    }

    #[test]
    fn rmdone_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-1",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmd").arg("1").arg("1");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-task-1"))
            .is_err());

        Ok(())
    }

    #[test]
    fn rmdone_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-1",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
                        "date": "2024-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-3",
                        "date": "2023-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-4",
                        "date": "2022-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-5",
                        "date": "2021-01-01",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmdone")
            .arg("1")
            .arg("1")
            .arg("3")
            .arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-4"))
            .stdout(predicate::str::contains("2: deadline-task-2"));

        Ok(())
    }

    #[test]
    fn rmdone_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
                        "date": "2024-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-3",
                        "date": "2023-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-4",
                        "date": "2022-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-5",
                        "date": "2021-01-01",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmd").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-4"))
            .stdout(predicate::str::contains("2: deadline-task-2"));

        Ok(())
    }
}

mod deadline_done_notdone {
    use super::*;

    #[test]
    fn notdone_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-notdone");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-notdone",
        ));

        Ok(())
    }

    #[test]
    fn notdone_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-nd");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-notdone",
        ));

        Ok(())
    }

    #[test]
    fn notdone_empty_done() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-notdone").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_abrev_empty_done() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-nd").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-notdone")
            .arg("0")
            .arg("a")
            .arg("2")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were \
            all either negative, zero, or exceeded the deadline done list's length.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-nd").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were \
            all either negative, zero, or exceeded the deadline done list's length.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_should_do_deadline_notdoneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-notdone")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You might as well do deadline-notdoneall since you want to \
            reverse all deadline done items.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_abrev_should_do_deadline_notdoneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-nd")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You might as well do deadline-notdoneall since you want to \
            reverse all deadline done items.",
        ));

        Ok(())
    }

    #[test]
    fn notdone_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-1",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-notdone").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-1"))
            .stdout(predicate::str::contains("1: deadline-task-2"));

        Ok(())
    }

    #[test]
    fn notdone_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-1",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-nd").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-1"))
            .stdout(predicate::str::contains("1: deadline-task-2"));

        Ok(())
    }

    #[test]
    fn notdone_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-1",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
                        "date": "2024-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-3",
                        "date": "2023-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-4",
                        "date": "2022-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-5",
                        "date": "2021-01-01",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-notdone")
            .arg("1")
            .arg("1")
            .arg("3")
            .arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-4"))
            .stdout(predicate::str::contains("2: deadline-task-2"))
            .stdout(predicate::str::contains("1: deadline-task-5"))
            .stdout(predicate::str::contains("2: deadline-task-3"))
            .stdout(predicate::str::contains("3: deadline-task-1"));

        Ok(())
    }

    #[test]
    fn notdone_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-1",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
                        "date": "2024-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-3",
                        "date": "2023-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-4",
                        "date": "2022-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-5",
                        "date": "2021-01-01",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-nd").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-4"))
            .stdout(predicate::str::contains("2: deadline-task-2"))
            .stdout(predicate::str::contains("1: deadline-task-5"))
            .stdout(predicate::str::contains("2: deadline-task-3"))
            .stdout(predicate::str::contains("3: deadline-task-1"));

        Ok(())
    }
}

mod deadline_done_cleardone {
    use super::*;

    #[test]
    fn cleardone_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-cleardone").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleardone_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-cd").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleardone_empty_done() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-cleardone");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn cleardone_abrev_empty_done() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-cd");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn cleardone_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-hello",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-cleardone");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-task-hello"))
            .is_err());

        Ok(())
    }

    #[test]
    fn cleardone_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-hello",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-cd");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-task-hello"))
            .is_err());

        Ok(())
    }
}

mod deadline_done_notdoneall {
    use super::*;

    #[test]
    fn notdoneall_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-notdoneall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn notdoneall_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-nda").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn notdoneall_empty_done() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-notdoneall");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn notdoneall_abrev_empty_done() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-nda");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline done list is currently empty.",
        ));

        Ok(())
    }

    #[test]
    fn notdoneall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-1",
                        "date": "2099-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
                        "date": "2098-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-3",
                        "date": "2097-01-01",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmd").arg("1");
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-notdoneall");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-3"))
            .stdout(predicate::str::contains("2: deadline-task-2"));

        Ok(())
    }

    #[test]
    fn notdoneall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "deadline-task-1",
                        "date": "2099-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-2",
                        "date": "2098-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "deadline-task-3",
                        "date": "2097-01-01",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmd").arg("1");
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-nda");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-3"))
            .stdout(predicate::str::contains("2: deadline-task-2"));

        Ok(())
    }
}

mod zzz_do_this_last {
    use super::*;

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
}
