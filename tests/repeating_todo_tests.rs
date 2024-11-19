use anyhow::Context;
use assert_cmd::prelude::*;
use chartodo::functions::{json_file_structs::*, repeating_tasks::repeating_helpers::*};
use predicates::prelude::*;
use std::{path::PathBuf, process::Command};

// cargo test --test repeating_todo_tests -- --test-threads=1

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

mod repeating_todo_add {
    use super::*;

    #[test]
    fn add_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-add");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-add",
        ));

        Ok(())
    }

    #[test]
    fn add_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-a");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-add",
        ));

        Ok(())
    }
    #[test]
    fn add_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-add").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You don't have the right amount of arguments when adding a repeating task.\n\tThere should be 3, 6, 9, etc. (i.e., divisible by 3) arguments after 'chartodo repeating-add'. You provided 1 argument(s).\n\tFormat: chartodo repeating-add ~task ~interval ~time-unit [...].\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\tExample: chartodo rp-a do-a-backflip 2 days.\n\tAnother example: chartodo rp-a new-item 3 days another-item 4 years",
        ));

        Ok(())
    }

    #[test]
    fn add_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-a").arg("1").arg("2").arg("3").arg("4");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You don't have the right amount of arguments when adding a repeating task.\n\tThere should be 3, 6, 9, etc. (i.e., divisible by 3) arguments after 'chartodo repeating-add'. You provided 4 argument(s).\n\tFormat: chartodo repeating-add ~task ~interval ~time-unit [...].\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\tExample: chartodo rp-a do-a-backflip 2 days.\n\tAnother example: chartodo rp-a new-item 3 days another-item 4 years",
        ));

        Ok(())
    }

    #[test]
    fn add_wrong_time_unit() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-a")
            .arg("repeating-task")
            .arg("3")
            .arg("seconds");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided time unit, 'seconds', in argument set '1', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.",
        ));

        Ok(())
    }

    #[test]
    fn add_abrev_wrong_time_unit() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-a")
            .arg("repeating-task")
            .arg("3")
            .arg("minutes")
            .arg("repeating-task-2")
            .arg("3")
            .arg("seconds");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided time unit, 'seconds', in argument set '2', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.",
        ));

        Ok(())
    }

    #[test]
    fn add_interval_not_u32() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-add")
            .arg("repeating-task")
            .arg("a")
            .arg("minutes")
            .arg("repeating-task-2")
            .arg("3")
            .arg("hours");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided interval, 'a', in argument set '1', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-a gym 2 days.",
        ));

        Ok(())
    }

    #[test]
    fn add_abrev_interval_not_u32() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-a")
            .arg("repeating-task")
            .arg("a")
            .arg("minutes");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided interval, 'a', in argument set '1', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-a gym 2 days.",
        ));

        Ok(())
    }

    #[test]
    fn add_interval_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-add")
            .arg("repeating-task")
            .arg("3")
            .arg("minutes")
            .arg("repeating-task-2")
            .arg("0")
            .arg("hours");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You had an interval of 0 in argument set '2'. You can't have an interval of 0, otherwise why are you even making a new repeating task?",
        ));

        Ok(())
    }

    #[test]
    fn add_abrev_interval_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-a")
            .arg("repeating-task")
            .arg("0")
            .arg("minutes")
            .arg("repeating-task-2")
            .arg("4")
            .arg("hours");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You had an interval of 0 in argument set '1'. You can't have an interval of 0, otherwise why are you even making a new repeating task?",
        ));

        Ok(())
    }

    #[test]
    fn add_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-add")
            .arg("repeating-task")
            .arg("25")
            .arg("minutes");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task"))
            .stdout(predicate::str::contains("interval: 25 minutes"));

        Ok(())
    }

    #[test]
    fn add_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-a")
            .arg("repeating-task")
            .arg("25")
            .arg("minutes");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task"))
            .stdout(predicate::str::contains("interval: 25 minutes"));

        Ok(())
    }

    #[test]
    fn add_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-add")
            .arg("repeating-task-1")
            .arg("25")
            .arg("minutes")
            .arg("repeating-task-2")
            .arg("24")
            .arg("minutes");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 24 minutes"))
            .stdout(predicate::str::contains("2: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 25 minutes"));

        Ok(())
    }

    #[test]
    fn add_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-a")
            .arg("repeating-task-1")
            .arg("25")
            .arg("minutes")
            .arg("repeating-task-2")
            .arg("24")
            .arg("minutes");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 24 minutes"))
            .stdout(predicate::str::contains("2: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 25 minutes"));

        Ok(())
    }
}

