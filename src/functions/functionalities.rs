use super::helpers::{add_positions_to_todo_and_done, print_the_lists, read_file_and_create_vecs};
use std::{fs::File, io::Write};

pub fn list() {
    // NB: read from file and separate it into vecs
    let (todo_buf, done_buf) = read_file_and_create_vecs("src/general_list.txt");
    // NB: add positions to todo_buf and done_buf before printing
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);
    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn add_todo_item(add_item: String) {
    // NB: read from file and separate into vecs
    let (mut todo_buf, done_buf) = read_file_and_create_vecs("src/general_list.txt");

    let writer = &mut std::io::stdout();

    // the following conditionals check for invalid inputs
    if add_item.is_empty() {
        return writeln!(
            writer,
            "Item cannot be empty. Please try again, or try --help"
        )
        .expect("writeln failed");
    }

    if add_item.trim() == "-----" {
        return writeln!(
            writer,
            "----- is an invalid item. It is the only invalid item. Please try again, or try --help"
        )
        .expect("writeln failed");
    }

    if add_item.trim().len() > 150 {
        return writeln!(
            writer,
            "The maximum length of an item is 150 characters. Please try again, or try --help"
        )
        .expect("writeln failed");
    }

    // -----

    // the following lines do the following in order:
    // 1. push new item to todo
    // 2. create a new file
    // 3. push todo_buf and done_buf to file
    todo_buf.push(add_item.clone());
    let mut new_file = File::create("src/general_list.txt");

    todo_buf.iter().for_each(|item| {
        writeln!(
            new_file.as_mut().expect("new_file couldn't be accessed"),
            "{}",
            item
        )
        .expect("writeln failed")
    });
    writeln!(
        new_file.as_mut().expect("new_file couldn't be accessed"),
        "-----"
    )
    .expect("writeln failed");
    done_buf.iter().for_each(|item| {
        writeln!(
            new_file.as_mut().expect("new_file couldn't be accessed"),
            "{}",
            item
        )
        .expect("writeln failed")
    });

    // ----

    // NB: todo_buf has a max len of 15
    // NB: adding items that would exceed the max len just erases the last item and replaces it
    // with the new one. idk how it does this, but i'm fine with it

    // NB: add positions to todo_buf and done_buf before printing
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "'{}' has been added to todo\n", add_item).expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}
