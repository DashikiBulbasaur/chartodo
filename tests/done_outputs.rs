mod outputs_helpers;

use assert_cmd::prelude::*;
use outputs_helpers::*;
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
    fn position_for_the_done_item_to_be_reversed_is_missing(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Did not provide the done item to be reversed back to todo. Good example: chartodo notdone 3, or chartodo notdone 3 4 5. If you have more questions, try chartodo help or chartodo --help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Did not provide the done item to be reversed back to todo. Good example: chartodo nd 3, or chartodo nd 3 4 5. If you have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn correctly_check_done_list_is_empty_when_reversing_done_item(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_empty_done_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is already empty, so there are no done items that can be reversed.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is already empty, so there are no done items that can be reversed.",
        ));

        Ok(())
    }

    #[test]
    fn the_number_of_arguments_meet_or_exceed_done_list_current_length(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone")
            .arg("1")
            .arg("3")
            .arg("2")
            .arg("6")
            .arg("1")
            .arg("4")
            .arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains("The number of arguments you provided meet or exceed the done list's current filled length. You might as well just do chartodo notdoneall. For more information, try chartodo help"));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd")
            .arg("1")
            .arg("3")
            .arg("2")
            .arg("6")
            .arg("1")
            .arg("4")
            .arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains("The number of arguments you provided meet or exceed the done list's current filled length. You might as well just do chartodo notdoneall. For more information, try chartodo help"));

        Ok(())
    }

    #[test]
    fn todo_list_is_too_full_cant_take_the_provided_done_items_to_reverse(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_almost_full_todo_list_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("5").arg("1");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently full. Try removing items or clearing it. For more information, try chartodo help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("5").arg("1");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is currently full. Try removing items or clearing it. For more information, try chartodo help",
        ));

        Ok(())
    }

    #[test]
    fn done_item_reversed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        // this one does nothing
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("abc");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        Ok(())
    }

    #[test]
    fn done_items_reversed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        // and some invalids too
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone")
            .arg("5")
            .arg("1")
            .arg("10")
            .arg("")
            .arg("0")
            .arg("b");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n7: this\n-----\nDONE\n1: is\n2: the\n3: done",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd")
            .arg("5")
            .arg("1")
            .arg("10")
            .arg("")
            .arg("0")
            .arg("b");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n7: this\n-----\nDONE\n1: is\n2: the\n3: done",
        ));

        // this one does nothing
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("abc").arg("xyz");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n7: this\n-----\nDONE\n1: is\n2: the\n3: done",
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