mod repeating_todo_add_start {
    use super::*;

    #[test]
    fn addstart_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addstart");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-addstart",
        ));

        Ok(())
    }

    #[test]
    fn addstart_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-as");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-addstart",
        ));

        Ok(())
    }
    #[test]
    fn addstart_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addstart").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific starting datetime.\n\tThere should be 5, 10, 15, etc. (i.e., divisible by 5) arguments after 'chartodo repeating-addstart'. You provided 1 argument(s).\n\tFormat: chartodo repeating-addstart ~task ~interval ~time-unit ~date ~time [...].\n\t\tDate should be in a yy-mm-dd format. Time should be in a 24-hour format.\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\tExample: chartodo rp-as new-item 3 days 2099-01-01 00:00.\n\tAnother example: chartodo rp-as new-item 3 days 2099-01-01 00:00 another-item 4 years 23:59",
        ));

        Ok(())
    }

    #[test]
    fn addstart_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-as").arg("1").arg("2").arg("3").arg("4");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific starting datetime.\n\tThere should be 5, 10, 15, etc. (i.e., divisible by 5) arguments after 'chartodo repeating-addstart'. You provided 4 argument(s).\n\tFormat: chartodo repeating-addstart ~task ~interval ~time-unit ~date ~time [...].\n\t\tDate should be in a yy-mm-dd format. Time should be in a 24-hour format.\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\tExample: chartodo rp-as new-item 3 days 2099-01-01 00:00.\n\tAnother example: chartodo rp-as new-item 3 days 2099-01-01 00:00 another-item 4 years 23:59",
        ));

        Ok(())
    }

    #[test]
    fn addstart_wrong_time() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addstart")
            .arg("repeating-task")
            .arg("3")
            .arg("days")
            .arg("2099-12-13")
            .arg("25:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided starting time, '25:00', in argument set '1', wasn't proper. Please provide a correct starting time in a 24-hour format, e.g., 23:04.",
        ));

        Ok(())
    }

    #[test]
    fn addstart_abrev_wrong_time() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-as")
            .arg("repeating-task")
            .arg("3")
            .arg("days")
            .arg("2099-12-13")
            .arg("23:59")
            .arg("repeating-task-2")
            .arg("4")
            .arg("weeks")
            .arg("2099-12-13")
            .arg("25:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided starting time, '25:00', in argument set '2', wasn't proper. Please provide a correct starting time in a 24-hour format, e.g., 23:04.",
        ));

        Ok(())
    }

    #[test]
    fn addstart_wrong_date() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addstart")
            .arg("repeating-task")
            .arg("3")
            .arg("days")
            .arg("2099-13-13")
            .arg("23:00")
            .arg("repeating-task-2")
            .arg("5")
            .arg("months")
            .arg("2099-12-13")
            .arg("23:59");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided starting date, '2099-13-13', in argument set '1', wasn't proper. Please provide a correct starting date in a year-month-day format, e.g., 2024-05-13.",
        ));

        Ok(())
    }

    #[test]
    fn addstart_abrev_wrong_date() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-as")
            .arg("repeating-task")
            .arg("3")
            .arg("days")
            .arg("2099-13-13")
            .arg("23:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided starting date, '2099-13-13', in argument set '1', wasn't proper. Please provide a correct starting date in a year-month-day format, e.g., 2024-05-13.",
        ));

        Ok(())
    }

    #[test]
    fn addstart_wrong_time_unit() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addstart")
            .arg("repeating-task")
            .arg("3")
            .arg("seconds")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided time unit, 'seconds', in argument set '1', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.",
        ));

        Ok(())
    }

    #[test]
    fn addstart_abrev_wrong_time_unit() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-as")
            .arg("repeating-task")
            .arg("3")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("23:08")
            .arg("repeating-task-2")
            .arg("3")
            .arg("seconds")
            .arg("2098-12-24")
            .arg("23:25");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided time unit, 'seconds', in argument set '2', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.",
        ));

        Ok(())
    }

    #[test]
    fn addstart_interval_not_u32() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addstart")
            .arg("repeating-task")
            .arg("a")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("14:15");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided interval, 'a', in argument set '1', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-a gym 2 days 2020-01-01 00:00",
        ));

        Ok(())
    }

    #[test]
    fn addstart_abrev_interval_not_u32() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-as")
            .arg("repeating-task")
            .arg("50")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("00:00")
            .arg("repeating-task-2")
            .arg("a")
            .arg("years")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided interval, 'a', in argument set '2', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-a gym 2 days 2020-01-01 00:00",
        ));

        Ok(())
    }

    #[test]
    fn addstart_interval_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addstart")
            .arg("repeating-task")
            .arg("3")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("20:00")
            .arg("repeating-task-2")
            .arg("0")
            .arg("hours")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You provided an interval of 0 in argument set '2'. You can't have an interval of 0, otherwise why are you even making a new repeating task?",
        ));

        Ok(())
    }

    #[test]
    fn addstart_abrev_interval_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-as")
            .arg("repeating-task")
            .arg("0")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("00:00")
            .arg("repeating-task-2")
            .arg("4")
            .arg("hours")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You provided an interval of 0 in argument set '1'. You can't have an interval of 0, otherwise why are you even making a new repeating task?",
        ));

        Ok(())
    }

    #[test]
    fn addstart_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-addstart")
            .arg("repeating-task")
            .arg("25")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-12-13 00:25"));

        Ok(())
    }

    #[test]
    fn addstart_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-as")
            .arg("repeating-task")
            .arg("25")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-12-13 00:25"));

        Ok(())
    }

    // note that tasks, after getting sorted for time, aren't sorted
    // alphabetically. if the times are equal, then tasks are added in the order
    // they were typed in
    #[test]
    fn addstart_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-addstart")
            .arg("repeating-task-2")
            .arg("25")
            .arg("minutes")
            .arg("2099-01-01")
            .arg("00:00")
            .arg("repeating-task-1")
            .arg("25")
            .arg("minutes")
            .arg("2099-01-01")
            .arg("00:01");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-01-01 00:25"))
            .stdout(predicate::str::contains("2: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-01-01 00:26"));

        Ok(())
    }

    #[test]
    fn addstart_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-as")
            .arg("repeating-task-2")
            .arg("25")
            .arg("minutes")
            .arg("2099-01-01")
            .arg("00:00")
            .arg("repeating-task-1")
            .arg("25")
            .arg("minutes")
            .arg("2099-01-01")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-01-01 00:25"))
            .stdout(predicate::str::contains("2: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-01-01 00:25"));

        Ok(())
    }
}

