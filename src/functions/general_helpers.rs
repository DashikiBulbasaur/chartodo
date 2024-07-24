use super::json_file_structs::*;
use super::repeating_tasks::repeating_helpers::*;
use chrono::{Days, Duration, Local, Months, NaiveDateTime};
use std::process::exit;

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

pub fn deadline_tasks_list(deadline_tasks: Tasks) -> (String, String) {
    let mut deadline_todo = String::from("");
    let mut counter: u8 = 1;
    deadline_tasks.todo.iter().for_each(|item| {
        let task = format!(
            "{}: {}\n   {}: {} {}\n",
            counter,
            item.task,
            check_if_due_or_not(item.date.clone().unwrap(), item.time.clone().unwrap()),
            item.date.clone().unwrap(),
            item.time.clone().unwrap()
        );
        counter += 1;
        deadline_todo.push_str(&task);
    });
    let deadline_todo = deadline_todo.trim_end();

    let mut deadline_done = String::from("");
    let mut counter: u8 = 1;
    deadline_tasks.done.iter().for_each(|item| {
        let task = format!(
            "{}: {}\n   done: {} {}\n",
            counter,
            item.task,
            item.date.clone().unwrap(),
            item.time.clone().unwrap()
        );
        counter += 1;
        deadline_done.push_str(&task);
    });
    let deadline_done = deadline_done.trim_end();

    match deadline_done.is_empty() {
        true => (deadline_todo.to_string(), deadline_done.to_string()),
        false => {
            let deadline_done = "DONE\n---\n".to_string() + deadline_done;
            (deadline_todo.to_string(), deadline_done)
        }
    }
}

fn check_if_due_or_not(date: String, time: String) -> String {
    if date < Local::now().date_naive().to_string()
        || date == Local::now().date_naive().to_string() && time < Local::now().time().to_string()
    {
        "MISSED".to_string()
    } else {
        "due".to_string()
    }
}

