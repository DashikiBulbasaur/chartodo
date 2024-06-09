use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

pub fn read_file_and_create_vecs(path: &str) -> (Vec<String>, Vec<String>) {
    let file = File::open(path)
        .expect("file doesn't exist even though it should. If this happens outside of a test, i.e., during use, please create a general_list.txt file in src");
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
    // NB: max len for todo_buf and done_buf is 15
    for line in file_buf {
        if line == "-----" {
            todo_done_demarcation = 1;
        } else {
            match todo_done_demarcation {
                0 => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 150 chars are ommitted
                    if todo_buf.len() < 15 && line.len() < 150 {
                        todo_buf.push(line.to_string());
                    }
                }
                _ => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 150 chars are ommitted
                    if done_buf.len() < 15 && line.len() < 150 {
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

#[cfg(test)]
mod helpers_unit_tests {

    use super::*;
    // note: I'd like to use assert_fs to create temp files, but I can't make NamedTempFile work
    // like in rust grrs cli tutorial

    #[test]
    fn reading_and_creating_vecs_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_file = File::create("test.txt")?;
        test_file.write(b"CHARTODO\nthis\nis\na\ntest\n---\n-----\nDONE\nplease\npass")?;

        let (test_todo, test_done) = read_file_and_create_vecs("test.txt");
        std::fs::remove_file("test.txt")?;

        let correct_test = vec![
            "CHARTODO".to_string(),
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
            "---".to_string(),
        ];
        let correct_done = vec!["DONE".to_string(), "please".to_string(), "pass".to_string()];
        assert_eq!((test_todo, test_done), (correct_test, correct_done));

        Ok(())
    }

    #[test]
    fn positions_in_lists_are_correct() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_file = File::create("test1.txt")?;
        test_file.write(b"CHARTODO\nthis\nis\na\ntest\n---\n-----\nDONE\nplease\npass")?;

        let (test_todo, test_done) = read_file_and_create_vecs("test1.txt");
        // note: different file cuz I think there's a concurrency issue when I try to delete the
        // same file from different test fns
        std::fs::remove_file("test1.txt")?;
        let (test_todo, test_done) = add_positions_to_todo_and_done(test_todo, test_done);

        let correct_test = vec![
            "CHARTODO".to_string(),
            "1: this".to_string(),
            "2: is".to_string(),
            "3: a".to_string(),
            "4: test".to_string(),
            "5: ---".to_string(),
        ];
        let correct_done = vec![
            "DONE".to_string(),
            "1: please".to_string(),
            "2: pass".to_string(),
        ];
        assert_eq!((test_todo, test_done), (correct_test, correct_done));

        Ok(())
    }
}