mod repeating_todo_add_end {
    use super::*;

    #[test]
    fn addend_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addend");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-addend",
        ));

        Ok(())
    }

    #[test]
    fn addend_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-ae");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-addend",
        ));

        Ok(())
    }
    #[test]
    fn addend_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addend").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific ending datetime.\n\tThere should be 5, 10, 15, etc. (i.e., divisible by 5) arguments after 'chartodo repeating-addend'. You provided 1 argument(s).\n\tFormat: chartodo repeating-addend ~task ~interval ~time-unit ~date ~time [...].\n\t\tDate must be in a yy-mm-format. Time must be in a 24-hour format.\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\tExample: chartodo rp-ae new-item 3 days 2099-01-01 00:00.\n\tAnother example: chartodo rp-ae new-item 3 days 2099-01-01 00:00 another-item 4 years 23:59",
        ));

        Ok(())
    }

    #[test]
    fn addend_abrev_wrong_num_of_args() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-ae").arg("1").arg("2").arg("3").arg("4");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific ending datetime.\n\tThere should be 5, 10, 15, etc. (i.e., divisible by 5) arguments after 'chartodo repeating-addend'. You provided 4 argument(s).\n\tFormat: chartodo repeating-addend ~task ~interval ~time-unit ~date ~time [...].\n\t\tDate must be in a yy-mm-format. Time must be in a 24-hour format.\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\tExample: chartodo rp-ae new-item 3 days 2099-01-01 00:00.\n\tAnother example: chartodo rp-ae new-item 3 days 2099-01-01 00:00 another-item 4 years 23:59",
        ));

        Ok(())
    }

    #[test]
    fn addend_wrong_time() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addend")
            .arg("repeating-task")
            .arg("3")
            .arg("days")
            .arg("2099-12-13")
            .arg("25:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided ending time, '25:00', in argument set '1', wasn't proper. Please provide a correct ending time in a 24-hour format, e.g., 23:04.",
        ));

        Ok(())
    }

    #[test]
    fn addend_abrev_wrong_time() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-ae")
            .arg("repeating-task")
            .arg("3")
            .arg("days")
            .arg("2099-12-13")
            .arg("23:59")
            .arg("repeating-task-2")
            .arg("4")
            .arg("weeks")
            .arg("2099-12-13")
            .arg("25:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided ending time, '25:00', in argument set '2', wasn't proper. Please provide a correct ending time in a 24-hour format, e.g., 23:04.",
        ));

        Ok(())
    }

    #[test]
    fn addend_wrong_date() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addend")
            .arg("repeating-task")
            .arg("3")
            .arg("days")
            .arg("2099-13-13")
            .arg("23:00")
            .arg("repeating-task-2")
            .arg("5")
            .arg("months")
            .arg("2099-12-13")
            .arg("23:59");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided ending date, '2099-13-13', in argument set '1', wasn't proper. Please provide a correct ending date in a year-month-day format, e.g., 2024-05-12.",
        ));

        Ok(())
    }

    #[test]
    fn addend_abrev_wrong_date() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-ae")
            .arg("repeating-task")
            .arg("3")
            .arg("days")
            .arg("2099-13-13")
            .arg("23:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided ending date, '2099-13-13', in argument set '1', wasn't proper. Please provide a correct ending date in a year-month-day format, e.g., 2024-05-12.",
        ));

        Ok(())
    }

    #[test]
    fn addend_wrong_time_unit() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addend")
            .arg("repeating-task")
            .arg("3")
            .arg("seconds")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided time unit, 'seconds', in argument set '1', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.",
        ));

        Ok(())
    }

    #[test]
    fn addend_abrev_wrong_time_unit() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-ae")
            .arg("repeating-task")
            .arg("3")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("23:08")
            .arg("repeating-task-2")
            .arg("3")
            .arg("seconds")
            .arg("2098-12-24")
            .arg("23:25");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided time unit, 'seconds', in argument set '2', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.",
        ));

        Ok(())
    }

    #[test]
    fn addend_interval_not_u32() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addend")
            .arg("repeating-task")
            .arg("a")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("14:15");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided interval, 'a', in argument set '1', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-ae gym 2 days 2000-01-01 00:00",
        ));

        Ok(())
    }

    #[test]
    fn addend_abrev_interval_not_u32() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-ae")
            .arg("repeating-task")
            .arg("50")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("00:00")
            .arg("repeating-task-2")
            .arg("a")
            .arg("years")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: Your provided interval, 'a', in argument set '2', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-ae gym 2 days 2000-01-01 00:00",
        ));

        Ok(())
    }

    #[test]
    fn addend_interval_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-addend")
            .arg("repeating-task")
            .arg("3")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("20:00")
            .arg("repeating-task-2")
            .arg("0")
            .arg("hours")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You provided an interval of 0 in argument set 2. You can't have an interval of 0, otherwise why are you even making a new repeating task?",
        ));

        Ok(())
    }

    #[test]
    fn addend_abrev_interval_is_zero() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-ae")
            .arg("repeating-task")
            .arg("0")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("00:00")
            .arg("repeating-task-2")
            .arg("4")
            .arg("hours")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: You provided an interval of 0 in argument set 1. You can't have an interval of 0, otherwise why are you even making a new repeating task?",
        ));

        Ok(())
    }

    #[test]
    fn addend_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-addend")
            .arg("repeating-task")
            .arg("25")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-12-13 00:00"));

        Ok(())
    }

    #[test]
    fn addend_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-ae")
            .arg("repeating-task")
            .arg("25")
            .arg("minutes")
            .arg("2099-12-13")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-12-13 00:00"));

        Ok(())
    }

    // note that tasks, after getting sorted for time, aren't sorted
    // alphabetically. if the times are equal, then tasks are added in the order
    // they were typed in
    #[test]
    fn addend_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-addend")
            .arg("repeating-task-2")
            .arg("25")
            .arg("minutes")
            .arg("2099-01-01")
            .arg("00:00")
            .arg("repeating-task-1")
            .arg("25")
            .arg("minutes")
            .arg("2099-01-01")
            .arg("00:01");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-01-01 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-01-01 00:01"));

        Ok(())
    }

    #[test]
    fn addend_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-ae")
            .arg("repeating-task-2")
            .arg("25")
            .arg("minutes")
            .arg("2099-01-01")
            .arg("00:00")
            .arg("repeating-task-1")
            .arg("25")
            .arg("minutes")
            .arg("2099-01-01")
            .arg("00:00");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-01-01 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 25 minutes"))
            .stdout(predicate::str::contains("due: 2099-01-01 00:00"));

        Ok(())
    }
}

