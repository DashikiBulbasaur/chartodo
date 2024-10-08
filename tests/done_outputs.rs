mod general_commands_tests;

use assert_cmd::prelude::*;
use general_commands_tests::*;
use predicates::prelude::*;
use std::process::Command;

// cargo test --test done_outputs -- --test-threads=1

mod clear_done_list_tests {
    use super::*;

    #[test]
    fn done_list_is_already_empty_cleartodo() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_done_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cleardone");
        cmd.assert()
            .try_success()?
            .stdout(predicate::str::contains("The done list is already empty."));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cd");
        cmd.assert()
            .try_success()?
            .stdout(predicate::str::contains("The done list is already empty."));

        Ok(())
    }

    #[test]
    fn done_list_was_cleared_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cleardone");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list was cleared.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("cd");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list was cleared.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE",
        ));

        Ok(())
    }
}

mod remove_done_item_tests {
    use super::*;

    #[test]
    fn position_for_done_item_to_be_removed_is_missing() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Did not provide the done item to be removed. Good example: chartodo rmdone 3, or chartodo rmdone 3 4 5. If you have more questions, try chartodo help or chartodo --help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Did not provide the done item to be removed. Good example: chartodo rmd 3, or chartodo rmd 3 4 5. If you have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn correctly_check_done_list_is_empty_when_removing_done_item(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_done_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is already empty, so there are no done items that can be removed.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("5").arg("1").arg("3");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is already empty, so there are no done items that can be removed.",
        ));

        Ok(())
    }

    #[test]
    fn arguments_given_meet_or_exceed_done_list_current_length(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone")
            .arg("5")
            .arg("1")
            .arg("2")
            .arg("3")
            .arg("4");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The number of arguments you provided meet or exceed the done list's current filled length. You might as well do chartodo cleardone. For more information, try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd")
            .arg("5")
            .arg("1")
            .arg("3")
            .arg("1")
            .arg("6")
            .arg("2")
            .arg("4");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The number of arguments you provided meet or exceed the done list's current filled length. You might as well do chartodo cleardone. For more information, try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn done_item_removed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        // nothing is removed from this one
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        Ok(())
    }

    #[test]
    fn done_items_removed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("5").arg("1");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: is\n2: the\n3: done",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("5").arg("1");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: is\n2: the\n3: done",
        ));

        // nothing is removed from this one
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("abc").arg("xyz");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: is\n2: the\n3: done",
        ));

        Ok(())
    }
}

mod reverse_done_item_back_to_todo_tests {
    use super::*;

    #[test]
    fn correctly_check_done_list_is_empty_when_reversing_done_item(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_done_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdoneall");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is empty, and so has no items that can be changed back to todo",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nda");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is empty, and so has no items that can be changed back to todo",
        ));

        Ok(())
    }

    #[test]
    fn todo_list_is_too_full_cant_take_the_entire_done_list_to_reverse(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_almost_full_todo_list_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdoneall");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You're trying to reverse too many dones back to todos, and doing so would exceed the todo list's maximum length. Please remove some or clear all of the todo items. To see how, try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nda");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You're trying to reverse too many dones back to todos, and doing so would exceed the todo list's maximum length. Please remove some or clear all of the todo items. To see how, try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn all_done_items_reversed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdoneall");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "All dones were reversed back to todos\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: this\n7: is\n8: the\n9: done\n10: list\n-----\nDONE",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nda");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "All dones were reversed back to todos\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: this\n7: is\n8: the\n9: done\n10: list\n-----\nDONE",
        ));

        Ok(())
    }
}

#[test]
fn zzz_resets_the_file() {
    // note: this is just to reset the file after all the changes for my own convenience.
    // the fn also starts with zzz cuz rust runs the tests in alphabetical order, and I
    // want this to be the last one everytime
    let _ = create_test_file();
}
