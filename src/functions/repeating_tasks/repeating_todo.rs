use super::repeating_helpers::*;
use crate::functions::json_file_structs::*;
use chrono::{Days, Duration, Local, Months, NaiveDate, NaiveDateTime, NaiveTime};
use std::io::Write;

// chartodo rp-a rp_task_1 3 days rp_task_2 4 days => len % 3

pub fn repeating_tasks_add(add: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if we have the right # of args
    // note/potential todo: i'd like to remove division here but idk what else to do lol
    if add.len() % 3 != 0 {
        return writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a repeating task. Proper example: chartodo rp-a new-item 3 days. Another: chartodo rp-a new-item 3 days another-item 4 years. After the command rp-a, there should be 3, 6, 9, etc. arguments.").expect("writeln failed");
    }

    // check how many sets of arguments there are
    let mut counter = add.len() / 3;

    // check if adding to repeating would overflow it past 15
    if counter + repeating_tasks.todo.len() > 15 {
        return writeln!(writer, "You're trying to add too many repeating tasks, as it would exceed the repeating todo list's max len of 15. Try removing at least {} items.", 15 - repeating_tasks.todo.len()).expect("writeln failed");
    }

    // loop thru the deadline args and parse for correctness
    // i'm looping from back to front, and that's the order that the new deadline tasks are gonna be added
    let mut new_repeatings: Vec<Task> = vec![];
    while counter > 0 {
        // note for interviews: my first instinct was to keep accessing last elem and delete as i go

        // unit: get counter * 3 - 1
        // interval: get counter * 3 - 2
        // task: get counter * 3 - 3

        // check if the unit is proper
        match add.get(counter * 3 - 1).unwrap().as_str() {
            "minutes" | "minute" | "hours" | "hour" | "days" | "day" | "weeks" | "week" | "months" | "month" | "years" | "year" => (),
            _ => return writeln!(writer, "ERROR: You didn't provide a proper time unit. It has to be one of the following: minutes, hours, days, weeks, months, years.
                NOTE: nothing on the list below changed.").expect("writeln failed")
        }

        // check if the interval is proper. has to be u32
        let interval: u32 = match add.get(counter * 3 - 2).unwrap().parse() {
            Ok(number) => number,
            Err(_) => return writeln!(writer, "ERROR: You didn't provide a proper interval for your repeating task. It can't be negative and can't be above 4294967295. Proper example: chartodo rp-a gym 2 days.
                NOTE: nothing on the list below changed.").expect("writeln failed"),
        };

        // check if interval is 0
        if add.get(counter * 3 - 2).unwrap().parse::<u32>().unwrap() == 0 {
            return writeln!(writer, "ERROR: You can't have an interval of 0, otherwise why are you even making a new repeating task?
                NOTE: nothing on the list below changed.").expect("writeln failed");
        }

        // create new Task struct
        let mut repeating_task = Task {
            task: "".to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: Some(false),
            repeat_original_date: None,
            repeat_original_time: None,
        };

        // check task is not over 40 chars. add to struct
        if add.get(counter * 3 - 3).unwrap().len() > 40 {
            return writeln!(writer, "ERROR: Your specified repeating task in argument set {} was over 40 characters long, which is not allowed.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed");
        };
        repeating_task.task = add.get(counter * 3 - 3).unwrap().to_string();

        // set interval and unit
        repeating_task.repeat_number = Some(interval);
        repeating_task.repeat_unit = Some(add.get(counter * 3 - 1).unwrap().to_owned());

        // get the date, time, repeat_original_date, and repeat_original_time
        let (date, time, repeat_original_date, repeat_original_time) =
            add_to_local_now(interval, add.get(counter * 3 - 1).unwrap().to_owned());

        // set the remaining fields
        repeating_task.date = Some(date);
        repeating_task.time = Some(time);
        repeating_task.repeat_original_date = Some(repeat_original_date);
        repeating_task.repeat_original_time = Some(repeat_original_time);

        // push new correct Task to a vec
        new_repeatings.push(repeating_task);

        counter -= 1;
    }

    // one by one, add new repeating tasks
    new_repeatings
        .iter()
        .for_each(|task| repeating_tasks.todo.push(task.to_owned()));

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

fn add_to_local_now(interval: u32, unit: String) -> (String, String, String, String) {
    // get two local nows: one for adding, the other to keep track of when task was set
    let mut add_to_now = Local::now();
    let return_original_now = add_to_now;

    match unit.as_str() {
        "minutes" | "minute" => add_to_now += Duration::minutes(interval.into()),
        "hours" | "hour" => add_to_now += Duration::hours(interval.into()),
        "days" | "day" => {
            add_to_now = add_to_now
                .checked_add_days(Days::new(interval.into()))
                .unwrap()
        }
        "weeks" | "week" => {
            let interval: u64 = interval.into();
            add_to_now = add_to_now
                .checked_add_days(Days::new(interval * 7))
                .unwrap()
        }
        "months" | "month" => {
            add_to_now = add_to_now
                .checked_add_months(Months::new(interval))
                .unwrap()
        }
        "years" | "year" => {
            add_to_now = add_to_now
                .checked_add_months(Months::new(interval * 12))
                .unwrap()
        }
        _ => (),
    }

    let date = format!("{}", add_to_now.format("%Y-%m-%d"));
    let time = format!("{}", add_to_now.format("%H:%M"));
    let repeat_original_date = format!("{}", return_original_now.format("%Y-%m-%d"));
    let repeat_original_time = format!("{}", return_original_now.format("%H:%M"));

    (date, time, repeat_original_date, repeat_original_time)
}

// note to self: i have to give users the ability to edit the due date and time of repeating tasks
// reasoning: idk i forgot. if i can't come up with/remember the reason for thinking this, this is off the table
// reason: users might want to keep the interval + unit the same but edit the starting/ending datetime of the task

pub fn repeating_tasks_add_start_datetime(start: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // chartodo repeating-addstart task 3 days 2022-01-01 00:00 task2 3 days 2023-01-01 00:00 => len % 5

    // check if we have the right # of args
    if start.len() % 5 != 0 {
        return writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific starting datetime. Proper example: chartodo rp-as new-item 3 days 2099-01-01 00:00. Another: chartodo rp-a new-item 3 days 2099-01-01 00:00 another-item 4 years. After the command rp-a, there should be 5, 10, 12, etc. arguments.
            NOTE: nothing on the list below changed.").expect("writeln failed");
    }

    // check how many sets of arguments there are
    let mut counter = start.len() / 5;

    // check if adding to repeating would overflow it past 15
    if counter + repeating_tasks.todo.len() > 15 {
        return writeln!(writer, "You're trying to add too many repeating tasks, as it would exceed the repeating todo list's max len of 15. Try removing at least {} items.
            NOTE: nothing changed on the list below.", 15 - repeating_tasks.todo.len()).expect("writeln failed");
    }

    // loop thru the deadline args and parse for correctness
    // i'm looping from back to front, and that's the order that the new deadline tasks are gonna be added
    let mut new_repeating_starts: Vec<Task> = vec![];
    while counter > 0 {
        // chartodo repeating-addstart task 3 days 2022-01-01 00:00 task2 3 days 2023-01-01 00:00 => len % 5

        // start_time: get counter * 5 - 1
        // start_date: get counter * 5 - 2
        // unit: get counter * 5 - 3
        // interval: get counter * 5 - 4
        // task: get counter * 5 - 5

        // check if the starting time is proper
        match NaiveTime::parse_from_str(start.get(counter * 5 - 1).unwrap().as_str(), "%H:%M") {
            Ok(_) => (),
            Err(_) => return writeln!(writer, "ERROR: You didn't provide a proper starting time in argument set {}. Provide a correct starting time in a 24-hour format, e.g., 23:04.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed")
        };

        // check if starting date is proper
        match NaiveDate::parse_from_str(start.get(counter * 5 - 2).unwrap().as_str(), "%Y-%m-%d") {
            Ok(_) => (),
            Err(_) => return writeln!(writer, "ERROR: You didn't provide a proper starting date in argument set {}. Provide a correct starting date in a year-month-day format, e.g., 2024-05-12.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed")
        };

        // check if unit time is proper
        match start.get(counter * 5 - 3).unwrap().as_str() {
            "minutes" | "minute" | "hours" | "hour" | "days" | "day" | "weeks" | "week" | "months" | "month" | "years" | "year" => (),
            _ => return writeln!(writer, "ERROR: You didn't provide a proper time unit in argument set {}. It has to be one of the following: minutes, hours, days, weeks, months, years.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed")
        }

        // check if the interval is proper
        let interval: u32 = match start.get(counter * 5 - 4).unwrap().parse() {
            Ok(number) => number,
            Err(_) => return writeln!(writer, "ERROR: You didn't provide a proper interval in argument set {}. It can't be negative and can't be above 4294967295. Proper example: chartodo rp-a gym 2 days.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed"),
        };

        // check if interval is 0
        if start.get(counter * 5 - 4).unwrap().parse::<u32>().unwrap() == 0 {
            return writeln!(writer, "ERROR: You provided an interval of 0 in argument set {}. You can't have an interval of 0, otherwise why are you even making a new repeating task?
                NOTE: nothing on the list below changed.", counter).expect("writeln failed");
        }

        // create new Task struct
        let mut repeating_task_start = Task {
            task: "".to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: Some(false),
            repeat_original_date: None,
            repeat_original_time: None,
        };

        // check task is not over 40 chars. add to struct
        if start.get(counter * 5 - 5).unwrap().len() > 40 {
            return writeln!(writer, "ERROR: Your specified repeating task in argument set {} was over 40 characters long, which is not allowed.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed");
        };
        repeating_task_start.task = start.get(counter * 5 - 5).unwrap().to_string();

        // set interval and unit
        repeating_task_start.repeat_number = Some(interval);
        repeating_task_start.repeat_unit = Some(start.get(counter * 5 - 3).unwrap().to_owned());

        // get the date, time, repeat_original_date, and repeat_original_time
        let (date, time, repeat_original_date, repeat_original_time) =
            add_to_given_starting_datetime(
                start.get(counter * 5 - 2).unwrap().to_string(),
                start.get(counter * 5 - 1).unwrap().to_string(),
                interval,
                start.get(counter * 5 - 3).unwrap().to_owned(),
            );

        // set the remaining fields
        repeating_task_start.date = Some(date);
        repeating_task_start.time = Some(time);
        repeating_task_start.repeat_original_date = Some(repeat_original_date);
        repeating_task_start.repeat_original_time = Some(repeat_original_time);

        // push new correct Task to a vec
        new_repeating_starts.push(repeating_task_start);

        counter -= 1;
    }

    // one by one, add new repeating tasks with starts
    new_repeating_starts
        .iter()
        .for_each(|task| repeating_tasks.todo.push(task.to_owned()));

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

fn add_to_given_starting_datetime(
    start_date: String,
    start_time: String,
    interval: u32,
    unit: String,
) -> (String, String, String, String) {
    // get a starting datetime to add to it
    // note: i wish i didn't have to do naivedate + naivetime -> str -> naivedatetime
    // note: this separate formatting + combining into string is necessary for correct parsing
    // let start_date = format!("{}", start_date.format("%Y-%m-%d"));
    // let start_time = format!("{}", start_time.format("%H:%M"));
    let starting_datetime = start_date + " " + &start_time;
    let starting_datetime = NaiveDateTime::parse_from_str(starting_datetime.as_str(), "%Y-%m-%d %H:%M").expect("You should never be able to see this error. Somehow, when parsing a datetime from str in fn add_to_given_starting_datetime in path src/functions/repeating_tasks/repeating_todo.rs, the parsing failed. Should never happen since there were several checks that happened up to this point. If you see this, please open an issue on github.");
    let mut add_to_start = starting_datetime;

    match unit.as_str() {
        "minutes" | "minute" => add_to_start += Duration::minutes(interval.into()),
        "hours" | "hour" => add_to_start += Duration::hours(interval.into()),
        "days" | "day" => {
            add_to_start = add_to_start
                .checked_add_days(Days::new(interval.into()))
                .unwrap()
        }
        "weeks" | "week" => {
            let interval: u64 = interval.into();
            add_to_start = add_to_start
                .checked_add_days(Days::new(interval * 7))
                .unwrap()
        }
        "months" | "month" => {
            add_to_start = add_to_start
                .checked_add_months(Months::new(interval))
                .unwrap()
        }
        "years" | "year" => {
            add_to_start = add_to_start
                .checked_add_months(Months::new(interval * 12))
                .unwrap()
        }
        // this arm should never activate
        _ => (),
    }

    let date = format!("{}", add_to_start.format("%Y-%m-%d"));
    let time = format!("{}", add_to_start.format("%H:%M"));
    let repeat_original_date = format!("{}", starting_datetime.format("%Y-%m-%d"));
    let repeat_original_time = format!("{}", starting_datetime.format("%H:%M"));

    (date, time, repeat_original_date, repeat_original_time)
}

pub fn repeating_tasks_add_end(add_end: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // chartodo repeating-addend task 3 days 2030-01-01 00:00 task2 4 months 2031-01-01 00:00 => len % 5

    // check if we have the right # of args
    if add_end.len() % 5 != 0 {
        return writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific ending datetime. Proper example: chartodo rp-ae new-item 3 days 2099-01-01 00:00. Another: chartodo rp-ae new-item 3 days 2099-01-01 00:00 another-item 4 years. After the command rp-ae, there should be 5, 10, 12, etc. arguments.
            NOTE: nothing on the list below changed.").expect("writeln failed");
    }

    // check how many sets of arguments there are
    let mut counter = add_end.len() / 5;

    // check if the user wants to add over 15 items
    if counter > 15 {
        return writeln!(writer, "ERROR: You want to add over 15 repeating tasks. Regardless of how many repeating tasks there currently are in the todo list, 15 is the maximum length for the todo list and you cannot add more than 15.
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // check if adding to repeating would overflow it past 15
    // note that this only triggers if the user only wants to add less than 15
    if counter + repeating_tasks.todo.len() > 15 {
        return writeln!(writer, "You're trying to add too many repeating tasks, as it would exceed the repeating todo list's max len of 15. Try removing at least {} items.
            NOTE: nothing changed on the list below.", (repeating_tasks.todo.len() + counter) - 15).expect("writeln failed");
    }

    // loop thru the deadline args and parse for correctness
    // i'm looping from back to front, and that's the order that the new deadline tasks are gonna be added
    let mut new_repeating_ends: Vec<Task> = vec![];
    while counter > 0 {
        // chartodo repeating-addend task 3 days 2030-01-01 00:00 task2 4 months 2031-01-01 00:00 => len % 5

        // end_time: get counter * 5 - 1
        // end_date: get counter * 5 - 2
        // unit: get counter * 5 - 3
        // interval: get counter * 5 - 4
        // task: get counter * 5 - 5

        // check if the starting time is proper
        match NaiveTime::parse_from_str(add_end.get(counter * 5 - 1).unwrap().as_str(), "%H:%M") {
            Ok(_) => (),
            Err(_) => return writeln!(writer, "ERROR: You didn't provide a proper ending time in argument set {}. Provide a correct ending time in a 24-hour format, e.g., 23:04.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed")
        }

        // check if starting date is proper
        match NaiveDate::parse_from_str(add_end.get(counter * 5 - 2).unwrap().as_str(), "%Y-%m-%d") {
            Ok(end_date) => end_date,
            Err(_) => return writeln!(writer, "ERROR: You didn't provide a proper ending date in argument set {}. Provide a correct ending date in a year-month-day format, e.g., 2024-05-12.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed")
        };

        // check if unit time is proper
        match add_end.get(counter * 5 - 3).unwrap().as_str() {
            "minutes" | "minute" | "hours" | "hour" | "days" | "day" | "weeks" | "week" | "months" | "month" | "years" | "year" => (),
            _ => return writeln!(writer, "ERROR: You didn't provide a proper time unit in argument set {}. It has to be one of the following: minutes, hours, days, weeks, months, years.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed")
        }

        // check if the interval is proper
        let interval: u32 = match add_end.get(counter * 5 - 4).unwrap().parse() {
            Ok(number) => number,
            Err(_) => return writeln!(writer, "ERROR: You didn't provide a proper interval in argument set {}. It can't be negative and can't be above 4294967295. Proper example: chartodo rp-ae gym 2 days 2000-01-01.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed"),
        };

        // check if interval is 0
        if interval == 0 {
            return writeln!(writer, "ERROR: You provided an interval of 0 in argument set {}. You can't have an interval of 0, otherwise why are you even making a new repeating task?
                NOTE: nothing on the list below changed.", counter).expect("writeln failed");
        }

        // create new Task struct
        let mut repeating_task_end = Task {
            task: "".to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: Some(false),
            repeat_original_date: None,
            repeat_original_time: None,
        };

        // check task is not over 40 chars. add to struct
        if add_end.get(counter * 5 - 5).unwrap().len() > 40 {
            return writeln!(writer, "ERROR: Your specified repeating task in argument set {} was over 40 characters long, which is not allowed.
                NOTE: nothing on the list below changed.", counter).expect("writeln failed");
        };
        repeating_task_end.task = add_end.get(counter * 5 - 5).unwrap().to_string();

        // set interval and unit
        repeating_task_end.repeat_number = Some(interval);
        repeating_task_end.repeat_unit = Some(add_end.get(counter * 5 - 3).unwrap().to_owned());

        // get the date, time, repeat_original_date, and repeat_original_time
        let (date, time, repeat_original_date, repeat_original_time) =
            subract_from_given_ending_datetime(
                add_end.get(counter * 5 - 2).unwrap().to_string(),
                add_end.get(counter * 5 - 1).unwrap().to_string(),
                interval,
                add_end.get(counter * 5 - 3).unwrap().to_owned(),
            );

        // set the remaining fields
        repeating_task_end.date = Some(date);
        repeating_task_end.time = Some(time);
        repeating_task_end.repeat_original_date = Some(repeat_original_date);
        repeating_task_end.repeat_original_time = Some(repeat_original_time);

        // push new correct Task to a vec
        new_repeating_ends.push(repeating_task_end);

        counter -= 1;
    }

    // one by one, add new repeating tasks with starts
    new_repeating_ends
        .iter()
        .for_each(|task| repeating_tasks.todo.push(task.to_owned()));

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

fn subract_from_given_ending_datetime(
    end_date: String,
    end_time: String,
    interval: u32,
    unit: String,
) -> (String, String, String, String) {
    // get a starting datetime to add to it
    // note: i wish i didn't have to do naivedate + naivetime -> str -> naivedatetime
    // note: this separate formatting + combining into string is necessary for correct parsing
    // let end_date = format!("{}", end_date.format("%Y-%m-%d"));
    // let end_time = format!("{}", end_time.format("%H:%M"));
    let ending_datetime = end_date + " " + &end_time;
    let ending_datetime = NaiveDateTime::parse_from_str(ending_datetime.as_str(), "%Y-%m-%d %H:%M").expect("You should never be able to see this error. Somehow, when parsing a datetime from str in fn subtract_from_given_ending_datetime in path src/functions/repeating_tasks/repeating_todo.rs, the parsing failed. Should never happen since there were several checks that happened up to this point. If you see this, please open an issue on github.");
    let mut subtract_from_end = ending_datetime;

    match unit.as_str() {
        "minutes" | "minute" => subtract_from_end -= Duration::minutes(interval.into()),
        "hours" | "hour" => subtract_from_end -= Duration::hours(interval.into()),
        "days" | "day" => {
            subtract_from_end = subtract_from_end
                .checked_sub_days(Days::new(interval.into()))
                .unwrap()
        }
        "weeks" | "week" => {
            let interval: u64 = interval.into();
            subtract_from_end = subtract_from_end
                .checked_sub_days(Days::new(interval * 7))
                .unwrap()
        }
        "months" | "month" => {
            subtract_from_end = subtract_from_end
                .checked_sub_months(Months::new(interval))
                .unwrap()
        }
        "years" | "year" => {
            subtract_from_end = subtract_from_end
                .checked_sub_months(Months::new(interval * 12))
                .unwrap()
        }
        // this arm should never activate
        _ => (),
    }

    let date = format!("{}", ending_datetime.format("%Y-%m-%d"));
    let time = format!("{}", ending_datetime.format("%H:%M"));
    let repeat_original_date = format!("{}", subtract_from_end.format("%Y-%m-%d"));
    let repeat_original_time = format!("{}", subtract_from_end.format("%H:%M"));

    (date, time, repeat_original_date, repeat_original_time)
}

pub fn repeating_tasks_done(done: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty. Try adding items to it first."
        )
        .expect("writeln failed");
    }

    // filter for viable positions
    let mut dones: Vec<usize> = vec![];
    done.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= repeating_tasks.todo.len()
        {
            dones.push(item.parse().unwrap());
        }
    });
    drop(done);

    // reverse sort the positions
    dones.sort();
    dones.reverse();
    dones.dedup();

    // check if the user basically specified the entire list
    if dones.len() >= repeating_tasks.todo.len() && repeating_tasks.todo.len() > 5 {
        return writeln!(
            writer,
            "ERROR: You've specified the entire repeating todo list that's relatively long. Might as well do chartodo repeating-doneall
            NOTE: nothing on the list below has changed"
        )
        .expect("writeln failed");
    }

    // if changing todos to done means the done list overflows, clear done list
    if dones.len() + repeating_tasks.done.len() > 10 {
        repeating_tasks.done.clear();
    }

    // before pushing to done, change each repeat_done field in each specified todo to true
    dones.iter().for_each(|position| {
        repeating_tasks
            .todo
            .get_mut(*position - 1)
            .unwrap()
            .repeat_done = Some(true);
    });

    // change todos to dones one by one
    dones.iter().for_each(|position| {
        repeating_tasks
            .done
            .push(repeating_tasks.todo.get(*position - 1).unwrap().to_owned());
        repeating_tasks.todo.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_reset_original_datetime_to_now(reset: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // filter for viable positions
    let mut resets: Vec<usize> = vec![];
    reset.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= repeating_tasks.todo.len()
        {
            resets.push(item.parse().unwrap());
        }
    });
    drop(reset);

    // reverse sort the positions
    resets.sort();
    resets.reverse();
    resets.dedup();

    // check if the user basically specified the entire list
    if resets.len() >= repeating_tasks.todo.len() && repeating_tasks.todo.len() > 5 {
        return writeln!(
            writer,
            "ERROR: You've specified the entire repeating todo list that's relatively long. Might as well do chartodo repeating-resetall
            NOTE: nothing on the list below has changed"
        )
        .expect("writeln failed");
    }

    // reset each original datetime to local::now and make new due datetimes
    resets.iter().for_each(|position| {
        // create new original + due datetimes
        let (date, time, repeat_original_date, repeat_original_time) = add_to_local_now(
            repeating_tasks
                .todo
                .get(*position - 1)
                .unwrap()
                .repeat_number
                .unwrap(),
            repeating_tasks
                .todo
                .get(*position - 1)
                .unwrap()
                .repeat_unit
                .as_ref()
                .unwrap()
                .to_string(),
        );

        // set the new datetimes
        repeating_tasks
            .todo
            .get_mut(*position - 1)
            .unwrap()
            .repeat_original_date = Some(repeat_original_date);
        repeating_tasks
            .todo
            .get_mut(*position - 1)
            .unwrap()
            .repeat_original_time = Some(repeat_original_time);
        repeating_tasks.todo.get_mut(*position - 1).unwrap().date = Some(date);
        repeating_tasks.todo.get_mut(*position - 1).unwrap().time = Some(time);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_rmtodo(rmtodo: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty. Try adding items to it first.
            NOTE: nothing has changed on the list below."
        )
        .expect("writeln failed");
    }

    // filter for viable positions
    let mut rmtodos: Vec<usize> = vec![];
    rmtodo.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= repeating_tasks.todo.len()
        {
            rmtodos.push(item.parse().unwrap());
        }
    });
    drop(rmtodo);

    // reverse sort
    rmtodos.sort();
    rmtodos.reverse();
    rmtodos.dedup();

    // check if user wants to remove all of the items
    if rmtodos.len() >= repeating_tasks.todo.len() && repeating_tasks.todo.len() > 5 {
        return writeln!(
            writer,
            "ERROR: You might as well do repeating-cleartodo since you want to remove all of the items from a relatively long list.
            NOTE: nothing has changed on the list below."
        )
        .expect("writeln failed");
    }

    // remove each item one by one
    rmtodos.iter().for_each(|position| {
        repeating_tasks.todo.remove(*position - 1);
    });
    drop(rmtodos);

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_doneall() {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "The repeating todo list is currently empty, so you can't change any todos to done."
        )
        .expect("writeln failed");
    }

    // clear done list if it will overflow
    if repeating_tasks.todo.len() + repeating_tasks.done.len() > 10 {
        repeating_tasks.done.clear();
    }

    // before pushing, change each repeat_done field to true
    repeating_tasks.todo.iter_mut().for_each(|task| {
        task.repeat_done = Some(true);
    });

    // push all todos to done
    repeating_tasks
        .todo
        .iter()
        .for_each(|item| repeating_tasks.done.push(item.to_owned()));
    repeating_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_clear_todo() {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(writer, "ERROR: The repeating todo list is currently empty. Try adding items to it first before removing any.
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // clear todo list
    repeating_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_show_start(start: Vec<String>) -> String {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();

    // open file and parse
    let repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return String::from(
            "error: the repeating todo list is currently empty. try adding items to it first.",
        );
    }

    // filter for viable positions
    let mut starts: Vec<usize> = vec![];
    start.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= repeating_tasks.todo.len()
        {
            starts.push(item.parse().unwrap());
        }
    });
    drop(start);

    // sort and dedup (sort first cuz dedup is faster if it's sorted, and sort makes it nicer)
    starts.sort();
    starts.dedup();

    // check if user wants to show starts for all of the items
    if starts.len() >= repeating_tasks.todo.len() && repeating_tasks.todo.len() > 5 {
        return String::from("ERROR: you might as well do repeating-startall since you want to show the starting datetimes for all of the repeating tasks.");
    }

    let mut show_starts = String::from("");
    starts.iter().for_each(|position| {
        let task_and_start = format!(
            "task: {}\n\tstart: {} {}\n",
            repeating_tasks.todo.get(*position - 1).unwrap().task,
            repeating_tasks
                .todo
                .get(*position - 1)
                .unwrap()
                .repeat_original_date
                .as_ref()
                .unwrap(),
            repeating_tasks
                .todo
                .get(*position - 1)
                .unwrap()
                .repeat_original_time
                .as_ref()
                .unwrap()
        );
        show_starts.push_str(task_and_start.as_str());
    });
    let show_starts = show_starts.trim_end();

    show_starts.to_string()
}

pub fn repeating_tasks_edit_all(edit_all: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // chartodo rp-ea 1 task 3 days start/end 2000-01-01 00:00. note that the user has to specify if the datetime is the start or end

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_all.len() != 7 {
        return writeln!(writer, "ERROR: You must specify the repeating todo's position and all the parameters that will be edited. A proper example would be: chartodo rp-ea 4 new-item 3 days end 2150-01-01 00:00. If you wanted to edit the starting date instead, replace 'end' with 'start', e.g., chartodo rp-ea 4 new-item 3 days start 2150-01-01 00:00
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // check if position is a valid number
    if edit_all.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "ERROR: You must provide a viable position. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if edit_all.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if edit_all.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        return writeln!(
            writer,
            "ERROR: Your position exceed's the repeating todo list's length. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // new item can't be more than 40 chars
    if edit_all.get(1).unwrap().len() > 40 {
        return writeln!(
            writer,
            "ERROR: Editing a todo item to be more than 40 characters is not allowed.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // interval isn't proper
    if edit_all.get(2).unwrap().parse::<u32>().is_err() {
        return writeln!(
            writer,
            "ERROR: The interval provided isn't proper. It must be in the (inclusive) range of 1 - 4294967295.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // check if interval is 0
    if edit_all.get(2).unwrap().parse::<u32>().unwrap() == 0 {
        return writeln!(
            writer,
            "ERROR: Your interval can't be 0, otherwise why are you even setting a repeating task?
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // unit of time isn't proper
    match edit_all.get(3).unwrap().as_str() {
        "minute" | "minutes" | "hour" | "hours" | "day" | "days" | "week" | "weeks" | "month" | "months" | "year" | "years" => (),
        _ => return writeln!(writer, "ERROR: didn't provide a proper time unit for the interval. Proper examples: minutes, hours, days, weeks, months or years.
            NOTE: nothing changed on the list below.").expect("writeln failed"),
    }

    // date isn't proper
    match NaiveDate::parse_from_str(edit_all.get(5).unwrap().as_str(), "%Y-%m-%d") {
        Ok(_) => (),
        Err(_) => return writeln!(writer, "ERROR: didn't provide a proper date. Must be in the following format: Year-Month-Day, e.g., 2000-01-01.
            NOTE: nothing changed on the list below.").expect("writeln failed"),
    }

    // time isn't proper
    match NaiveTime::parse_from_str(edit_all.last().unwrap().as_str(), "%H:%M") {
        Ok(_) => (),
        Err(_) => return writeln!(writer, "ERROR: didn't provide a proper time. Must be in the following 24-hour format: H:M, e.g., 13:08.
            NOTE: nothing changed on the list below.").expect("writeln failed"),
    }

    // check if it's start or end and do the proper operation
    let (date, time, repeat_original_date, repeat_original_time);
    match edit_all.get(4).unwrap().as_str() {
        "start" => {
            (date, time, repeat_original_date, repeat_original_time) =
                add_to_given_starting_datetime(
                    edit_all.get(5).unwrap().to_string(),
                    edit_all.last().unwrap().to_string(),
                    edit_all.get(2).unwrap().parse::<u32>().unwrap(),
                    edit_all.get(3).unwrap().to_string(),
                )
        }
        "end" => {
            (date, time, repeat_original_date, repeat_original_time) =
                subract_from_given_ending_datetime(
                    edit_all.get(5).unwrap().to_string(),
                    edit_all.last().unwrap().to_string(),
                    edit_all.get(2).unwrap().parse::<u32>().unwrap(),
                    edit_all.get(3).unwrap().to_string(),
                )
        }
        _ => {
           return writeln!(writer, "ERROR: you must specify whether the given datetime is the starting or ending datetime. Please use the 'start' or 'end' keywords and nothing else.
               NOTE: nothing changed on the list below.").expect("writeln failed")
        }
    }

    // get the task and edit
    let position: usize = edit_all.first().unwrap().parse().unwrap();
    repeating_tasks.todo.get_mut(position - 1).unwrap().task = edit_all.get(1).unwrap().to_string();
    repeating_tasks.todo.get_mut(position - 1).unwrap().date = Some(date);
    repeating_tasks.todo.get_mut(position - 1).unwrap().time = Some(time);
    repeating_tasks
        .todo
        .get_mut(position - 1)
        .unwrap()
        .repeat_number = Some(edit_all.get(2).unwrap().parse::<u32>().unwrap());
    repeating_tasks
        .todo
        .get_mut(position - 1)
        .unwrap()
        .repeat_unit = Some(edit_all.get(3).unwrap().to_string());
    repeating_tasks
        .todo
        .get_mut(position - 1)
        .unwrap()
        .repeat_original_date = Some(repeat_original_date);
    repeating_tasks
        .todo
        .get_mut(position - 1)
        .unwrap()
        .repeat_original_time = Some(repeat_original_time);

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_edit_task(edit_task: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // chartodo rp-ea 1 new-task

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_task.len() != 2 {
        return writeln!(writer, "ERROR: You must specify the repeating todo's position and all the new task to change it to. A proper example would be: chartodo rp-eta 4 new-item.
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // check if position is a valid number
    if edit_task.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "ERROR: You must provide a viable position. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if edit_task.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if edit_task.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        return writeln!(
            writer,
            "ERROR: Your position exceed's the repeating todo list's length. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // new item can't be more than 40 chars
    if edit_task.last().unwrap().len() > 40 {
        return writeln!(
            writer,
            "ERROR: Editing a todo item to be more than 40 characters is not allowed.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // get the todo and edit task
    let position: usize = edit_task.first().unwrap().parse().unwrap();
    repeating_tasks.todo.get_mut(position - 1).unwrap().task =
        edit_task.last().unwrap().to_string();

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_edit_interval(edit_interval: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // chartodo rp-ea 1 3

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_interval.len() != 2 {
        return writeln!(writer, "ERROR: You must specify the repeating todo's position and all the parameters that will be edited. A proper example would be: chartodo rp-ea 4 new-item 3 days end 2150-01-01 00:00. If you wanted to edit the starting date instead, replace 'end' with 'start', e.g., chartodo rp-ea 4 new-item 3 days start 2150-01-01 00:00
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // check if position is a valid number
    if edit_interval.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "ERROR: You must provide a viable position. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if edit_interval.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if edit_interval.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        return writeln!(
            writer,
            "ERROR: Your position exceed's the repeating todo list's length. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // interval isn't proper
    if edit_interval.last().unwrap().parse::<u32>().is_err() {
        return writeln!(
            writer,
            "ERROR: The interval provided isn't proper. It must be in the (inclusive) range of 1 - 4294967295.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // check if interval is 0
    if edit_interval.last().unwrap().parse::<u32>().unwrap() == 0 {
        return writeln!(
            writer,
            "ERROR: Your interval can't be 0, otherwise why are you even setting a repeating task?
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // get the updated datetimes
    let position: usize = edit_interval.first().unwrap().parse::<usize>().unwrap() - 1;
    let (date, time, repeat_original_date, repeat_original_time) = add_to_given_starting_datetime(
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_original_date
            .as_ref()
            .unwrap()
            .to_string(),
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_original_time
            .as_ref()
            .unwrap()
            .to_string(),
        edit_interval.last().unwrap().parse::<u32>().unwrap(),
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_unit
            .as_ref()
            .unwrap()
            .to_string(),
    );

    // edit the task
    repeating_tasks.todo.get_mut(position).unwrap().date = Some(date);
    repeating_tasks.todo.get_mut(position).unwrap().time = Some(time);
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_number = Some(edit_interval.last().unwrap().parse::<u32>().unwrap());
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_date = Some(repeat_original_date);
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_time = Some(repeat_original_time);

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_edit_time_unit(edit_unit: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // chartodo rp-ea 1 weeks

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_unit.len() != 2 {
        return writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the interval time unit to. A proper example would be: chartodo rp-ea 4 weeks. That would change repeating task #4's time unit to 'weeks'.
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // check if position is a valid number
    if edit_unit.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "ERROR: You must provide a viable position. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if edit_unit.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if edit_unit.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        return writeln!(
            writer,
            "ERROR: Your position exceed's the repeating todo list's length. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // unit of time isn't proper
    match edit_unit.last().unwrap().as_str() {
        "minute" | "minutes" | "hour" | "hours" | "day" | "days" | "week" | "weeks" | "month" | "months" | "year" | "years" => (),
        _ => return writeln!(writer, "ERROR: didn't provide a proper time unit for the interval. Proper examples: minutes, hours, days, weeks, months or years.
            NOTE: nothing changed on the list below.").expect("writeln failed"),
    }

    // get the updated datetimes
    let position: usize = edit_unit.first().unwrap().parse::<usize>().unwrap() - 1;
    let (date, time, repeat_original_date, repeat_original_time) = add_to_given_starting_datetime(
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_original_date
            .as_ref()
            .unwrap()
            .to_string(),
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_original_time
            .as_ref()
            .unwrap()
            .to_string(),
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_number
            .unwrap(),
        edit_unit.last().unwrap().to_string(),
    );

    // update the datetimes and time unit
    repeating_tasks.todo.get_mut(position).unwrap().date = Some(date);
    repeating_tasks.todo.get_mut(position).unwrap().time = Some(time);
    repeating_tasks.todo.get_mut(position).unwrap().repeat_unit =
        Some(edit_unit.last().unwrap().to_string());
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_date = Some(repeat_original_date);
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_time = Some(repeat_original_time);

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_edit_interval_unit(edit_interval_unit: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // chartodo rp-ea 1 3 days

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_interval_unit.len() != 3 {
        return writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the interval and time unit to. A proper example would be: chartodo rp-ea 4 3 days.
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // check if position is a valid number
    if edit_interval_unit
        .first()
        .unwrap()
        .parse::<usize>()
        .is_err()
    {
        return writeln!(
            writer,
            "ERROR: You must provide a viable position. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if edit_interval_unit
        .first()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        == 0
    {
        return writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if edit_interval_unit
        .first()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        > repeating_tasks.todo.len()
    {
        return writeln!(
            writer,
            "ERROR: Your position exceed's the repeating todo list's length. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // interval isn't proper
    if edit_interval_unit.get(1).unwrap().parse::<u32>().is_err() {
        return writeln!(
            writer,
            "ERROR: The interval provided isn't proper. It must be in the (inclusive) range of 1 - 4294967295.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // check if interval is 0
    if edit_interval_unit.get(1).unwrap().parse::<u32>().unwrap() == 0 {
        return writeln!(
            writer,
            "ERROR: Your interval can't be 0, otherwise why are you even setting a repeating task?
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // unit of time isn't proper
    match edit_interval_unit.last().unwrap().as_str() {
        "minute" | "minutes" | "hour" | "hours" | "day" | "days" | "week" | "weeks" | "month" | "months" | "year" | "years" => (),
        _ => return writeln!(writer, "ERROR: didn't provide a proper time unit for the interval. Proper examples: minutes, hours, days, weeks, months or years.
            NOTE: nothing changed on the list below.").expect("writeln failed"),
    }

    // get the updated datetimes
    let position: usize = edit_interval_unit
        .first()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        - 1;
    let (date, time, repeat_original_date, repeat_original_time) = add_to_given_starting_datetime(
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_original_date
            .as_ref()
            .unwrap()
            .to_string(),
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_original_time
            .as_ref()
            .unwrap()
            .to_string(),
        edit_interval_unit.get(1).unwrap().parse().unwrap(),
        edit_interval_unit.last().unwrap().to_string(),
    );

    // get the task and edit
    repeating_tasks.todo.get_mut(position).unwrap().date = Some(date);
    repeating_tasks.todo.get_mut(position).unwrap().time = Some(time);
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_number = Some(edit_interval_unit.get(1).unwrap().parse::<u32>().unwrap());
    repeating_tasks.todo.get_mut(position).unwrap().repeat_unit =
        Some(edit_interval_unit.last().unwrap().to_string());
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_date = Some(repeat_original_date);
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_time = Some(repeat_original_time);

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_edit_start(edit_start: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // chartodo rp-es 1 2100-01-01 00:00

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_start.len() != 3 {
        return writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the repeating task's starting datetime to. A proper example would be: chartodo rp-es 4 2100-12-24 13:08
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // check if position is a valid number
    if edit_start.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "ERROR: You must provide a viable position. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if edit_start.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if edit_start.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        return writeln!(
            writer,
            "ERROR: Your position exceed's the repeating todo list's length. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // date isn't proper
    match NaiveDate::parse_from_str(edit_start.get(1).unwrap().as_str(), "%Y-%m-%d") {
        Ok(_) => (),
        Err(_) => return writeln!(writer, "ERROR: didn't provide a proper date. Must be in the following format: Year-Month-Day, e.g., 2000-01-01.
            NOTE: nothing changed on the list below.").expect("writeln failed"),
    }

    // time isn't proper
    match NaiveTime::parse_from_str(edit_start.last().unwrap().as_str(), "%H:%M") {
        Ok(_) => (),
        Err(_) => return writeln!(writer, "ERROR: didn't provide a proper time. Must be in the following 24-hour format: H:M, e.g., 13:08.
            NOTE: nothing changed on the list below.").expect("writeln failed"),
    }

    // get the updated datetimes based on the given starting datetime
    let position: usize = edit_start.first().unwrap().parse::<usize>().unwrap() - 1;
    let (date, time, repeat_original_date, repeat_original_time) = add_to_given_starting_datetime(
        edit_start.get(1).unwrap().to_string(),
        edit_start.last().unwrap().to_string(),
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_number
            .unwrap(),
        repeating_tasks
            .todo
            .get(position)
            .unwrap()
            .repeat_unit
            .as_ref()
            .unwrap()
            .to_string(),
    );

    // edit the task
    repeating_tasks.todo.get_mut(position).unwrap().date = Some(date);
    repeating_tasks.todo.get_mut(position).unwrap().time = Some(time);
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_date = Some(repeat_original_date);
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_time = Some(repeat_original_time);

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}

pub fn repeating_tasks_edit_end(edit_end: Vec<String>) {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited.
            NOTE: nothing changed on the list below."
        )
        .expect("writeln failed");
    }

    // chartodo rp-ea 1 2100-12-24 13:08

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_end.len() != 3 {
        return writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the repeating task's ending datetime to. A proper example would be: chartodo rp-ee 4 2100-12-14 13:08
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // check if position is a valid number
    if edit_end.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "ERROR: You must provide a viable position. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if edit_end.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if edit_end.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        return writeln!(
            writer,
            "ERROR: Your position exceed's the repeating todo list's length. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // date isn't proper
    match NaiveDate::parse_from_str(edit_end.get(1).unwrap().as_str(), "%Y-%m-%d") {
        Ok(_) => (),
        Err(_) => return writeln!(writer, "ERROR: didn't provide a proper date. Must be in the following format: Year-Month-Day, e.g., 2000-01-01.
            NOTE: nothing changed on the list below.").expect("writeln failed"),
    }

    // time isn't proper
    match NaiveTime::parse_from_str(edit_end.last().unwrap().as_str(), "%H:%M") {
        Ok(_) => (),
        Err(_) => return writeln!(writer, "ERROR: didn't provide a proper time. Must be in the following 24-hour format: H:M, e.g., 13:08.
            NOTE: nothing changed on the list below.").expect("writeln failed"),
    }

    // get the updated datetimes from the given ending datetime
    let position: usize = edit_end.first().unwrap().parse::<usize>().unwrap() - 1;
    let (date, time, repeat_original_date, repeat_original_time) =
        subract_from_given_ending_datetime(
            edit_end.get(1).unwrap().to_string(),
            edit_end.last().unwrap().to_string(),
            repeating_tasks
                .todo
                .get(position)
                .unwrap()
                .repeat_number
                .unwrap(),
            repeating_tasks
                .todo
                .get(position)
                .unwrap()
                .repeat_unit
                .as_ref()
                .unwrap()
                .to_string(),
        );

    // get the task and edit
    repeating_tasks.todo.get_mut(position).unwrap().date = Some(date);
    repeating_tasks.todo.get_mut(position).unwrap().time = Some(time);
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_date = Some(repeat_original_date);
    repeating_tasks
        .todo
        .get_mut(position)
        .unwrap()
        .repeat_original_time = Some(repeat_original_time);

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);
}