mod repeating_todo_done {
    use super::*;

    #[test]
    fn done_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-done");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-done",
        ));

        Ok(())
    }

    #[test]
    fn done_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-d");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-done",
        ));

        Ok(())
    }

    #[test]
    fn done_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-done").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn done_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-d").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn done_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-done").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn done_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-d").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn done_should_do_repeating_doneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-done")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire repeating todo list that's relatively long. You should do chartodo repeating-doneall",
        ));

        Ok(())
    }

    #[test]
    fn done_abrev_should_do_repeating_doneall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-d")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire repeating todo list that's relatively long. You should do chartodo repeating-doneall",
        ));

        Ok(())
    }

    #[test]
    fn done_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2025-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("repeating-done").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("done: 2025-01-04 00:00"))
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("due: 2025-01-05 00:00"));

        Ok(())
    }

    #[test]
    fn done_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2025-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("rp-d").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("done: 2025-01-04 00:00"))
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("due: 2025-01-05 00:00"));

        Ok(())
    }

    #[test]
    fn done_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-done")
            .arg("1")
            .arg("1")
            .arg("3")
            .arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("done: 2099-01-02 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("done: 2099-01-04 00:00"))
            .stdout(predicate::str::contains("3: repeating-task"))
            .stdout(predicate::str::contains("interval: 5 days"))
            .stdout(predicate::str::contains("done: 2099-01-06 00:00"))
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("due: 2099-01-03 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("due: 2099-01-05 00:00"));

        Ok(())
    }

    #[test]
    fn done_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-d").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("done: 2099-01-02 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("done: 2099-01-04 00:00"))
            .stdout(predicate::str::contains("3: repeating-task"))
            .stdout(predicate::str::contains("interval: 5 days"))
            .stdout(predicate::str::contains("done: 2099-01-06 00:00"))
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("due: 2099-01-03 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("due: 2099-01-05 00:00"));

        Ok(())
    }
}

