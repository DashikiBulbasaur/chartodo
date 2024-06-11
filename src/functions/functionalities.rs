use super::helpers::{add_positions_to_todo_and_done, print_the_lists, read_file_and_create_vecs};
use std::{fs::File, io::Write, sync::PoisonError};

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

    // the following conditionals check for invalid inputs.
    if add_item.is_empty() {
        return writeln!(
            writer,
            "Items to be added to the todo list cannot be empty. Please try again, or try --help"
        )
        .expect("writeln failed");
    }

    // note: this will never activate i think. either clap or cargo/rust panics when something like
    // -- is added.
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

    writeln!(writer, "'{}' was added to todo\n", add_item).expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}

pub fn change_todo_item_to_done(position: String) {
    let (mut todo_buf, mut done_buf) = read_file_and_create_vecs("src/general_list.txt");

    let writer = &mut std::io::stdout();

    if position.is_empty() {
        return writeln!(
            writer,
            "You must specify the todo item's position. A good example would be: 'chartodo done 3'. Please try again, or try --help."
        )
        .expect("writeln failed");
    }

    if position.parse::<u32>().is_err() {
        return writeln!(
            writer,
            "You must specify the todo item's position, and it has to be a number that is not zero or negative. A good example would be: 'chartodo done 3'. Please try again, or try --help."
        )
        .expect("writeln failed");

        // NB: the user can't seem to do a negative number arg like -1, or else clap/cargo panics 
        // and complains. I also can't seem to test for it
    }

    if todo_buf.is_empty() {
        writeln!(
            writer,
            "The todo list is currently empty, so there are no todo items that can be marked as done."
        )
        .expect("writeln failed");

        return print_the_lists(todo_buf, done_buf);
    }

    if position.parse::<u32>().unwrap() == 0 {
        return writeln!(
            writer,
            "The position specified cannot be 0. Try a position that is between 1 and {}. Please try again, or try --help.", todo_buf.len() - 1
        )
        .expect("writeln failed");
    }

    if position.parse::<u32>().unwrap() > (todo_buf.len() - 1).try_into().unwrap() {
        return writeln!(
            writer,
            "The todo list is smaller than your specified position; therefore, the item you want to mark as done doesn't exist. The position has to be {} or lower. Please try again, or try --help.", todo_buf.len() - 1
        )
        .expect("writeln failed");
    }

    // get the todo item, remove it from todo, and push it to done
    let position = position.parse::<usize>().unwrap();
    let todo_to_done = todo_buf.get(position).unwrap().to_string();
    todo_buf.remove(position);
    done_buf.push(todo_to_done.clone());

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

    // NB: add positions to todo and done b4 printing
    let (todo_buf, done_buf) = add_positions_to_todo_and_done(todo_buf, done_buf);

    writeln!(writer, "'{}' was marked as done\n", todo_to_done).expect("writeln failed");

    // NB: print the lists
    print_the_lists(todo_buf, done_buf);
}
