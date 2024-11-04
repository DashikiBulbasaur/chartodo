use anyhow::Context;
use assert_cmd::prelude::*;
use chartodo::functions::{deadline_tasks::deadline_helpers::*, json_file_structs::*};
use predicates::prelude::*;
use std::{path::PathBuf, process::Command};

// cargo test --test deadline_todo_tests -- --test-threads=1

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
}

mod deadline_todo_add {
    use super::*;

    #[test]
    fn adding_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-add");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline add argument",
        ));

        Ok(())
    }

    #[test]
    fn adding_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-a");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline add argument",
        ));

        Ok(())
    }

    #[test]
    fn adding_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-add").arg("1");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You don't have the right amount of arguments when adding a deadline task.\n\tThere should be 3, 6, 9, etc. (i.e., divisible by 3) arguments after 'chartodo deadline-add'. You provided 1 argument(s).\n\tFormat: chartodo deadline-add ~task ~date ~time [...].\n\t\tDate must be in a yy-mm-dd format. Time must be in a 24-hour format.\n\tExample: chartodo dl-a new-item 2099-01-01 00:00\n\tAnother example: chartodo dl-a new-item 2099-01-01 00:00 another-item 2199-01-01 23:59"));

        Ok(())
    }

    #[test]
    fn adding_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-a").arg("1").arg("2").arg("3").arg("4");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You don't have the right amount of arguments when adding a deadline task.\n\tThere should be 3, 6, 9, etc. (i.e., divisible by 3) arguments after 'chartodo deadline-add'. You provided 4 argument(s).\n\tFormat: chartodo deadline-add ~task ~date ~time [...].\n\t\tDate must be in a yy-mm-dd format. Time must be in a 24-hour format.\n\tExample: chartodo dl-a new-item 2099-01-01 00:00\n\tAnother example: chartodo dl-a new-item 2099-01-01 00:00 another-item 2199-01-01 23:59"));

        Ok(())
    }

    #[test]
    fn adding_incorrect_time() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-add")
            .arg("deadline-task")
            .arg("2020-01-01")
            .arg("25:28");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: Your specified time for a new deadline task in argument set 1, '25:28', was invalid. Please provide a correct time in a 24-hour format, e.g. 20:05."));

        Ok(())
    }

    #[test]
    fn adding_abrev_incorrect_time() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-a")
            .arg("deadline-task")
            .arg("2020-01-01")
            .arg("13:28")
            .arg("deadline-task-2")
            .arg("2020-01-01")
            .arg("25:28");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: Your specified time for a new deadline task in argument set 2, '25:28', was invalid. Please provide a correct time in a 24-hour format, e.g. 20:05."));

        Ok(())
    }

    #[test]
    fn adding_incorrect_date() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-add")
            .arg("deadline-task")
            .arg("2020-13-01")
            .arg("13:28");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: Your specified date for a new deadline task in argument set 1, '2020-13-01', was invalid. Please provide a correct time in a year-month-day format, e.g. 2099-12-12."));

        Ok(())
    }

    #[test]
    fn adding_abrev_incorrect_date() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-a")
            .arg("deadline-task")
            .arg("2020-12-01")
            .arg("13:28")
            .arg("deadline-task-2")
            .arg("2020-13-01")
            .arg("13:28");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: Your specified date for a new deadline task in argument set 2, '2020-13-01', was invalid. Please provide a correct time in a year-month-day format, e.g. 2099-12-12."));

        Ok(())
    }

    #[test]
    fn adding_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-add")
            .arg("deadline-task")
            .arg("2020-01-01")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("MISSED: 2020-01-01 00:00"));

        Ok(())
    }

    #[test]
    fn adding_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-a")
            .arg("deadline-task")
            .arg("2099-12-13")
            .arg("14:37");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("due: 2099-12-13 14:37"));

        Ok(())
    }

    #[test]
    fn adding_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-add")
            .arg("deadline-task")
            .arg("2020-01-01")
            .arg("00:00")
            .arg("deadline-task-2")
            .arg("2099-12-13")
            .arg("12:01");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("MISSED: 2020-01-01 00:00"))
            .stdout(predicate::str::contains("2: deadline-task-2"))
            .stdout(predicate::str::contains("due: 2099-12-13 12:01"));

        Ok(())
    }

    #[test]
    fn adding_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-a")
            .arg("deadline-task")
            .arg("2099-12-13")
            .arg("14:37")
            .arg("deadline-task-2")
            .arg("2020-01-01")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-2"))
            .stdout(predicate::str::contains("MISSED: 2020-01-01 00:00"))
            .stdout(predicate::str::contains("2: deadline-task"))
            .stdout(predicate::str::contains("due: 2099-12-13 14:37"));

        Ok(())
    }
}

