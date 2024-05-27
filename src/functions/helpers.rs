use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

pub fn read_file_and_create_vecs() -> (Vec<String>, Vec<String>) {
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
    for line in file_buf {
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

    (todo_buf, done_buf)
}

pub fn add_positions_to_todo_and_done(
    mut todo_buf: Vec<String>,
    mut done_buf: Vec<String>,
) -> (Vec<String>, Vec<String>) {
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

    (todo_buf, done_buf)
}

pub fn print_the_lists(todo_buf: Vec<String>, done_buf: Vec<String>) {
    // though short, this one happens pretty often

    let writer = &mut std::io::stdout();

    // print the lists
    todo_buf
        .iter()
        .for_each(|item| writeln!(writer, "{}", item).expect("writeln failed"));
    writeln!(writer, "-----").expect("writeln failed");
    done_buf
        .iter()
        .for_each(|item| writeln!(writer, "{}", item).expect("writeln failed"));
}
