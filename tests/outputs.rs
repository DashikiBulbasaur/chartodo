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
    test_file
        .write(b"CHARTODO\nthis\nis\nthe\ntodo\nlist\n-----\nDONE\nthis\nis\nthe\ndone\nlist")?;

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
        "CHARTODO\n1: this\n2: is\n3: the\n4: todo\n5: list\n6: item\n-----\nDONE\n1: this\n2: is\n3: the\n4: done\n5: list",
    ));

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
fn invalid_input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("chartodo")?;
    cmd.arg("blahblah");
    cmd.assert().success().stdout(predicate::str::contains(
        "invalid command. please try again, or try --help",
    ));

    Ok(())
}

#[test]
fn resets_the_file() {
    // note: this is just to reset the file after all the changes for my own convenience
    let _ = create_test_file();
}