mod deadline_todo_add_no_time {
    use super::*;

    #[test]
    fn add_no_time_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-addonlydate");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline-addonlydate argument",
        ));

        Ok(())
    }

    #[test]
    fn add_no_time_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-aod");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline-addonlydate argument",
        ));

        Ok(())
    }

    #[test]
    fn add_no_time_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-addonlydate").arg("1");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You don't have the right amount of arguments when adding a deadline task w/ no time.\n\tThere should be 2, 4, 6, etc. (i.e., divisible by 2) arguments after 'chartodo deadline-addonlydate'. You provided 1 argument(s).\n\tFormat: chartodo deadline-addonlydate ~task ~date [...].\n\t\tDate must be in a yy-mm-dd format. The time defaults to 00:00.\n\tExample: chartodo dl-aod new-item 2099-01-01\n\tAnother example: chartodo dl-aod new-item 2099-01-01 another-item 2199-01-01"));

        Ok(())
    }

    #[test]
    fn add_no_time_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-aod").arg("1").arg("2").arg("3");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You don't have the right amount of arguments when adding a deadline task w/ no time.\n\tThere should be 2, 4, 6, etc. (i.e., divisible by 2) arguments after 'chartodo deadline-addonlydate'. You provided 3 argument(s).\n\tFormat: chartodo deadline-addonlydate ~task ~date [...].\n\t\tDate must be in a yy-mm-dd format. The time defaults to 00:00.\n\tExample: chartodo dl-aod new-item 2099-01-01\n\tAnother example: chartodo dl-aod new-item 2099-01-01 another-item 2199-01-01"));

        Ok(())
    }

    #[test]
    fn add_no_time_incorrect_date() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-addonlydate")
            .arg("deadline-task")
            .arg("2020-13-01");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: Your specified date in argument set 1, '2020-13-01', was invalid. Please provide a correct time in a year-month-day format, e.g. 2099-12-12."));

        Ok(())
    }

    #[test]
    fn add_no_time_abrev_incorrect_date() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-aod")
            .arg("deadline-task")
            .arg("2020-12-01")
            .arg("deadline-task-2")
            .arg("2020-13-01");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: Your specified date in argument set 2, '2020-13-01', was invalid. Please provide a correct time in a year-month-day format, e.g. 2099-12-12."));

        Ok(())
    }

    #[test]
    fn add_no_time_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-addonlydate")
            .arg("deadline-task")
            .arg("2020-01-01");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("MISSED: 2020-01-01 00:00"));

        Ok(())
    }

    #[test]
    fn add_no_time_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-aod").arg("deadline-task").arg("2099-12-13");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("due: 2099-12-13 00:00"));

        Ok(())
    }

    #[test]
    fn add_no_time_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-addonlydate")
            .arg("deadline-task")
            .arg("2020-01-01")
            .arg("deadline-task-2")
            .arg("2099-12-13");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("MISSED: 2020-01-01 00:00"))
            .stdout(predicate::str::contains("2: deadline-task-2"))
            .stdout(predicate::str::contains("due: 2099-12-13 00:00"));

        Ok(())
    }

    #[test]
    fn add_no_time_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-aod")
            .arg("deadline-task")
            .arg("2099-12-13")
            .arg("deadline-task-2")
            .arg("2020-01-01");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-2"))
            .stdout(predicate::str::contains("MISSED: 2020-01-01 00:00"))
            .stdout(predicate::str::contains("2: deadline-task"))
            .stdout(predicate::str::contains("due: 2099-12-13 00:00"));

        Ok(())
    }
}

