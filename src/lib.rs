use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

pub fn list() {
    let file = File::open("src/general_list.txt")
        .expect("general_list.txt doesn't exist even though it should. Please create a general_list.txt file in src");
    // TODO: if this fails, perhaps create a file and open it one more time
    let reader = BufReader::new(file);
    let writer = &mut std::io::stdout();

    // separate the lists into vecs so i can do operations on them
    let mut file_buf: Vec<String> = vec![];
    let mut todo_buf: Vec<String> = vec![];
    let mut done_buf: Vec<String> = vec![];

    // the reason for doing this separately is that Rust complains if I use line to check for
    // conditions
    for line in reader.lines() {
        file_buf.push(line.expect("couldn't unwrap line and push to file_buf"));
    }

    // if this is 1, that means the todo list is done and the loop can push to done_buf
    let mut todo_done_demarcation = 0;
    // NB: max len for todo_buf and done_buf is 10
    for line in &file_buf {
        if line == "-----" {
            todo_done_demarcation = 1;
        } else {
            match todo_done_demarcation {
                0 => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 150 chars are ommitted
                    if todo_buf.len() < 10 && line.len() < 150 {
                        todo_buf.push(line.to_string());
                    }
                }
                _ => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 150 chars are ommitted
                    if done_buf.len() < 10 && line.len() < 150 {
                        done_buf.push(line.to_string());
                    }
                }
            }
        }
    }

    // NB: the reason general_list.txt doesn't have the number positions on it is bc it's harder to
    // modify and manipulate it with most/all of the functionalities that mutate the list if it had
    // the index positions. Leaving it with no positions and adding it when needed is my preferred
    // approach.

    // add the positions to the todo items
    let mut index = 1;
    // Skip the 1st element cuz that's TODO
    for item in todo_buf.iter_mut().skip(1) {
        let mut index_format = format!("{index}: ");
        index_format.push_str(item);
        *item = index_format;
        index += 1;
    }

    // add the positions to the done items
    let mut index = 1;
    // Skip the 1st element cuz that's DONE
    for item in done_buf.iter_mut().skip(1) {
        let mut index_format = format!("{index}: ");
        index_format.push_str(item);
        *item = index_format;
        index += 1;
    }

    // clear file_buf since it still has its old elements
    file_buf.clear();

    // push the todo items to file_buf
    todo_buf
        .iter()
        .for_each(|item| file_buf.push(item.to_string()));

    // push the demarcation to file_buf
    file_buf.push("-----".to_string());

    // push the done items to file_buf
    done_buf
        .iter()
        .for_each(|item| file_buf.push(item.to_string()));

    // print file_buf
    file_buf
        .iter()
        .for_each(|item| writeln!(writer, "{}", item).expect("writeln failed"));
}

pub fn add_todo_item(item: String) {
    let writer = &mut std::io::stdout();

    if item.is_empty() {
        return writeln!(
            writer,
            "Item cannot be empty. Please try again, or try --help"
        )
        .expect("writeln failed");
    }

    if item.trim() == "-----" {
        return writeln!(
            writer,
            "----- is an invalid item. It is the only invalid item. Please try again, or try --help"
        )
        .expect("writeln failed");
    }

    if item.trim().len() > 150 {
        return writeln!(
            writer,
            "The maximum length of an item is 150 characters. Please try again, or try --help"
        )
        .expect("writeln failed");
    }

    let file = File::open("src/general_list.txt")
        .expect("general_list.txt doesn't exist even though it should. Please create a general_list.txt file in src");
    // TODO: if this fails, perhaps create a file and open it one more time
    let reader = BufReader::new(file);

    // separate the lists into vecs so i can do operations on them
    let mut file_buf: Vec<String> = vec![];
    let mut todo_buf: Vec<String> = vec![];
    let mut done_buf: Vec<String> = vec![];

    // the reason for doing this separately is that Rust complains if I use line to check for
    // conditions
    for line in reader.lines() {
        file_buf.push(line.expect("couldn't unwrap line and push to file_buf"));
    }

    // if this is 1, that means the todo list is done and the loop can push to done_buf
    let mut todo_done_demarcation = 0;
    // NB: max len for todo_buf and done_buf is 10
    for line in &file_buf {
        if line == "-----" {
            todo_done_demarcation = 1;
        } else {
            match todo_done_demarcation {
                0 => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 150 chars are ommitted
                    if todo_buf.len() < 10 && line.len() < 150 {
                        todo_buf.push(line.to_string());
                    }
                }
                _ => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 150 chars are ommitted
                    if done_buf.len() < 10 && line.len() < 150 {
                        done_buf.push(line.to_string());
                    }
                }
            }
        }
    }

    todo_buf.push(item.clone());

    let mut new_file = File::create("src/general_list.txt");

    // clear file_buf since it still has its old elements
    file_buf.clear();

    // push the todo items to file_buf
    todo_buf
        .iter()
        .for_each(|item| file_buf.push(item.to_string()));

    // push the demarcation to file_buf
    file_buf.push("-----".to_string());

    // push the done items to file_buf
    done_buf
        .iter()
        .for_each(|item| file_buf.push(item.to_string()));

    file_buf.iter().for_each(|item| {
        writeln!(
            new_file.as_mut().expect("new_file couldn't be accessed"),
            "{}",
            item
        )
        .expect("writeln failed")
    });

    // add the positions to the todo items
    let mut index = 1;
    // Skip the 1st element cuz that's TODO
    for item in todo_buf.iter_mut().skip(1) {
        let mut index_format = format!("{index}: ");
        index_format.push_str(item);
        *item = index_format;
        index += 1;
    }

    // add the positions to the done items
    let mut index = 1;
    // Skip the 1st element cuz that's DONE
    for item in done_buf.iter_mut().skip(1) {
        let mut index_format = format!("{index}: ");
        index_format.push_str(item);
        *item = index_format;
        index += 1;
    }

    // clear file_buf since it still has its old elements
    file_buf.clear();

    // push the todo items to file_buf
    todo_buf
        .iter()
        .for_each(|item| file_buf.push(item.to_string()));

    // push the demarcation to file_buf
    file_buf.push("-----".to_string());

    // push the done items to file_buf
    done_buf
        .iter()
        .for_each(|item| file_buf.push(item.to_string()));

    writeln!(writer, "{} has been added to the todo list\n", item).expect("writeln failed");

    file_buf
        .iter()
        .for_each(|item| writeln!(writer, "{}", item).expect("writeln failed"));
}