mod repeating_todo_reset {
    use super::*;

    #[test]
    fn reset_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-reset");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-reset/repeating-donereset",
        ));

        Ok(())
    }

    #[test]
    fn reset_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-r");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-reset/repeating-donereset",
        ));

        Ok(())
    }

    #[test]
    fn donereset_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-donereset");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-reset/repeating-donereset",
        ));

        Ok(())
    }

    #[test]
    fn donereset_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-dr");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-reset/repeating-donereset",
        ));

        Ok(())
    }

    #[test]
    fn reset_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-reset").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn reset_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-r").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn donereset_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-donereset").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn donereset_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-dr").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn reset_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-reset")
            .arg("0")
            .arg("a")
            .arg("2")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn reset_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-r").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn donereset_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-donereset")
            .arg("0")
            .arg("a")
            .arg("2")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn donereset_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-dr").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn reset_should_do_repeating_resetall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-reset")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire repeating todo list that's relatively long. You should do chartodo repeating-resetall/repeating-doneresetall",
        ));

        Ok(())
    }

    #[test]
    fn reset_abrev_should_do_repeating_resetall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-r")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire repeating todo list that's relatively long. You should do chartodo repeating-resetall/repeating-doneresetall",
        ));

        Ok(())
    }

    #[test]
    fn donereset_should_do_repeating_doneresetall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-donereset")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire repeating todo list that's relatively long. You should do chartodo repeating-resetall/repeating-doneresetall",
        ));

        Ok(())
    }

    #[test]
    fn donereset_abrev_should_do_repeating_doneresetall() -> Result<(), Box<dyn std::error::Error>>
    {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-dr")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire repeating todo list that's relatively long. You should do chartodo repeating-resetall/repeating-doneresetall",
        ));

        Ok(())
    }

    // might be impossible to check when it's due
    #[test]
    fn reset_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task-1",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2025-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("repeating-reset").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"));

        Ok(())
    }

    #[test]
    fn reset_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task-1",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2025-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("rp-r").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"));

        Ok(())
    }

    #[test]
    fn donereset_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task-1",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2025-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("repeating-donereset").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"));

        Ok(())
    }

    #[test]
    fn donereset_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task-1",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2025-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("rp-dr").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"));

        Ok(())
    }

    #[test]
    fn reset_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-reset")
            .arg("2")
            .arg("2")
            .arg("4")
            .arg("6");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("3: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("4: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("5: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"));

        Ok(())
    }

    #[test]
    fn reset_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-r").arg("2").arg("2").arg("4").arg("6");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("3: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("4: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("5: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"));

        Ok(())
    }

    #[test]
    fn donereset_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-donereset")
            .arg("2")
            .arg("2")
            .arg("4")
            .arg("6");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("3: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("4: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("5: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"));

        Ok(())
    }

    #[test]
    fn donereset_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-dr").arg("2").arg("2").arg("4").arg("6");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("3: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("4: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("5: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"));

        Ok(())
    }
}