mod deadline_todo_add_no_date {
    use super::*;

    #[test]
    fn add_no_date_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-addonlytime");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline-addonlytime argument",
        ));

        Ok(())
    }

    #[test]
    fn add_no_date_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-aot");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline-addonlytime argument",
        ));

        Ok(())
    }

    #[test]
    fn add_no_date_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-addonlytime").arg("1");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You don't have the right amount of arguments when adding a deadline task w/ no time.\n\tThere should be 2, 4, 6, etc. (i.e., divisible by 2) arguments after 'chartodo deadline-addonlytime'. You provided 1 argument(s).\n\tFormat: chartodo deadline-addonlytime ~task ~time [...].\n\t\tTime must be in a 24-hour format. The date defaults to your current date.\n\tExample: chartodo dl-aot new-item 00:00\n\tAnother example: chartodo dl-aot new-item 23:59 another-item 23:59"));

        Ok(())
    }

    #[test]
    fn add_no_date_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-aot").arg("1").arg("2").arg("3");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You don't have the right amount of arguments when adding a deadline task w/ no time.\n\tThere should be 2, 4, 6, etc. (i.e., divisible by 2) arguments after 'chartodo deadline-addonlytime'. You provided 3 argument(s).\n\tFormat: chartodo deadline-addonlytime ~task ~time [...].\n\t\tTime must be in a 24-hour format. The date defaults to your current date.\n\tExample: chartodo dl-aot new-item 00:00\n\tAnother example: chartodo dl-aot new-item 23:59 another-item 23:59"));

        Ok(())
    }

    #[test]
    fn add_no_date_incorrect_time() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-addonlytime")
            .arg("deadline-task")
            .arg("25:28");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: Your specified time for a new deadline task in argument set 1, '25:28', was invalid. Please provide a correct time in a 24-hour format, e.g. 20:05."));

        Ok(())
    }

    #[test]
    fn add_no_date_abrev_incorrect_time() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-aot")
            .arg("deadline-task")
            .arg("13:28")
            .arg("deadline-task-2")
            .arg("25:28");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: Your specified time for a new deadline task in argument set 2, '25:28', was invalid. Please provide a correct time in a 24-hour format, e.g. 20:05."));

        Ok(())
    }

    // i think it's possible to check the date and some other elements
    #[test]
    fn add_no_date_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-addonlytime")
            .arg("deadline-task")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("00:00"));

        Ok(())
    }

    #[test]
    fn add_no_date_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-aot").arg("deadline-task").arg("14:37");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("14:37"));

        Ok(())
    }

    #[test]
    fn add_no_date_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-addonlytime")
            .arg("deadline-task")
            .arg("00:00")
            .arg("deadline-task-2")
            .arg("12:01");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("00:00"))
            .stdout(predicate::str::contains("2: deadline-task-2"))
            .stdout(predicate::str::contains("12:01"));

        Ok(())
    }

    #[test]
    fn add_no_date_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-aot")
            .arg("deadline-task")
            .arg("14:37")
            .arg("deadline-task-2")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-2"))
            .stdout(predicate::str::contains("00:00"))
            .stdout(predicate::str::contains("2: deadline-task"))
            .stdout(predicate::str::contains("14:37"));

        Ok(())
    }
}

mod deadline_todo_done {
    use super::*;

