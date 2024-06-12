use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{fs::File, io::Write, process::Command};

// note: to run these tests, do cargo test --test outputs -- --test-threads=1
// note that it seems to be 90% working with cargo test --test outputs, though it may just be a
// coincidence. not running it on one thread prevents the file reset at the end

// note: to run only integration tests, do cargo test --test '*'. For this program's case, probably
// good to also add test-threads=1

fn create_test_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut test_file = File::create("src/general_list.txt")?;
    test_file.write_all(
        b"CHARTODO\nthis\nis\nthe\ntodo\nlist\n-----\nDONE\nthis\nis\nthe\ndone\nlist",
    )?;

    Ok(())
}

#[test]
fn list_prints_correctly() -> Result<(), Box<dyn std::error::Error>> {
    // note: I really don't like doing it this way, but the program only ever accesses one file
    // (unless I decide to expand), and so the only way to do integration testing is to access that
    // one file again and again
    let _ = create_test_file();

    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("list");
    cmd.assert().success().stdout(predicate::str::contains(
        "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
    ));

    Ok(())
}

#[test]
fn adds_item_correctly() -> Result<(), Box<dyn std::error::Error>> {
    let _ = create_test_file();

    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("add").arg("item");
    cmd.assert().success().stdout(predicate::str::contains(
        "'item' was added to todo\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
    ));

    // NB: in functionalities.rs, there is only one \n after the notification. so there should be
    // no space between the noti and the list proper, yet there is one. i've yet to find out why
    // this is

    Ok(())
}

#[test]
fn empty_add_item() -> Result<(), Box<dyn std::error::Error>> {
    // note: I don't know how this would ever activate. On main, it panics if there's no item to be
    // added. I guess this would activate if a person pasted a no-character/empty string to the
    // console?

    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("add").arg("");
    cmd.assert().try_success()?.stdout(predicate::str::contains(
        "Items to be added to the todo list cannot be empty. Please try again, or try --help",
    ));

    Ok(())
}

#[test]
fn item_to_be_added_is_too_long() -> Result<(), Box<dyn std::error::Error>> {
    // note: the character limit for the list is 150
    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("add").arg("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    cmd.assert().try_success()?.stdout(predicate::str::contains(
        "The maximum length of an item is 150 characters. Please try again, or try --help",
    ));

    Ok(())
}

mod todo_item_is_done_tests {
    use super::*;

    #[test]
    fn todo_item_to_be_marked_as_done_is_not_specified() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position. A good example would be: 'chartodo done 3'. Please try again, or try --help.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position. A good example would be: 'chartodo done 3'. Please try again, or try --help.",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_the_todo_item_to_be_marked_as_done(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo done 3'. Please try again, or try --help.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo done 3'. Please try again, or try --help.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_the_todo_item_to_be_marked_as_done_is_zero(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try --help.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try --help.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_todo_item_to_be_marked_as_done_is_too_big(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to mark as done doesn't exist. The position has to be 5 or lower. Please try again, or try --help.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to mark as done doesn't exist. The position has to be 5 or lower. Please try again, or try --help.",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_moved_to_done_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("done").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was marked as done\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: list",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_moved_to_done_correctly_shortcut() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("d").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was marked as done\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list\n6: list",
        ));

        Ok(())
    }
}

mod remove_todo_item_tests {
    use super::*;

    #[test]
    fn position_for_todo_item_to_be_removed_is_not_specified(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be removed. A good example would be: 'chartodo rmtodo 3'. Please try again, or try --help.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be removed. A good example would be: 'chartodo rmtodo 3'. Please try again, or try --help.",
        ));

        Ok(())
    }

    #[test]
    fn position_is_not_a_number_or_not_u8_for_todo_item_to_be_removed(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmtodo 3'. Please try again, or try --help.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("a");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "You must specify the todo item's position that will be removed, and it has to be a number that is not zero or negative. For now, your number also can't be bigger than 255. A good example would be: 'chartodo rmtodo 3'. Please try again, or try --help.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_the_todo_item_to_be_removed_is_zero() -> Result<(), Box<dyn std::error::Error>>
    {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try --help.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("0");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The position specified cannot be 0. Try a position that is between 1 and 5. Please try again, or try --help.",
        ));

        Ok(())
    }

    #[test]
    fn position_for_todo_item_to_be_removed_is_too_big() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be 5 or lower. Please try again, or try --help.",
        ));

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("10");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "The todo list is smaller than your specified position; therefore, the item you want to remove doesn't exist. The position has to be 5 or lower. Please try again, or try --help.",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_removed_correctly() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmtodo").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was removed from todo\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        Ok(())
    }

    #[test]
    fn todo_item_removed_correctly_shortcut() -> Result<(), Box<dyn std::error::Error>> {
        let _ = create_test_file();

        let mut cmd = Command::cargo_bin("chartodo")?;
        cmd.arg("rmt").arg("5");
        cmd.assert().try_success()?.stdout(predicate::str::contains(
            "'list' was removed from todo\n\nCHARTODO\n1: this\n2: is\n3: the\n4: todo\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
        ));

        Ok(())
    }
}

#[test]
fn invalid_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("blahblah");
    cmd.assert().success().stdout(predicate::str::contains(
        "invalid command. please try again, or try --help",
    ));

    Ok(())
}

#[test]
fn zzz_resets_the_file() {
    // note: this is just to reset the file after all the changes for my own convenience.
    // the fn also starts with zzz cuz rust runs the tests in alphabetical order, and I
    // want this to be the last one everytime
    let _ = create_test_file();
}
