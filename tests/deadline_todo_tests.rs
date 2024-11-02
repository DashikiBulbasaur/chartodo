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

mod deadline_todo_done {}

mod deadline_todo_rmtodo {}

mod deadline_todo_cleartodo {}

mod deadline_todo_doneall {}

mod deadline_todo_edit_all {}

mod deadline_todo_edit_task {}

mod deadline_todo_edit_date {}

mod deadline_todo_edit_time {}

mod deadline_todo_edit_datetime {}

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