    #[test]
    fn done_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-done");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline-done argument",
        ));

        Ok(())
    }

    #[test]
    fn done_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-d");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline-done argument",
        ));

        Ok(())
    }

    #[test]
    fn done_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-done").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn done_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-d").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn done_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-done").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the deadline todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn done_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-d").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the deadline todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn done_should_do_deadline_doneall() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-done")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire list. Might as well do chartodo deadline-doneall",
        ));

        Ok(())
    }

    #[test]
    fn done_abrev_should_do_deadline_doneall() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-d")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire list. Might as well do chartodo deadline-doneall",
        ));

        Ok(())
    }

    #[test]
    fn done_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-done").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("1: deadline-task-2"));

        Ok(())
    }

    #[test]
    fn done_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-d").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task"))
            .stdout(predicate::str::contains("1: deadline-task-2"));

        Ok(())
    }

    #[test]
    fn done_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-done").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-5"))
            .stdout(predicate::str::contains("2: deadline-task-3"))
            .stdout(predicate::str::contains("3: deadline-task"))
            .stdout(predicate::str::contains("1: deadline-task-4"))
            .stdout(predicate::str::contains("2: deadline-task-2"));

        Ok(())
    }

    #[test]
    fn done_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-d").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-5"))
            .stdout(predicate::str::contains("2: deadline-task-3"))
            .stdout(predicate::str::contains("3: deadline-task"))
            .stdout(predicate::str::contains("1: deadline-task-4"))
            .stdout(predicate::str::contains("2: deadline-task-2"));

        Ok(())
    }
}

mod deadline_todo_rmtodo {
    use super::*;

    #[test]
    fn rmtodo_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmtodo");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline-rmtodo argument",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmt");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide a deadline-rmtodo argument",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-rmtodo").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-rmt").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmtodo")
            .arg("0")
            .arg("a")
            .arg("2")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the deadline todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmt").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the deadline todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_should_do_deadline_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmtodo")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You might as well do deadline-cleartodo since you want to remove all of the items.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_should_do_deadline_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmt")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You might as well do deadline-cleartodo since you want to remove all of the items.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-hello",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmtodo").arg("1").arg("1");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-task-hello"))
            .is_err());

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-hello",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmt").arg("1").arg("1");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-task-hello"))
            .is_err());

        Ok(())
    }

    #[test]
    fn rmtodo_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-rmtodo")
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
    fn rmtodo_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-rmt").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-4"))
            .stdout(predicate::str::contains("2: deadline-task-2"));

        Ok(())
    }
}

mod deadline_todo_cleartodo {
    use super::*;

    #[test]
    fn cleartodo_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-cleartodo").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleartodo_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ct").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleartodo_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-cleartodo");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty. Try adding items to it first before removing any.",
        ));

        Ok(())
    }

    #[test]
    fn cleartodo_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-ct");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty. Try adding items to it first before removing any.",
        ));

        Ok(())
    }

    #[test]
    fn cleartodo_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-cleartodo");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-task-hello"))
            .is_err());

        Ok(())
    }

    #[test]
    fn cleartodo_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ct");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: deadline-task-hello"))
            .is_err());

        Ok(())
    }
}

mod deadline_todo_doneall {
    use super::*;

    #[test]
    fn doneall_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-doneall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn doneall_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-da").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn doneall_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-doneall");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so you can't change any todos to done.",
        ));

        Ok(())
    }

    #[test]
    fn doneall_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-da");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so you can't change any todos to done.",
        ));

        Ok(())
    }

    #[test]
    fn doneall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-doneall");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-2"))
            .stdout(predicate::str::contains("2: deadline-task-1"));

        Ok(())
    }

    #[test]
    fn doneall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-da");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: deadline-task-2"))
            .stdout(predicate::str::contains("2: deadline-task-1"));

        Ok(())
    }
}

mod deadline_todo_edit_all {
    use super::*;

