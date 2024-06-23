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
            "Did not provide the done item to be removed. Good example: chartodo rmdone 3. If you have more questions, try chartodo help or chartodo --help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Did not provide the done item to be removed. Good example: chartodo rmd 3. If you have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn position_for_done_item_to_be_removed_is_empty() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be removed. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be removed. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_done_item_to_be_removed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("256");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmdone 3'. Please try again, or try 'chartodo help'.",
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
        cmd.arg("rmd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is already empty, so there are no done items that can be removed.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_the_done_item_to_be_removed_is_zero() -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_done_item_to_be_removed_is_too_big() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn done_item_removed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was removed from done\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was removed from done\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
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
            "Did not provide the done item to be reversed back to todo. Good example: chartodo notdone 3. If you have more questions, try chartodo help or chartodo --help",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd");
        cmd.assert().try_failure()?.stderr(predicate::str::contains(
            "Did not provide the done item to be reversed back to todo. Good example: chartodo nd 3. If you have more questions, try chartodo help or chartodo --help",
        ));

        Ok(())
    }

    #[test]
    fn position_for_done_item_to_be_reversed_is_empty() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be reversed. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be reversed. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_done_item_to_be_reversed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be reversed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the done item's position that will be reversed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo notdone 3', and if there was a done item at position 3, it would be reversed back to a todo item. Please try again, or try 'chartodo help'.",
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
    fn position_for_the_done_item_to_be_reversed_is_zero() -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_done_item_to_be_reversed_is_too_big() -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is smaller than your specified position; therefore, the item you want to reverse doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The done list is smaller than your specified position; therefore, the item you want to reverse doesn't exist. The position has to be 5 or lower. Please try again, or try 'chartodo help'.",
        ));

        Ok(())
    }

    #[test]
    fn done_item_reversed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("notdone").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was reversed from done back to todo.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
        ));

        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("nd").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was reversed from done back to todo.\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done",
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
