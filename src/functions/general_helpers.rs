use super::json_file_structs::*;
use super::repeating_tasks::repeating_helpers::*;
use chrono::{Days, Duration, Local, Months, NaiveDateTime};

// these Tasks struct come in already sorted I think
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
            check_if_due_or_not(item.date.as_ref().unwrap(), item.time.as_ref().unwrap()),
            item.date.as_ref().unwrap(),
            item.time.as_ref().unwrap()
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
            item.date.as_ref().unwrap(),
            item.time.as_ref().unwrap()
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

// only possible to unit test if results are MISSED or if date and time are so far beyond current date and time that
// it's impossible to get 'due' unless you spoofed your own time
fn check_if_due_or_not(date: &String, time: &String) -> String {
    if date < &Local::now().date_naive().to_string()
        || date == &Local::now().date_naive().to_string() && time < &Local::now().time().to_string()
    {
        "MISSED".to_string()
    } else {
        "due".to_string()
    }
}

pub fn repeating_tasks_list(mut repeating_tasks: Tasks) -> (String, String) {
    // check if any repeating tasks are done first. if they are, push to todo and remove from done
    // housekeeping
    let now_date = &Local::now().date_naive().to_string();
    let now_time = &Local::now().time().to_string();
    let mut remove_these: Vec<Task> = vec![];

    // pretty sure the following is an expensive action
    let mut check_if_sorted: bool = true;
    repeating_tasks.done.iter().for_each(|task| {
        // double check that repeat_done = true and check if due date+time passed
        // note: that double check may not be necessary
        if (task.repeat_done.unwrap() && now_date > task.date.as_ref().unwrap())
            | (task.repeat_done.unwrap()
                && now_date == task.date.as_ref().unwrap()
                && now_time > task.time.as_ref().unwrap())
        {
            check_if_sorted = false;
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

            // potential TODO: remove these clones? don't think it's possible
            let new_original_date = task.date.clone().unwrap();
            let new_original_time = task.time.clone().unwrap();
            let add_to_this = new_original_date.clone() + " " + &new_original_time;

            // change to naivedatetime. also being random w/ eprintln and exit
            let mut change_to_date_time_and_add =
                match NaiveDateTime::parse_from_str(add_to_this.as_str(), "%Y-%m-%d %H:%M") {
                    Ok(datetime) => datetime,
                    Err(_) => {
                        eprintln!(
                            "ERROR: While changing a repeating task that was \
                            marked done back to todo since the due date had passed, there was an error with \
                            parsing the set date and time to a NaiveDateTime struct. Parsing it to that \
                            ensures that I can set a new due date and time since the old due date + time \
                            passed and the repeating task is now not done. You should never be able to see \
                            this"
                        );
                        std::process::exit(1);
                    }
                };

            // based on the time unit, add to naivedatetime
            match task.repeat_unit.as_ref().unwrap().as_str() {
                "minutes" | "minute" => {
                    change_to_date_time_and_add +=
                        Duration::minutes(task.repeat_number.unwrap().into())
                }
                "hours" | "hour" => {
                    change_to_date_time_and_add +=
                        Duration::hours(task.repeat_number.unwrap().into())
                }
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
    // this is an expensive action.
    // TODO: keep track of index and then remove based on position. can also insert baesd on known position
    // TODO: note that if I can insert and remove instead of push and retain, I have to consider whether sorting would be needed
    // note that inserting and removing based on position can be expensive depending on the position
    // i would also have to remove from highest to lowest position
    remove_these.iter().for_each(|task| {
        repeating_tasks.done.retain(|i| *i != *task);
    });

    // before sorting, check if anything was changed at all
    // sort_by_key is commented out since it uses clone and I'd like to avoid that. still, sort_by may take longer
    /* repeating_tasks
        .todo
        .sort_by_key(|item| (item.date.to_owned().unwrap(), item.time.to_owned().unwrap()));
    repeating_tasks
        .done
        .sort_by_key(|item| (item.date.to_owned().unwrap(), item.time.to_owned().unwrap()));
        */

    if !check_if_sorted {
        repeating_tasks.todo.sort_by(|x, y| {
            match x.date.as_ref().unwrap().cmp(y.date.as_ref().unwrap()) {
                std::cmp::Ordering::Equal => x.time.as_ref().unwrap().cmp(y.time.as_ref().unwrap()),
                lesser_or_greater => lesser_or_greater,
            }
        });
        repeating_tasks.done.sort_by(|x, y| {
            match x.date.as_ref().unwrap().cmp(y.date.as_ref().unwrap()) {
                std::cmp::Ordering::Equal => x.time.as_ref().unwrap().cmp(y.time.as_ref().unwrap()),
                lesser_or_greater => lesser_or_greater,
            }
        });
    }

    let mut repeating_todo = String::from("");
    let mut counter: u8 = 1;
    repeating_tasks.todo.iter().for_each(|item| {
        let task = format!(
            "{}: {}\n   interval: {} {}\n   {}: {} {}\n",
            counter,
            item.task,
            item.repeat_number.unwrap(),
            item.repeat_unit.clone().unwrap(),
            check_if_due_or_not(item.date.as_ref().unwrap(), item.time.as_ref().unwrap()),
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
            "{}: {}\n   interval: {} {}\n   done: {} {}\n",
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
    if !check_if_sorted {
        write_changes_to_new_repeating_tasks(repeating_tasks);
    }

    match repeating_done.is_empty() {
        true => (repeating_todo.to_string(), repeating_done.to_string()),
        false => {
            let repeating_done = "DONE\n---\n".to_string() + repeating_done;
            (repeating_todo.to_string(), repeating_done)
        }
    }
}

// cargo test general_helpers_unit_tests
#[cfg(test)]
mod general_helpers_unit_tests {
    use super::*;

    #[test]
    fn regular_tasks_list_is_correct() {
        let regular_tasks = Tasks {
            todo: vec![
                Task {
                    task: String::from("todo1"),
                    date: None,
                    time: None,
                    repeat_number: None,
                    repeat_unit: None,
                    repeat_done: None,
                    repeat_original_date: None,
                    repeat_original_time: None,
                },
                Task {
                    task: String::from("todo2"),
                    date: None,
                    time: None,
                    repeat_number: None,
                    repeat_unit: None,
                    repeat_done: None,
                    repeat_original_date: None,
                    repeat_original_time: None,
                },
            ],
            done: vec![Task {
                task: String::from("done"),
                date: None,
                time: None,
                repeat_number: None,
                repeat_unit: None,
                repeat_done: None,
                repeat_original_date: None,
                repeat_original_time: None,
            }],
        };
        let correct_todo_string = String::from("1: todo1\n2: todo2");
        let correct_done_string = String::from("DONE\n---\n1: done");
        let (regular_todo, regular_done) = regular_tasks_list(regular_tasks);

        assert_eq!(correct_todo_string, regular_todo);
        assert_eq!(correct_done_string, regular_done);
    }

    #[test]
    fn due_or_not_is_correct() {
        let should_be_missed =
            check_if_due_or_not(&String::from("2020-01-01"), &String::from("00:00"));
        let should_be_due =
            check_if_due_or_not(&String::from("2300-01-01"), &String::from("00:00"));

        assert_eq!(should_be_missed, "MISSED".to_string());
        assert_eq!(should_be_due, "due".to_string());
    }

    #[test]
    fn deadline_tasks_list_is_correct() {
        let deadline_tasks = Tasks {
            todo: vec![
                Task {
                    task: String::from("todo1"),
                    date: Some(String::from("1900-01-01")),
                    time: Some(String::from("00:00")),
                    repeat_number: None,
                    repeat_unit: None,
                    repeat_done: None,
                    repeat_original_date: None,
                    repeat_original_time: None,
                },
                Task {
                    task: String::from("todo2"),
                    date: Some(String::from("2300-01-01")),
                    time: Some(String::from("23:48")),
                    repeat_number: None,
                    repeat_unit: None,
                    repeat_done: None,
                    repeat_original_date: None,
                    repeat_original_time: None,
                },
            ],
            done: vec![Task {
                task: String::from("done"),
                date: Some(String::from("1930-12-25")),
                time: Some(String::from("01:06")),
                repeat_number: None,
                repeat_unit: None,
                repeat_done: None,
                repeat_original_date: None,
                repeat_original_time: None,
            }],
        };
        let correct_todo_string = String::from(
            "1: todo1\n   MISSED: 1900-01-01 00:00\n2: todo2\n   due: 2300-01-01 23:48",
        );
        let correct_done_string = String::from("DONE\n---\n1: done\n   done: 1930-12-25 01:06");
        let (deadline_todo, deadline_done) = deadline_tasks_list(deadline_tasks);

        assert_eq!(correct_todo_string, deadline_todo);
        assert_eq!(correct_done_string, deadline_done);
    }

    #[test]
    fn repeating_tasks_list_is_correct() {
        let repeating_tasks = Tasks {
            todo: vec![
                Task {
                    task: String::from("todo1"),
                    date: Some(String::from("1900-01-01")),
                    time: Some(String::from("00:00")),
                    repeat_number: Some(1),
                    repeat_unit: Some(String::from("year")),
                    repeat_done: Some(false),
                    repeat_original_date: Some(String::from("1899-01-01")),
                    repeat_original_time: Some(String::from("00:00")),
                },
                Task {
                    task: String::from("todo2"),
                    date: Some(String::from("2300-01-01")),
                    time: Some(String::from("23:48")),
                    repeat_number: Some(2),
                    repeat_unit: Some(String::from("months")),
                    repeat_done: Some(false),
                    repeat_original_date: Some(String::from("2299-11-01")),
                    repeat_original_time: Some(String::from("23:48")),
                },
            ],
            done: vec![Task {
                task: String::from("done"),
                date: Some(String::from("2425-12-25")),
                time: Some(String::from("01:06")),
                repeat_number: Some(100),
                repeat_unit: Some("minutes".to_string()),
                repeat_done: Some(true),
                repeat_original_date: Some("2425-12-24".to_string()),
                repeat_original_time: Some("22:40".to_string()),
            }],
        };
        let correct_todo_string = String::from(
            "1: todo1\n   interval: 1 year\n   MISSED: 1900-01-01 00:00\n2: todo2\n   interval: 2 months\n   due: 2300-01-01 23:48",
        );
        let correct_done_string =
            String::from("DONE\n---\n1: done\n   interval: 100 minutes\n   done: 2425-12-25 01:06");
        let (deadline_todo, deadline_done) = repeating_tasks_list(repeating_tasks);

        assert_eq!(correct_todo_string, deadline_todo);
        assert_eq!(correct_done_string, deadline_done);
    }
}
