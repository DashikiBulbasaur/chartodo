use super::json_file_structs::Tasks;

pub fn regular_tasks_list(regular_tasks: Tasks) -> (String, String) {
    let mut regular_todo = String::from("");
    let mut counter: u8 = 1;
    regular_tasks.todo.iter().for_each(|item| {
        let task = format!("{}: {}\n", counter, item.task);
        counter += 1;
        regular_todo.push_str(&task);
    });
    let regular_todo = regular_todo.trim_end();

    let mut regular_done = String::from("");
    let mut counter: u8 = 1;
    regular_tasks.done.iter().for_each(|item| {
        let task = format!("{}: {}\n", counter, item.task);
        counter += 1;
        regular_done.push_str(&task);
    });
    let regular_done = regular_done.trim_end();

    match regular_done.is_empty() {
        true => (regular_todo.to_string(), regular_done.to_string()),
        false => {
            let regular_done = "DONE\n---\n".to_string() + regular_done;
            (regular_todo.to_string(), regular_done)
        }
    }
}

/*
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
    */