pub fn repeating_tasks_list(mut repeating_tasks: Tasks) -> (String, String) {
    // check if any repeating tasks are done first. if they are, push to todo and remove from done
    let now_date = Local::now().date_naive().to_string();
    let now_time = Local::now().time().to_string();
    let mut remove_these: Vec<Task> = vec![];
    repeating_tasks.done.iter().for_each(|task| {
        // double check that repeat_done = true and check if due date+time passed
        // note: that double check may not be necessary
        if (task.repeat_done.unwrap() && now_date > task.date.clone().unwrap())
            | (task.repeat_done.unwrap()
                && now_date == task.date.clone().unwrap()
                && now_time > task.time.clone().unwrap())
        {
            // get new original date+time, and prepare to change string to naivedatetime
            // note: design decision time. for finished repeating tasks, should I use the original due datetime as
            // the starting datetime for a refreshed repeating task? Or should I use Local::now as the starting
            // datetime, aka the moment that this function is called to print the list?
            //
            // answer: i think i'll keep the original due datetime -> new starting datetime feature. i think this is
            // to avoid having two different repeating-type tasks, and this is much cleaner + easier. i can
            // just do a rp-reset command that resets the starting datetime to local now
            // 2) And also to keep the starting + ending datetimes consistent and not actually contingent
            // on when the list was shown/printed
            let new_original_date = task.date.clone().unwrap();
            let new_original_time = task.time.clone().unwrap();
            let add_to_this = new_original_date.clone() + " " + &new_original_time;

            // change to naivedatetime
            let mut change_to_date_time_and_add = match NaiveDateTime::parse_from_str(add_to_this.as_str(), "%Y-%m-%d %H:%M") {
                Ok(datetime) => datetime,
                Err(_) => {
                    eprintln!("ERROR: While changing a repeating task that was marked done back to todo since the due date had passed, there was an error with parsing the set date and time to a NaiveDateTime struct. Parsing it to that ensures that I can set a new due date and time since the old due date + time passed and the repeating task is now not done. You should never be able to see this");
                    exit(1);
                }
            };

            // based on the time unit, add to naivedatetime
            match task.repeat_unit.clone().unwrap().as_str() {
                "minutes" | "minute" => change_to_date_time_and_add += Duration::minutes(task.repeat_number.unwrap().into()),
                "hours" | "hour" => change_to_date_time_and_add += Duration::hours(task.repeat_number.unwrap().into()),
                "days" | "day" => {
                    change_to_date_time_and_add = change_to_date_time_and_add
                        .checked_add_days(Days::new(task.repeat_number.unwrap().into()))
                        .unwrap()
                }
                "weeks" | "week" => {
                    let interval: u64 = task.repeat_number.unwrap().into();
                    change_to_date_time_and_add = change_to_date_time_and_add
                        .checked_add_days(Days::new(interval * 7))
                        .unwrap()
                }
                "months" | "month" => {
                    change_to_date_time_and_add = change_to_date_time_and_add
                        .checked_add_months(Months::new(task.repeat_number.unwrap()))
                        .unwrap()
                }
                "years" | "year" => {
                    change_to_date_time_and_add = change_to_date_time_and_add
                        .checked_add_months(Months::new(task.repeat_number.unwrap() * 12))
                        .unwrap()
                }
                _ => (),
            }

            // get new due date+time as string
            let new_date = format!("{}", change_to_date_time_and_add.format("%Y-%m-%d"));
            let new_time = format!("{}", change_to_date_time_and_add.format("%H:%M"));

            // create a new task (borrow checker), set new due date+time, new original date+time, and repeat_done = false
            let new_task = Task {
                task: task.task.clone(),
                date: Some(new_date),
                time: Some(new_time),
                repeat_number: task.repeat_number,
                repeat_unit: task.repeat_unit.clone(),
                repeat_done: Some(false),
                repeat_original_date: Some(new_original_date),
                repeat_original_time: Some(new_original_time),
            };
            repeating_tasks.todo.push(new_task);
            // i'm pretty sure this is an expensive action
            remove_these.push(task.to_owned());
        }
    });
    remove_these.iter().for_each(|task| {
        repeating_tasks.done.retain(|i| *i != *task);
    });

    // sort before changing to strings
    repeating_tasks
        .todo
        .sort_by_key(|item| (item.date.to_owned().unwrap(), item.time.to_owned().unwrap()));
    repeating_tasks
        .done
        .sort_by_key(|item| (item.date.clone().unwrap(), item.time.clone().unwrap()));

    let mut repeating_todo = String::from("");
    let mut counter: u8 = 1;
    repeating_tasks.todo.iter().for_each(|item| {
        let task = format!(
            "{}: {}\n   interval: every {} {}\n   {}: {} {}\n",
            counter,
            item.task,
            item.repeat_number.unwrap(),
            item.repeat_unit.clone().unwrap(),
            check_if_due_or_not(item.date.clone().unwrap(), item.time.clone().unwrap()),
            item.date.clone().unwrap(),
            item.time.clone().unwrap()
        );
        counter += 1;
        repeating_todo.push_str(&task);
    });
    let repeating_todo = repeating_todo.trim_end();

    let mut repeating_done = String::from("");
    let mut counter: u8 = 1;
    repeating_tasks.done.iter().for_each(|item| {
        let task = format!(
            "{}: {}\n   interval: every {} {}\n   done: {} {}\n",
            counter,
            item.task,
            item.repeat_number.unwrap(),
            item.repeat_unit.clone().unwrap(),
            item.date.clone().unwrap(),
            item.time.clone().unwrap()
        );
        counter += 1;
        repeating_done.push_str(&task);
    });
    let repeating_done = repeating_done.trim_end();

    // write changes to file. wanted to do this after sorting, but for borrowing reasons, can't
    write_changes_to_new_repeating_tasks(repeating_tasks);

    match repeating_done.is_empty() {
        true => (repeating_todo.to_string(), repeating_done.to_string()),
        false => {
            let repeating_done = "DONE\n---\n".to_string() + repeating_done;
            (repeating_todo.to_string(), repeating_done)
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
