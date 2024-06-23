use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

// TODO: all these let _ = .. that I keep doing where I just ignore the errors probably shouldn't
// be there. Error handling for those later

fn create_dir_and_file_if_needed() -> Result<(), Box<dyn std::error::Error>> {
    let mut something = dirs::data_dir().expect("could not get path $HOME/.local/share/");
    something.push("chartodo");

    if !something.exists() {
        // note: this isn't create_dir_all cuz if god forbid the file paths leading up to it
        // somehow don't exist, i'd rather it just fail than to force create them
        let _ = std::fs::create_dir(something.clone());
    }
    something.push("general_list.txt");

    if !Path::new(&something).exists() {
        let mut general_list = File::create(something)?;
        general_list.write_all(
            b"CHARTODO\nthis\nis\nthe\ntodo\nlist\n-----\nDONE\nthis\nis\nthe\ndone\nlist",
        )?;
    }

    Ok(())
}

pub fn read_file_and_create_vecs(path: PathBuf) -> (Vec<String>, Vec<String>) {
    let _ = create_dir_and_file_if_needed();

    let file = File::open(path).expect("could not open file in path");
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
    // NB: max len for todo_buf is 15 and 30 for done_buf
    for line in file_buf {
        if line == "-----" {
            todo_done_demarcation = 1;
        } else {
            match todo_done_demarcation {
                0 => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 50 chars are ommitted
                    if todo_buf.len() < 15 && line.len() < 30 {
                        todo_buf.push(line.to_string());
                    }
                }
                _ => {
                    // only applies if the user manually modifies general_list.txt
                    // lines with more than 50 chars are ommitted
                    if done_buf.len() < 15 && line.len() < 30 {
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

pub fn create_new_file_and_write(
    path: PathBuf,
    todo_buf: Vec<String>,
    done_buf: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    let mut new_file = File::create(path);

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

    (todo_buf, done_buf)
}

#[cfg(test)]
mod helpers_unit_tests {
    // note: to run this specifically, do cargo test helpers_unit_tests, or just helpers

    use super::*;
    // note: I'd like to use assert_fs to create temp files, but I can't make NamedTempFile work
    // like in rust grrs cli tutorial

    #[test]
    fn reading_and_creating_vecs_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_file = File::create("test.txt")?;
        test_file.write_all(b"CHARTODO\nthis\nis\na\ntest\n---\n-----\nDONE\nplease\npass")?;

        let (test_todo, test_done) = read_file_and_create_vecs("test.txt".into());
        std::fs::remove_file("test.txt")?;

        let correct_todo = vec![
            "CHARTODO".to_string(),
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
            "---".to_string(),
        ];
        let correct_done = vec!["DONE".to_string(), "please".to_string(), "pass".to_string()];
        assert_eq!((test_todo, test_done), (correct_todo, correct_done));

        Ok(())
    }

    #[test]
    fn positions_in_lists_are_correct() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_file = File::create("test1.txt")?;
        test_file.write_all(b"CHARTODO\nthis\nis\na\ntest\n---\n-----\nDONE\nplease\npass")?;

        let (test_todo, test_done) = read_file_and_create_vecs("test1.txt".into());
        // note: different file cuz I think there's a concurrency issue when I try to delete the
        // same file from different test fns. I could just run these one by one with the same file,
        // with test-threads=1, but that takes 2 long and is a last resort
        std::fs::remove_file("test1.txt")?;
        let (test_todo, test_done) = add_positions_to_todo_and_done(test_todo, test_done);

        let correct_todo = vec![
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
        assert_eq!((test_todo, test_done), (correct_todo, correct_done));

        Ok(())
    }

    #[test]
    fn writing_changes_to_file_is_correct() -> Result<(), Box<dyn std::error::Error>> {
        // note: this might be a convoluted piece of mess, but i wrote this for my own peace of
        // mind so i know that what's written on the file is correct

        let correct_todo = vec![
            "CHARTODO".to_string(),
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
        ];
        let correct_done = vec!["DONE".to_string(), "please".to_string(), "pass".to_string()];
        let mut correct_full_list = vec![];
        correct_todo
            .iter()
            .for_each(|item| correct_full_list.push(item));
        let binding = "-----".to_string();
        correct_full_list.push(&binding);
        correct_done
            .iter()
            .for_each(|item| correct_full_list.push(item));

        let (_, _) = create_new_file_and_write(
            "test2.txt".into(),
            correct_todo.clone(),
            correct_done.clone(),
        );
        let (test_todo, test_done) = read_file_and_create_vecs("test2.txt".into());
        std::fs::remove_file("test2.txt")?;

        let mut test_full_list = vec![];
        test_todo.iter().for_each(|item| test_full_list.push(item));
        test_full_list.push(&binding);
        test_done.iter().for_each(|item| test_full_list.push(item));

        assert_eq!(correct_full_list, test_full_list);

        Ok(())
    }
}