mod repeating_todo_rmtodo {
    use super::*;

    #[test]
    fn rmtodo_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-rmtodo");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-rmtodo",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-rmt");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-rmtodo",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-rmtodo").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-rmt").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-rmtodo")
            .arg("0")
            .arg("a")
            .arg("2")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-rmt").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_should_do_repeating_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-rmtodo")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire repeating todo list, one that's relatively long. You should do repeating-cleartodo",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_should_do_repeating_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-rmt")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You've specified the entire repeating todo list, one that's relatively long. You should do repeating-cleartodo",
        ));

        Ok(())
    }

    #[test]
    fn rmtodo_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("repeating-rmtodo").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("due: 2099-01-05 00:00"));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("rp-rmt").arg("1").arg("1");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("due: 2099-01-05 00:00"));

        Ok(())
    }

    #[test]
    fn rmtodo_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-rmtodo")
            .arg("1")
            .arg("1")
            .arg("3")
            .arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("due: 2099-01-03 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("due: 2099-01-05 00:00"));

        Ok(())
    }

    #[test]
    fn rmtodo_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-rmt").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("due: 2099-01-03 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("due: 2099-01-05 00:00"));

        Ok(())
    }
}

mod repeating_todo_show_start {
    use super::*;

    #[test]
    fn start_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-start");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-start",
        ));

        Ok(())
    }

    #[test]
    fn start_abrev_nothing() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-s");
        cmd.assert().failure().stderr(predicate::str::contains(
            "didn't provide arguments for repeating-start",
        ));

        Ok(())
    }

    #[test]
    fn start_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-start").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn start_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-s").arg("1").arg("1").arg("2");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn start_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-start")
            .arg("0")
            .arg("a")
            .arg("2")
            .arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn start_abrev_no_valid_args() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-s").arg("0").arg("a").arg("2").arg("");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.",
        ));

        Ok(())
    }

    #[test]
    fn start_should_do_repeating_startall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-start")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You want to show the start times for an entire list that's relatively long, You should do repeating-startall",
        ));

        Ok(())
    }

    #[test]
    fn start_abrev_should_do_repeating_startall() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-s")
            .arg("1")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4")
            .arg("5")
            .arg("6");
        cmd.assert().success().stdout(predicate::str::contains(
            "WARNING: You want to show the start times for an entire list that's relatively long, You should do repeating-startall",
        ));

        Ok(())
    }

    #[test]
    fn start_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("repeating-start").arg("1").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "task: repeating-task\n\tstart: 2025-01-01 00:00",
        ));

        Ok(())
    }

    #[test]
    fn start_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "repeating-task",
                        "date": "2025-01-04",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "repeating-task-2",
                        "date": "2099-01-05",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "days",
                        "repeat_done": false,
                        "repeat_original_date": "2025-01-01",
                        "repeat_original_time": "00:00"
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
        cmd.arg("rp-s").arg("1").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "task: repeating-task\n\tstart: 2025-01-01 00:00",
        ));

        Ok(())
    }

    #[test]
    fn start_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-start")
            .arg("1")
            .arg("1")
            .arg("3")
            .arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("task: repeating-task-5\n\tstart: 2099-01-01 00:00\ntask: repeating-task-3\n\tstart: 2099-01-01 00:00\ntask: repeating-task\n\tstart: 2099-01-01 00:00"));

        Ok(())
    }

    #[test]
    fn start_abrev_multiple_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-s").arg("1").arg("1").arg("3").arg("5");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("task: repeating-task-5\n\tstart: 2099-01-01 00:00\ntask: repeating-task-3\n\tstart: 2099-01-01 00:00\ntask: repeating-task\n\tstart: 2099-01-01 00:00"));

        Ok(())
    }
}