    #[test]
    fn editall_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editall");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-editall",
        ));

        Ok(())
    }

    #[test]
    fn editall_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ea");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-editall",
        ));

        Ok(())
    }

    #[test]
    fn editall_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-editall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn editall_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-ea").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn editall_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position and all the parameters that will be edited.\n\tThere should be 4 arguments after 'chartodo deadline-editall'. You provided 1 argument(s).\n\tFormat: chartodo deadline-editall ~position ~task ~date ~time\n\t\tDate must be in a yy-mm-dd format. Time must be in a 24-hour format.\n\tExample: chartodo dl-ea 4 new-item 2150-01-01 00:00"));

        Ok(())
    }

    #[test]
    fn editall_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ea").arg("1").arg("2").arg("3");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position and all the parameters that will be edited.\n\tThere should be 4 arguments after 'chartodo deadline-editall'. You provided 3 argument(s).\n\tFormat: chartodo deadline-editall ~position ~task ~date ~time\n\t\tDate must be in a yy-mm-dd format. Time must be in a 24-hour format.\n\tExample: chartodo dl-ea 4 new-item 2150-01-01 00:00"));

        Ok(())
    }

    #[test]
    fn editall_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editall")
            .arg("a")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn editall_abrev_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ea")
            .arg("a")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn editall_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editall")
            .arg("0")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn editall_abrev_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ea")
            .arg("0")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn editall_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editall")
            .arg("2")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceed's the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn editall_abrev_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ea")
            .arg("2")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceed's the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn editall_date_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editall")
            .arg("1")
            .arg("edited-task")
            .arg("2099-13-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The date provided, '2099-13-13', isn't proper. It must be in a yy-mm-dd format, e.g., 2001-12-13",
        ));

        Ok(())
    }

    #[test]
    fn editall_abrev_date_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ea")
            .arg("1")
            .arg("edited-task")
            .arg("2099-13-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The date provided, '2099-13-13', isn't proper. It must be in a yy-mm-dd format, e.g., 2001-12-13",
        ));

        Ok(())
    }

    #[test]
    fn editall_time_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editall")
            .arg("1")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The time provided, '24:37', isn't proper. It must be in a 24-hour format, e.g., 23:08",
        ));

        Ok(())
    }

    #[test]
    fn editall_abrev_time_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ea")
            .arg("1")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The time provided, '24:37', isn't proper. It must be in a 24-hour format, e.g., 23:08",
        ));

        Ok(())
    }

    #[test]
    fn editall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editall")
            .arg("1")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("14:37");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: edited-task"))
            .stdout(predicate::str::contains("due: 2099-12-13 14:37"));

        Ok(())
    }

    #[test]
    fn editall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ea")
            .arg("1")
            .arg("edited-task")
            .arg("2099-12-13")
            .arg("14:37");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: edited-task"))
            .stdout(predicate::str::contains("due: 2099-12-13 14:37"));

        Ok(())
    }
}

mod deadline_todo_edit_task {
    use super::*;

    #[test]
    fn edittask_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittask");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-edittask",
        ));

        Ok(())
    }

    #[test]
    fn edittask_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eta");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-edittask",
        ));

        Ok(())
    }

    #[test]
    fn edittask_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-edittask").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn edittask_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-eta").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn edittask_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittask").arg("1");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position that will be edited and what to edit the task to.\n\tThere should be 2 arguments after 'chartodo deadline-edittask'. You provided 1 argument(s).\n\tFormat: chartodo deadline-edittask ~position ~task.\n\tExample: chartodo dl-eta 4 new-item"));

        Ok(())
    }

    #[test]
    fn edittask_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eta").arg("1").arg("2").arg("3");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position that will be edited and what to edit the task to.\n\tThere should be 2 arguments after 'chartodo deadline-edittask'. You provided 3 argument(s).\n\tFormat: chartodo deadline-edittask ~position ~task.\n\tExample: chartodo dl-eta 4 new-item"));

        Ok(())
    }

    #[test]
    fn edittask_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittask").arg("a").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn edittask_abrev_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eta").arg("a").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn edittask_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittask").arg("0").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn edittask_abrev_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eta").arg("0").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn edittask_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittask").arg("2").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceed's the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn edittask_abrev_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eta").arg("2").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceed's the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn edittask_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittask").arg("1").arg("edited-task");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: edited-task"));

        Ok(())
    }

    #[test]
    fn edittask_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eta").arg("1").arg("edited-task");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: edited-task"));

        Ok(())
    }
}

