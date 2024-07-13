use super::deadline_helpers::*;
use crate::functions::json_file_structs::*;
use std::io::Write;
use chrono::{NaiveDate, NaiveTime};


// chartodo dl-a new-item 2025-01-01 00:00 > len = 3
// chartodo dl-a new-item 2025-01-01 00:00 2nd-item 2025-01-02 00:00 > len = 6

// chartodo dl-ant new-item 2025-01-01 > len = 2

// chartodo dl-and new-item 00:00 > len = 2

pub fn deadline_tasks_add(add: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if we have the right # of args
    // note/potential todo: i'd like to remove division here but idk what else to do lol
    if add.len() % 3 != 0 {
        return writeln!(writer, "You don't have the right amount of arguments when adding a deadline task. Proper example: chartodo dl-a new-item 2099-01-01 00:00. Another: chartodo dl-a new-item 2099-01-01 00:00 another-item 2199-01-01 23:59. After the command dl-a, there should be 3, 6, 9, etc. arguments.").expect("writeln failed");
    }

    // check how many sets of arguments there are 
    let mut counter = add.len() / 3;

    // loop thru the deadline args and parse for correctness
    // i'm looping from back to front, and that's the order that the new deadline tasks are gonna be added
    let mut new_deadlines: Vec<Task> = vec![];
    while counter > 0 {
        // note for interviews: my first instinct was to keep accessing last elem and delete as i go

        // time: get counter * 3 - 1
        // date: get counter * 3 - 2
        // task: get counter * 3 - 3

        // create new Task struct
        let mut deadline_task = Task {
            task: "".to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: None,
        };

        // check time. if correct, change format and add to struct
        let time: NaiveTime = match add.get(counter * 3 - 1).unwrap().parse() {
            Ok(yes) => yes,
            Err(_) => return writeln!(writer, "Your specified time in argument set {} was invalid. Please provide a correct time in a 24-hour format, e.g. 20:05.", counter).expect("writeln failed"),
        };
        deadline_task.time = Some(format!("{}", time.format("%H:%M")));

        // check date and add to struct
        let date: NaiveDate = match add.get(counter * 3 - 2).unwrap().parse() {
            Ok(yes) => yes,
            Err(_) => return writeln!(writer, "Your specified date in argument set {} was invalid. Please provide a correct time in a year-month-day format, e.g. 2099-12-12.", counter).expect("writeln failed"),
        };
        deadline_task.date = Some(date.to_string());

        // check task is not over 40 chars. add to struct
        if add.get(counter * 3 - 3).unwrap().len() > 40 {
            return writeln!(writer, "Your specified deadline task in argument set {} was over 40 characters long, which is not allowed.", counter).expect("writeln failed");
        };
        deadline_task.task = add.get(counter * 3 - 3).unwrap().to_string();

        // push new correct Task to a vec
        new_deadlines.push(deadline_task);

        counter -= 1;
    }

    // one by one, add new deadline tasks
    new_deadlines.iter().for_each(|task| deadline_tasks.todo.push(task.to_owned()));

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_add_no_time(add_no_time: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if right # of arguments
    if add_no_time.len() % 2 != 0 {
        return writeln!(writer, "You don't have the right amount of arguments when adding a deadline task w/ no time. Proper example: chartodo dl-ant new-item 2099-01-01. Another: chartodo dl-a new-item 2099-01-01 another-item 2199-01-01. After the command dl-ant, there should be 2, 4, 6, etc. arguments.").expect("writeln failed");
    }

    // check how many sets of arguments there are 
    let mut counter = add_no_time.len() / 2;

    // loop thru the deadline args and parse for correctness
    // i'm looping from back to front, and that's the order that the new deadline tasks are gonna be added
    let mut new_deadlines: Vec<Task> = vec![];
    while counter > 0 {
        // note for interviews: my first instinct was to keep accessing last elem and delete as i go

        // date: get counter * 2 - 1
        // task: get counter * 2 - 2

        // create new Task struct
        let mut deadline_task = Task {
            task: "".to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: None,
        };

        // check date and add to struct
        let date: NaiveDate = match add_no_time.get(counter * 2 - 1).unwrap().parse() {
            Ok(yes) => yes,
            Err(_) => return writeln!(writer, "Your specified date in argument set {} was invalid. Please provide a correct time in a year-month-day format, e.g. 2099-12-12.", counter).expect("writeln failed"),
        };
        deadline_task.date = Some(date.to_string());

        // check task is not over 40 chars. add to struct
        if add_no_time.get(counter * 2 - 2).unwrap().len() > 40 {
            return writeln!(writer, "Your specified deadline task in argument set {} was over 40 characters long, which is not allowed.", counter).expect("writeln failed");
        };
        deadline_task.task = add_no_time.get(counter * 2 - 2).unwrap().to_string();

        // default time: 00:00
        deadline_task.time = Some("00:00".to_string());

        // push new correct Task to a vec
        new_deadlines.push(deadline_task);

        counter -= 1;
    }

    // one by one, add new deadline tasks
    new_deadlines.iter().for_each(|task| deadline_tasks.todo.push(task.to_owned()));

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}