mod repeating_todo_doneall {
    use super::*;

    #[test]
    fn doneall_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-doneall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn doneall_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-da").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn doneall_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-doneall");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty, so you can't change any todos to done.",
        ));

        Ok(())
    }

    #[test]
    fn doneall_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-da");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty, so you can't change any todos to done.",
        ));

        Ok(())
    }

    #[test]
    fn doneall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-rmt").arg("2");
        cmd.assert().success();
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-doneall");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("done: 2099-01-02 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("done: 2099-01-04 00:00"))
            .stdout(predicate::str::contains("3: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("done: 2099-01-06 00:00"))
            .stdout(predicate::str::contains("4: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"))
            .stdout(predicate::str::contains("done: 2099-01-05 00:00"));

        Ok(())
    }

    #[test]
    fn doneall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-rmt").arg("2");
        cmd.assert().success();
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-da");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("done: 2099-01-02 00:00"))
            .stdout(predicate::str::contains("2: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("done: 2099-01-04 00:00"))
            .stdout(predicate::str::contains("3: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("done: 2099-01-06 00:00"))
            .stdout(predicate::str::contains("4: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"))
            .stdout(predicate::str::contains("done: 2099-01-05 00:00"));

        Ok(())
    }
}

mod repeating_todo_cleartodo {
    use super::*;

    #[test]
    fn cleartodo_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-cleartodo").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleartodo_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-ct").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn cleartodo_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-cleartodo");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first before removing any.",
        ));

        Ok(())
    }

    #[test]
    fn cleartodo_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-ct");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first before removing any.",
        ));

        Ok(())
    }

    #[test]
    fn cleartodo_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-cleartodo");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-task-3"))
            .is_err());

        Ok(())
    }

    #[test]
    fn cleartodo_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-ct");
        assert!(cmd
            .assert()
            .success()
            .try_stdout(predicate::str::contains("1: repeating-task-3"))
            .is_err());

        Ok(())
    }
}