mod deadline_todo_edit_date {
    use super::*;

    #[test]
    fn editdate_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdate");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-editdate",
        ));

        Ok(())
    }

    #[test]
    fn editdate_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ed");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-editdate",
        ));

        Ok(())
    }

    #[test]
    fn editdate_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-editdate").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn editdate_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-ed").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn editdate_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdate").arg("1");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position that will be edited and what to edit the date to.\n\tThere should be two arguments after 'chartodo deadline-editdate'. You provided 1 argument(s).\n\tFormat: chartodo deadline-editdate ~position ~date.\n\t\tDate must be in a yy-mm-dd format.\n\tExample: chartodo dl-ed 4 2150-01-01"));

        Ok(())
    }

    #[test]
    fn editdate_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ed").arg("1").arg("2").arg("3");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position that will be edited and what to edit the date to.\n\tThere should be two arguments after 'chartodo deadline-editdate'. You provided 3 argument(s).\n\tFormat: chartodo deadline-editdate ~position ~date.\n\t\tDate must be in a yy-mm-dd format.\n\tExample: chartodo dl-ed 4 2150-01-01"));

        Ok(())
    }

    #[test]
    fn editdate_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdate").arg("a").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn editdate_abrev_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ed").arg("a").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn editdate_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdate").arg("0").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn editdate_abrev_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ed").arg("0").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn editdate_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdate").arg("2").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceeds the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn editdate_abrev_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ed").arg("2").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceeds the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn editdate_date_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdate").arg("1").arg("2099-13-13");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The date provided, '2099-13-13', isn't proper. It must be in a yy-mm-dd format, e.g., 2021-12-24.",
        ));

        Ok(())
    }

    #[test]
    fn editdate_abrev_date_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ed").arg("1").arg("2099-13-13");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The date provided, '2099-13-13', isn't proper. It must be in a yy-mm-dd format, e.g., 2021-12-24.",
        ));

        Ok(())
    }

    #[test]
    fn editdate_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdate").arg("1").arg("2099-12-13");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("due: 2099-12-13"));

        Ok(())
    }

    #[test]
    fn editdate_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-ed").arg("1").arg("2099-12-13");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("due: 2099-12-13"));

        Ok(())
    }
}

mod deadline_todo_edit_time {
    use super::*;

    #[test]
    fn edittime_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittime");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-edittime",
        ));

        Ok(())
    }

    #[test]
    fn edittime_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eti");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-edittime",
        ));

        Ok(())
    }

    #[test]
    fn edittime_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-edittime").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn edittime_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-eti").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn edittime_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittime").arg("1");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position that will be edited and what to edit the time to.\n\tThere should be 2 arguments after 'chartodo deadline-edittime'. You provided 1 argument(s).\n\tFormat: chartodo deadline-edittime ~position ~time.\n\t\tTime must be in a 24-hour format.\n\tExample: chartodo dl-eti 4 23:59"));

        Ok(())
    }

    #[test]
    fn edittime_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eti").arg("1").arg("2").arg("3");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position that will be edited and what to edit the time to.\n\tThere should be 2 arguments after 'chartodo deadline-edittime'. You provided 3 argument(s).\n\tFormat: chartodo deadline-edittime ~position ~time.\n\t\tTime must be in a 24-hour format.\n\tExample: chartodo dl-eti 4 23:59"));

        Ok(())
    }

    #[test]
    fn edittime_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittime").arg("a").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn edittime_abrev_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eti").arg("a").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn edittime_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittime").arg("0").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn edittime_abrev_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eti").arg("0").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn edittime_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittime").arg("2").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceeds the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn edittime_abrev_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eti").arg("2").arg("edited-task");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceeds the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn edittime_time_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittime").arg("1").arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The time provided, '24:37', isn't proper. It must be in a 24-hour format, e.g., 23:08",
        ));

        Ok(())
    }

    #[test]
    fn edittime_abrev_time_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eti").arg("1").arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The time provided, '24:37', isn't proper. It must be in a 24-hour format, e.g., 23:08",
        ));

        Ok(())
    }

    #[test]
    fn edittime_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
                        "date": "2021-01-01",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-edittime").arg("1").arg("14:37");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("MISSED: 2021-01-01 14:37"));

        Ok(())
    }

    #[test]
    fn edittime_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
                        "date": "2021-01-01",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-eti").arg("1").arg("14:37");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("MISSED: 2021-01-01 14:37"));

        Ok(())
    }
}