mod repeating_todo_resetall {
    use super::*;

    #[test]
    fn resetall_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-resetall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn resetall_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-ra").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn doneresetall_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-doneresetall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn doneresetall_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-dra").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn resetall_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-resetall");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn resetall_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-ra");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn doneresetall_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-doneresetall");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn doneresetall_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-dra");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn resetall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-resetall");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("2: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("3: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("4: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("5: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"));

        Ok(())
    }

    #[test]
    fn resetall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-ra");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("2: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("3: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("4: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("5: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"));

        Ok(())
    }

    #[test]
    fn doneresetall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-doneresetall");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("2: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("3: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("4: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("5: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"));

        Ok(())
    }

    #[test]
    fn doneresetall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-dra");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("1: repeating-task-5"))
            .stdout(predicate::str::contains("interval: 1 day"))
            .stdout(predicate::str::contains("2: repeating-task-4"))
            .stdout(predicate::str::contains("interval: 2 days"))
            .stdout(predicate::str::contains("3: repeating-task-3"))
            .stdout(predicate::str::contains("interval: 3 days"))
            .stdout(predicate::str::contains("4: repeating-task-2"))
            .stdout(predicate::str::contains("interval: 4 days"))
            .stdout(predicate::str::contains("5: repeating-task-1"))
            .stdout(predicate::str::contains("interval: 5 days"));

        Ok(())
    }
}

mod repeating_todo_showstartall {
    use super::*;

    #[test]
    fn startall_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("repeating-startall").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn startall_abrev_no_args_allowed() -> Result<(), Box<dyn std::error::Error>> {
        // actions
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rp-sa").arg("1");
        cmd.assert().success().stdout(predicate::str::contains(
            "Invalid command. Please try again, or try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn startall_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("repeating-startall");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn startall_abrev_empty_todo() -> Result<(), Box<dyn std::error::Error>> {
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
        cmd.arg("rp-sa");
        cmd.assert().success().stdout(predicate::str::contains(
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.",
        ));

        Ok(())
    }

    #[test]
    fn startall_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("repeating-startall");
        cmd.assert().success().stdout(predicate::str::contains("task: repeating-task-5\n\tstart: 2099-01-01 00:00\ntask: repeating-task-4\n\tstart: 2099-01-01 00:00\ntask: repeating-task-3\n\tstart: 2099-01-01 00:00\ntask: repeating-task-2\n\tstart: 2099-01-01 00:00\ntask: repeating-task-1\n\tstart: 2099-01-01 00:00"));

        Ok(())
    }

    #[test]
    fn startall_abrev_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
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
        cmd.arg("rp-sa");
        cmd.assert().success().stdout(predicate::str::contains("task: repeating-task-5\n\tstart: 2099-01-01 00:00\ntask: repeating-task-4\n\tstart: 2099-01-01 00:00\ntask: repeating-task-3\n\tstart: 2099-01-01 00:00\ntask: repeating-task-2\n\tstart: 2099-01-01 00:00\ntask: repeating-task-1\n\tstart: 2099-01-01 00:00"));

        Ok(())
    }
}

mod repeating_todo_editall {}

mod repeating_todo_edittask {}

mod repeating_todo_editinterval {}

mod repeating_todo_edittimeunit {}

mod repeating_todo_editintervalunit {}

mod repeating_todo_editstart {}

mod repeating_todo_editend {}

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