mod deadline_todo_edit_datetime {
    use super::*;

    #[test]
    fn editdatetime_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdatetime");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-editdatetime",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-edt");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for deadline-editdatetime",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("deadline-editdatetime").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("dl-edt").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The deadline todo list is currently empty, so there are no todos that can be edited.",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdatetime").arg("1");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position and what to edit the datetime to.\n\tThere should be 3 arguments after 'chartodo deadline-editdatetime'. You provided 1 argument(s).\n\tFormat: chartodo deadline-editdatetime ~position ~date ~time.\n\t\tDate should be in a yy-mm-dd format. Time should be in a 24-hour format.\n\tExample: chartodo dl-edt 4 2150-01-01 00:00"));

        Ok(())
    }

    #[test]
    fn editdatetime_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-edt").arg("1").arg("2").arg("3").arg("4");
        cmd.assert().success().stdout(predicate::str::contains("ERROR: You must specify the deadline todo's position and what to edit the datetime to.\n\tThere should be 3 arguments after 'chartodo deadline-editdatetime'. You provided 4 argument(s).\n\tFormat: chartodo deadline-editdatetime ~position ~date ~time.\n\t\tDate should be in a yy-mm-dd format. Time should be in a 24-hour format.\n\tExample: chartodo dl-edt 4 2150-01-01 00:00"));

        Ok(())
    }

    #[test]
    fn editdatetime_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdatetime")
            .arg("a")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_abrev_position_not_usize() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-edt").arg("a").arg("2099-12-13").arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: 'a' isn't a valid position. Try something between 1 and 1.",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdatetime")
            .arg("0")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_abrev_position_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-edt").arg("0").arg("2099-12-13").arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdatetime")
            .arg("2")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceeds the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_abrev_position_not_in_range() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-edt").arg("2").arg("2099-12-13").arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your position, '2', exceeds the todo list's length. Try something between 1 and 1",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_date_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdatetime")
            .arg("1")
            .arg("2099-13-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: '2099-13-13' isn't a proper date in a yy-mm-dd format, e.g., 2100-12-24.",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_abrev_date_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-edt").arg("1").arg("2099-13-13").arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: '2099-13-13' isn't a proper date in a yy-mm-dd format, e.g., 2100-12-24.",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_time_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdatetime")
            .arg("1")
            .arg("2099-12-13")
            .arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: '24:37' isn't a proper time in a 24-hour format, e.g., 13:28",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_abrev_time_is_wrong() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-edt").arg("1").arg("2099-12-13").arg("24:37");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: '24:37' isn't a proper time in a 24-hour format, e.g., 13:28",
        ));

        Ok(())
    }

    #[test]
    fn editdatetime_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("deadline-editdatetime")
            .arg("1")
            .arg("2099-12-13")
            .arg("14:37");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("due: 2099-12-13 14:37"));

        Ok(())
    }

    #[test]
    fn editdatetime_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "deadline-task-1",
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

        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("dl-edt").arg("1").arg("2099-12-13").arg("14:37");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("due: 2099-12-13 14:37"));

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
