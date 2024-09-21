use super::repeating_helpers::*;
use crate::functions::json_file_structs::*;
use chrono::{Days, Duration, Local, Months, NaiveDate, NaiveDateTime, NaiveTime};
use std::io::Write;

// chartodo rp-a rp_task_1 3 days rp_task_2 4 days => len % 3

pub fn repeating_tasks_add(add: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if we have the right # of args
    // note/potential todo: i'd like to remove division here but idk what else to do lol
    if add.len() % 3 != 0 {
        writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a repeating task. Proper example: chartodo rp-a new-item 3 days. Another: chartodo rp-a new-item 3 days another-item 4 years. After the command rp-a, there should be 3, 6, 9, etc. arguments.").expect("writeln failed");

        // error = true
        return true;
    }

    let mut counter: usize = 1;
    // loop thru the args and parse for correctness
    while counter <= add.len() / 3 {
        // note for interviews: my first instinct was to keep accessing last elem and delete as i go

        // unit: get counter * 3 - 1
        // interval: get counter * 3 - 2
        // task: get counter * 3 - 3

        // check if the unit is proper
        match add.get(counter * 3 - 1).unwrap().as_str() {
            "minutes" | "minute" | "hours" | "hour" | "days" | "day" | "weeks" | "week"
            | "months" | "month" | "years" | "year" => (),
            _ => {
                writeln!(writer, "ERROR: Your provided time unit, '{}', in argument set '{}', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.
                NOTE: nothing on the list below changed.", add.get(counter * 3 - 1).unwrap(), counter).expect("writeln failed");

                // error = true
                return true;
            }
        }

        // check if the interval is proper. has to be u32
        if add.get(counter * 3 - 2).unwrap().parse::<u32>().is_err() {
            writeln!(writer, "Your provided interval, '{}', in argument set '{}', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-a gym 2 days.", add.get(counter * 3 - 2).unwrap(), counter).expect("writeln failed");

            // error = true
            return true;
        }

        // check if interval is 0
        if add.get(counter * 3 - 2).unwrap().parse::<u32>().unwrap() == 0 {
            writeln!(writer, "ERROR: You had an interval of 0 in argument set '{}'. You can't have an interval of 0, otherwise why are you even making a new repeating task?", add.get(counter * 3 - 2).unwrap()).expect("writeln failed");

            // error = true
            return true;
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

        // add task to struct
        repeating_task.task = add.get(counter * 3 - 3).unwrap().to_string();

        // set interval and unit
        repeating_task.repeat_number =
            Some(add.get(counter * 3 - 2).unwrap().parse::<u32>().unwrap());
        repeating_task.repeat_unit = Some(add.get(counter * 3 - 1).unwrap().to_owned());

        // get the date, time, repeat_original_date, and repeat_original_time
        let (date, time, repeat_original_date, repeat_original_time) = add_to_local_now(
            add.get(counter * 3 - 2).unwrap().parse::<u32>().unwrap(),
            add.get(counter * 3 - 1).unwrap().to_owned(),
        );

        // set the remaining fields
        repeating_task.date = Some(date);
        repeating_task.time = Some(time);
        repeating_task.repeat_original_date = Some(repeat_original_date);
        repeating_task.repeat_original_time = Some(repeat_original_time);

        // push new correct Task
        repeating_tasks.todo.push(repeating_task);

        counter += 1;
    }

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
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

pub fn repeating_tasks_add_start_datetime(start: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // chartodo repeating-addstart task 3 days 2022-01-01 00:00 task2 3 days 2023-01-01 00:00 => len % 5

    // check if we have the right # of args
    if start.len() % 5 != 0 {
        writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific starting datetime. Proper example: chartodo rp-as new-item 3 days 2099-01-01 00:00. Another: chartodo rp-a new-item 3 days 2099-01-01 00:00 another-item 4 years. After the command rp-a, there should be 5, 10, 15, etc. (i.e., divisible by 5) arguments.").expect("writeln failed");

        // error = true
        return true;
    }

    let mut counter: usize = 1;
    // loop thru the args and parse for correctness
    while counter <= start.len() / 5 {
        // chartodo repeating-addstart task 3 days 2022-01-01 00:00 task2 3 days 2023-01-01 00:00 => len % 5

        // start_time: get counter * 5 - 1
        // start_date: get counter * 5 - 2
        // unit: get counter * 5 - 3
        // interval: get counter * 5 - 4
        // task: get counter * 5 - 5

        // check if the starting time is proper
        if NaiveTime::parse_from_str(start.get(counter * 5 - 1).unwrap().as_str(), "%H:%M").is_err()
        {
            writeln!(writer, "ERROR: Your provided starting time, '{}', in argument set '{}', wasn't proper. Please provide a correct starting time in a 24-hour format, e.g., 23:04.", start.get(counter * 5 - 1).unwrap(), counter).expect("writeln failed");

            // error = true
            return true;
        }

        // check if starting date is proper
        if NaiveDate::parse_from_str(start.get(counter * 5 - 4).unwrap().as_str(), "%Y-%m-%d")
            .is_err()
        {
            writeln!(writer, "ERROR: Your provided starting date, '{}', in argument set '{}', wasn't proper. Please provide a correct starting date in a year-month-day format, e.g., 2024-05-12.", start.get(counter * 5 - 4).unwrap(), counter).expect("writeln failed");

            // error = true
            return true;
        }

        // check if unit time is proper
        match start.get(counter * 5 - 3).unwrap().as_str() {
            "minutes" | "minute" | "hours" | "hour" | "days" | "day" | "weeks" | "week"
            | "months" | "month" | "years" | "year" => (),
            _ => {
                writeln!(writer, "ERROR: Your provided time unit, '{}', in argument set '{}', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.", start.get(counter * 5 - 3).unwrap(), counter).expect("writeln failed");

                // error = true
                return true;
            }
        }

        // check if the interval is proper
        if start.get(counter * 5 - 4).unwrap().parse::<u32>().is_err() {
            writeln!(writer, "ERROR: Your provided interval, '{}', in argument set '{}', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-a gym 2 days.", start.get(counter * 5 - 4).unwrap(), counter).expect("writeln failed");

            // error = true
            return true;
        }

        // check if interval is 0
        if start.get(counter * 5 - 4).unwrap().parse::<u32>().unwrap() == 0 {
            writeln!(writer, "ERROR: You provided an interval of 0 in argument set '{}'. You can't have an interval of 0, otherwise why are you even making a new repeating task?", counter).expect("writeln failed");

            // error = true
            return true;
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

        // add task to struct
        repeating_task_start.task = start.get(counter * 5 - 5).unwrap().to_string();

        // set interval and unit
        repeating_task_start.repeat_number =
            Some(start.get(counter * 5 - 4).unwrap().parse::<u32>().unwrap());
        repeating_task_start.repeat_unit = Some(start.get(counter * 5 - 3).unwrap().to_owned());

        // get the date, time, repeat_original_date, and repeat_original_time
        let (date, time, repeat_original_date, repeat_original_time) =
            add_to_given_starting_datetime(
                start.get(counter * 5 - 2).unwrap().to_string(),
                start.get(counter * 5 - 1).unwrap().to_string(),
                start.get(counter * 5 - 4).unwrap().parse::<u32>().unwrap(),
                start.get(counter * 5 - 3).unwrap().to_owned(),
            );

        // set the remaining fields
        repeating_task_start.date = Some(date);
        repeating_task_start.time = Some(time);
        repeating_task_start.repeat_original_date = Some(repeat_original_date);
        repeating_task_start.repeat_original_time = Some(repeat_original_time);

        // push new correct Task
        repeating_tasks.todo.push(repeating_task_start);

        counter += 1;
    }

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
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

pub fn repeating_tasks_add_end(add_end: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // chartodo repeating-addend task 3 days 2030-01-01 00:00 task2 4 months 2031-01-01 00:00 => len % 5

    // check if we have the right # of args
    if add_end.len() % 5 != 0 {
        writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific ending datetime. Proper example: chartodo rp-ae new-item 3 days 2099-01-01 00:00. Another: chartodo rp-ae new-item 3 days 2099-01-01 00:00 another-item 4 years. After the command rp-ae, there should be 5, 10, 15, etc. (i.e., divisible by 5) arguments.").expect("writeln failed");

        // error = true
        return true;
    }

    // check how many sets of arguments there are
    let mut counter: usize = 1;
    // loop thru the args and parse for correctness
    while counter <= add_end.len() / 5 {
        // chartodo repeating-addend task 3 days 2030-01-01 00:00 task2 4 months 2031-01-01 00:00 => len % 5

        // end_time: get counter * 5 - 1
        // end_date: get counter * 5 - 2
        // unit: get counter * 5 - 3
        // interval: get counter * 5 - 4
        // task: get counter * 5 - 5

        // check if the starting time is proper
        if NaiveTime::parse_from_str(add_end.get(counter * 5 - 1).unwrap().as_str(), "%H:%M")
            .is_err()
        {
            writeln!(writer, "ERROR: Your provided ending time, '{}', in argument set '{}', wasn't proper. Please provide a correct ending time in a 24-hour format, e.g., 23:04.", add_end.get(counter * 5 - 1).unwrap(), counter).expect("writeln failed");

            // error = true
            return true;
        }

        // check if starting date is proper
        if NaiveDate::parse_from_str(add_end.get(counter * 5 - 2).unwrap().as_str(), "%Y-%m-%d")
            .is_err()
        {
            writeln!(writer, "ERROR: Your provided ending date, '{}', in argument set '{}', wasn't proper. Please provide a correct ending date in a year-month-day format, e.g., 2024-05-12.", add_end.get(counter * 5 - 2).unwrap(), counter).expect("writeln failed");

            // error = true
            return true;
        }

        // check if unit time is proper
        match add_end.get(counter * 5 - 3).unwrap().as_str() {
            "minutes" | "minute" | "hours" | "hour" | "days" | "day" | "weeks" | "week"
            | "months" | "month" | "years" | "year" => (),
            _ => {
                writeln!(writer, "ERROR: Your provided time unit, '{}', in argument set '{}', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.", add_end.get(counter * 5 - 3).unwrap(), counter).expect("writeln failed");

                // error = true
                return true;
            }
        }

        // check if the interval is proper
        if add_end
            .get(counter * 5 - 4)
            .unwrap()
            .parse::<u32>()
            .is_err()
        {
            writeln!(writer, "ERROR: Your provided interval, '{}', in argument set '{}', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-ae gym 2 days 2000-01-01.", add_end.get(counter * 5 - 4).unwrap(), counter).expect("writeln failed");

            // error = true
            return true;
        }

        // check if interval is 0
        if add_end
            .get(counter * 5 - 4)
            .unwrap()
            .parse::<u32>()
            .unwrap()
            == 0
        {
            writeln!(writer, "ERROR: You provided an interval of 0 in argument set {}. You can't have an interval of 0, otherwise why are you even making a new repeating task?", counter).expect("writeln failed");

            // error = true
            return true;
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

        // add task to struct
        repeating_task_end.task = add_end.get(counter * 5 - 5).unwrap().to_string();

        // set interval and unit
        repeating_task_end.repeat_number = Some(
            add_end
                .get(counter * 5 - 4)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        );
        repeating_task_end.repeat_unit = Some(add_end.get(counter * 5 - 3).unwrap().to_owned());

        // get the date, time, repeat_original_date, and repeat_original_time
        let (date, time, repeat_original_date, repeat_original_time) =
            subract_from_given_ending_datetime(
                add_end.get(counter * 5 - 2).unwrap().to_string(),
                add_end.get(counter * 5 - 1).unwrap().to_string(),
                add_end
                    .get(counter * 5 - 4)
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
                add_end.get(counter * 5 - 3).unwrap().to_owned(),
            );

        // set the remaining fields
        repeating_task_end.date = Some(date);
        repeating_task_end.time = Some(time);
        repeating_task_end.repeat_original_date = Some(repeat_original_date);
        repeating_task_end.repeat_original_time = Some(repeat_original_time);

        // push new correct Task
        repeating_tasks.todo.push(repeating_task_end);

        counter += 1;
    }

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
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

pub fn repeating_tasks_done(mut done: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty. Try adding items to it first."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable positions
    for i in (0..done.len()).rev() {
        if done.get(i).unwrap().parse::<usize>().is_err()
        || done.get(i).unwrap().is_empty() // this will never trigger smh
        || done.get(i).unwrap().parse::<usize>().unwrap() == 0
        || done.get(i).unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len()
        {
            done.swap_remove(i);
        }
    }

    // no valid arguments
    if done.is_empty() {
        writeln!(writer, "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    done.sort();
    done.dedup();

    // check if the user basically specified the entire list
    if done.len() >= repeating_tasks.todo.len() && repeating_tasks.todo.len() > 5 {
        writeln!(
            writer,
            "WARNING: You've specified the entire repeating todo list that's relatively long. You should do chartodo repeating-doneall"
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // before pushing to done, change each repeat_done field in each specified todo to true
    done.iter().for_each(|position| {
        repeating_tasks
            .todo
            .get_mut(position.parse::<usize>().unwrap() - 1)
            .unwrap()
            .repeat_done = Some(true);
    });

    // change todos to dones one by one
    done.iter().rev().for_each(|position| {
        repeating_tasks.done.push(
            repeating_tasks
                .todo
                .get(position.parse::<usize>().unwrap() - 1)
                .unwrap()
                .to_owned(),
        );
        repeating_tasks
            .todo
            .remove(position.parse::<usize>().unwrap() - 1);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

pub fn repeating_tasks_reset_original_datetime_to_now(mut reset: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty. Try adding items to it first."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable positions
    for i in (0..reset.len()).rev() {
        if reset.get(i).unwrap().parse::<usize>().is_err()
        || reset.get(i).unwrap().is_empty() // this will never trigger smh
        || reset.get(i).unwrap().parse::<usize>().unwrap() == 0
        || reset.get(i).unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len()
        {
            reset.swap_remove(i);
        }
    }

    // no valid args
    if reset.is_empty() {
        writeln!(writer, "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    reset.sort();
    reset.dedup();

    // check if the user basically specified the entire list
    if reset.len() >= repeating_tasks.todo.len() && repeating_tasks.todo.len() > 5 {
        writeln!(
            writer,
            "ERROR: You've specified the entire repeating todo list that's relatively long. You should do chartodo repeating-resetall"
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // reset each original datetime to local::now and make new due datetimes
    reset.iter().for_each(|position| {
        // create new original + due datetimes
        let (date, time, repeat_original_date, repeat_original_time) = add_to_local_now(
            repeating_tasks
                .todo
                .get(position.parse::<usize>().unwrap() - 1)
                .unwrap()
                .repeat_number
                .unwrap(),
            repeating_tasks
                .todo
                .get(position.parse::<usize>().unwrap() - 1)
                .unwrap()
                .repeat_unit
                .as_ref()
                .unwrap()
                .to_string(),
        );

        // set the new datetimes
        repeating_tasks
            .todo
            .get_mut(position.parse::<usize>().unwrap() - 1)
            .unwrap()
            .repeat_original_date = Some(repeat_original_date);
        repeating_tasks
            .todo
            .get_mut(position.parse::<usize>().unwrap() - 1)
            .unwrap()
            .repeat_original_time = Some(repeat_original_time);
        repeating_tasks
            .todo
            .get_mut(position.parse::<usize>().unwrap() - 1)
            .unwrap()
            .date = Some(date);
        repeating_tasks
            .todo
            .get_mut(position.parse::<usize>().unwrap() - 1)
            .unwrap()
            .time = Some(time);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

pub fn repeating_tasks_rmtodo(mut rmtodo: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty. Try adding items to it first."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable positions
    for i in (0..rmtodo.len()).rev() {
        if rmtodo.get(i).unwrap().parse::<usize>().is_err()
        || rmtodo.get(i).unwrap().is_empty() // this will never trigger smh
        || rmtodo.get(i).unwrap().parse::<usize>().unwrap() == 0
        || rmtodo.get(i).unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len()
        {
            rmtodo.swap_remove(i);
        }
    }

    // no valid args
    if rmtodo.is_empty() {
        writeln!(writer, "ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.").expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    rmtodo.sort();
    rmtodo.dedup();

    // check if user wants to remove all of the items
    if rmtodo.len() >= repeating_tasks.todo.len() && repeating_tasks.todo.len() > 5 {
        writeln!(
            writer,
            "WARNING: You've specified the entire repeating todo list, one that's relatively long. You should do repeating-cleartodo"
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // remove each item one by one
    rmtodo.iter().rev().for_each(|position| {
        repeating_tasks
            .todo
            .remove(position.parse::<usize>().unwrap() - 1);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

pub fn repeating_tasks_doneall() -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so you can't change any todos to done."
        )
        .expect("writeln failed");

        // error = treu
        return true;
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

    // error = false
    false
}

pub fn repeating_tasks_clear_todo() -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(writer, "ERROR: The repeating todo list is currently empty. Try adding items to it first before removing any.").expect("writeln failed");

        // error = true
        return true;
    }

    // clear todo list
    repeating_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

pub fn repeating_tasks_show_start(mut start: Vec<String>) -> String {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();

    // open file and parse
    let repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return String::from(
            "ERROR: the repeating todo list is currently empty. try adding items to it first.",
        );
    }

    // filter for viable positions
    for i in (0..start.len()).rev() {
        if start.get(i).unwrap().parse::<usize>().is_err()
        || start.get(i).unwrap().is_empty() // this will never trigger smh
        || start.get(i).unwrap().parse::<usize>().unwrap() == 0
        || start.get(i).unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len()
        {
            start.swap_remove(i);
        }
    }

    // no valid args
    if start.is_empty() {
        return String::from("ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length.");
    }

    // sort and dedup
    start.sort();
    start.dedup();

    // check if user wants to show starts for all of the items
    if start.len() >= repeating_tasks.todo.len() && repeating_tasks.todo.len() > 5 {
        return String::from("ERROR: you might as well do repeating-startall since you want to show the starting datetimes for all of the repeating tasks.");
    }

    let mut show_starts = String::from("");
    start.iter().for_each(|position| {
        let task_and_start = format!(
            "task: {}\n\tstart: {} {}\n",
            repeating_tasks
                .todo
                .get(position.parse::<usize>().unwrap() - 1)
                .unwrap()
                .task,
            repeating_tasks
                .todo
                .get(position.parse::<usize>().unwrap() - 1)
                .unwrap()
                .repeat_original_date
                .as_ref()
                .unwrap(),
            repeating_tasks
                .todo
                .get(position.parse::<usize>().unwrap() - 1)
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

pub fn repeating_tasks_resetall() -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty. Try adding items to it first."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // reset each original datetime to local::now and make new due datetimes
    repeating_tasks.todo.iter_mut().for_each(|task| {
        // create new original + due datetimes
        let (date, time, repeat_original_date, repeat_original_time) = add_to_local_now(
            task.repeat_number.unwrap(),
            task.repeat_unit.as_ref().unwrap().to_string(),
        );

        // set the new datetimes
        task.repeat_original_date = Some(repeat_original_date);
        task.repeat_original_time = Some(repeat_original_time);
        task.date = Some(date);
        task.time = Some(time);
    });

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

pub fn repeating_tasks_showstartall() -> String {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();

    // open file and parse
    let repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        return String::from(
            "ERROR: the repeating todo list is currently empty. try adding items to it first.",
        );
    }

    let mut show_starts = String::from("");
    repeating_tasks.todo.iter().for_each(|task| {
        let task_and_start = format!(
            "task: {}\n\tstart: {} {}\n",
            task.task,
            task.repeat_original_date.as_ref().unwrap(),
            task.repeat_original_time.as_ref().unwrap()
        );
        show_starts.push_str(task_and_start.as_str());
    });
    let show_starts = show_starts.trim_end();

    show_starts.to_string()
}

pub fn repeating_tasks_edit_all(edit_all: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // chartodo rp-ea 1 task 3 days start/end 2000-01-01 00:00. note that the user has to specify if the datetime is the start or end

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_all.len() != 7 {
        writeln!(writer, "ERROR: You must specify the repeating todo's position and all the parameters that will be edited. A proper example would be: chartodo rp-ea 4 new-item 3 days end 2150-01-01 00:00. If you wanted to edit the starting date instead, replace 'end' with 'start', e.g., chartodo rp-ea 4 new-item 3 days start 2150-01-01 00:00").expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if edit_all.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: The position you provided, '{}', was invalid. Try something between 1 and {}.",
            edit_all.first().unwrap(),
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if edit_all.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if edit_all.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: The position you provided, '{}', exceeds the repeating todo list's length. Try something between 1 and {}.",
            edit_all.first().unwrap(), repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // interval isn't proper
    if edit_all.get(2).unwrap().parse::<u32>().is_err() {
        writeln!(
            writer,
            "ERROR: The interval you provided, '{}', wasn't proper. It must be in the range of 1 - 4294967295 (i.e., it has to be u32).", edit_all.get(2).unwrap()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // check if interval is 0
    if edit_all.get(2).unwrap().parse::<u32>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Your interval can't be 0, otherwise why are you even setting a repeating task?"
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // unit of time isn't proper
    match edit_all.get(3).unwrap().as_str() {
        "minute" | "minutes" | "hour" | "hours" | "day" | "days" | "week" | "weeks" | "month"
        | "months" | "year" | "years" => (),
        _ => {
            writeln!(writer, "ERROR: The time unit you provided, '{}', wasn't proper. Proper examples: minutes, hours, days, weeks, months or years.", edit_all.get(3).unwrap()).expect("writeln failed");

            // error = treu
            return true;
        }
    }

    // date isn't proper
    if NaiveDate::parse_from_str(edit_all.get(5).unwrap().as_str(), "%Y-%m-%d").is_err() {
        writeln!(writer, "ERROR: The date you provided, '{}', wasn't proper. It must be in the following format: Year-Month-Day, e.g., 2000-01-01.", edit_all.get(5).unwrap()).expect("writeln failed");

        // error = true
        return true;
    }

    // time isn't proper
    if NaiveTime::parse_from_str(edit_all.last().unwrap().as_str(), "%H:%M").is_err() {
        writeln!(writer, "ERROR: The time you provided, '{}', wasn't proper. It must be in the following 24-hour format: H:M, e.g., 13:08.", edit_all.last().unwrap()).expect("writeln failed");

        // error = true
        return true;
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
            writeln!(writer, "ERROR: you must specify whether the given datetime is the starting or ending datetime. Please use the 'start' or 'end' keywords and nothing else.").expect("writeln failed");

            // error = treu
            return true;
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

    // error = false
    false
}

pub fn repeating_tasks_edit_task(edit_task: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // chartodo rp-eta 1 new-task

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_task.len() != 2 {
        writeln!(writer, "ERROR: You must specify the repeating todo's position and all the new task to change it to. A proper example would be: chartodo rp-eta 4 new-item.").expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if edit_task.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: The position you provided, '{}', was invalid. Try something between 1 and {}.",
            edit_task.first().unwrap(),
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if edit_task.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if edit_task.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceed's the repeating todo list's length. Try something between 1 and {}.",
            edit_task.first().unwrap(), repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // get the todo and edit task
    let position: usize = edit_task.first().unwrap().parse().unwrap();
    repeating_tasks.todo.get_mut(position - 1).unwrap().task =
        edit_task.last().unwrap().to_string();

    // write changes to file
    write_changes_to_new_repeating_tasks(repeating_tasks);

    // error = false
    false
}

pub fn repeating_tasks_edit_interval(edit_interval: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // chartodo rp-ei 1 3

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_interval.len() != 2 {
        writeln!(writer, "ERROR: You must specify the repeating todo's position and all the parameters that will be edited. A proper example would be: chartodo rp-ea 4 new-item 3 days end 2150-01-01 00:00. If you wanted to edit the starting date instead, replace 'end' with 'start', e.g., chartodo rp-ea 4 new-item 3 days start 2150-01-01 00:00").expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if edit_interval.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: The position you provided, '{}', was invalid. Try something between 1 and {}.",
            edit_interval.first().unwrap(),
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if edit_interval.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if edit_interval.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceeds the repeating todo list's length. Try something between 1 and {}.",
            edit_interval.first().unwrap(), repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // interval isn't proper
    if edit_interval.last().unwrap().parse::<u32>().is_err() {
        writeln!(
            writer,
            "ERROR: The interval you provided, '{}', wasn't proper. It must be in the (inclusive) range of 1 - 4294967295 (i.e., it has to be u32).", edit_interval.last().unwrap()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // check if interval is 0
    if edit_interval.last().unwrap().parse::<u32>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Your interval can't be 0, otherwise why are you even setting a repeating task?"
        )
        .expect("writeln failed");

        // error = true
        return true;
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

    // error = false
    false
}

pub fn repeating_tasks_edit_time_unit(edit_unit: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // chartodo rp-eu 1 weeks

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_unit.len() != 2 {
        writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the interval time unit to. A proper example would be: chartodo rp-ea 4 weeks. That would change repeating task #4's time unit to 'weeks'.").expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if edit_unit.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: The position you provided, '{}', was invalid. Try something between 1 and {}.",
            edit_unit.first().unwrap(),
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if edit_unit.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if edit_unit.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceeds the repeating todo list's length. Try something between 1 and {}.",
            edit_unit.first().unwrap(), repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // unit of time isn't proper
    match edit_unit.last().unwrap().as_str() {
        "minute" | "minutes" | "hour" | "hours" | "day" | "days" | "week" | "weeks" | "month"
        | "months" | "year" | "years" => (),
        _ => {
            writeln!(writer, "ERROR: didn't provide a proper time unit for the interval. Proper examples: minutes, hours, days, weeks, months or years.").expect("writeln failed");

            // error = true
            return true;
        }
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

    // error = false
    false
}

pub fn repeating_tasks_edit_interval_unit(edit_interval_unit: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // chartodo rp-eiu 1 3 days

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_interval_unit.len() != 3 {
        writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the interval and time unit to. A proper example would be: chartodo rp-ea 4 3 days.").expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if edit_interval_unit
        .first()
        .unwrap()
        .parse::<usize>()
        .is_err()
    {
        writeln!(
            writer,
            "ERROR: The position you provided, '{}', wasn't valid. Try something between 1 and {}.",
            edit_interval_unit.first().unwrap(),
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if edit_interval_unit
        .first()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        == 0
    {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if edit_interval_unit
        .first()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        > repeating_tasks.todo.len()
    {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceeds the repeating todo list's length. Try something between 1 and {}.",
            edit_interval_unit.first().unwrap(), repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // interval isn't proper
    if edit_interval_unit.get(1).unwrap().parse::<u32>().is_err() {
        writeln!(
            writer,
            "ERROR: The interval you provided, '{}', isn't proper. It must be in the (inclusive) range of 1 - 4294967295, (i.e., it has to be u32).", edit_interval_unit.get(1).unwrap()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // check if interval is 0
    if edit_interval_unit.get(1).unwrap().parse::<u32>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Your interval can't be 0, otherwise why are you even setting a repeating task?"
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // unit of time isn't proper
    match edit_interval_unit.last().unwrap().as_str() {
        "minute" | "minutes" | "hour" | "hours" | "day" | "days" | "week" | "weeks" | "month"
        | "months" | "year" | "years" => (),
        _ => {
            writeln!(writer, "ERROR: didn't provide a proper time unit for the interval. Proper examples: minutes, hours, days, weeks, months or years.").expect("writeln failed");

            // error = treu
            return true;
        }
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

    // error = false
    false
}

pub fn repeating_tasks_edit_start(edit_start: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // chartodo rp-es 1 2100-01-01 00:00

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_start.len() != 3 {
        writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the repeating task's starting datetime to. A proper example would be: chartodo rp-es 4 2100-12-24 13:08").expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if edit_start.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: The position you provided, '{}', was invalid. Try something between 1 and {}.",
            edit_start.first().unwrap(),
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if edit_start.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if edit_start.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceeds the repeating todo list's length. Try something between 1 and {}.",
            edit_start.first().unwrap(), repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // date isn't proper
    if NaiveDate::parse_from_str(edit_start.get(1).unwrap().as_str(), "%Y-%m-%d").is_err() {
        writeln!(writer, "ERROR: The date you provided, '{}', wasn't proper. It must be in the following format: Year-Month-Day, e.g., 2000-01-01.", edit_start.get(1).unwrap()).expect("writeln failed");

        // error = true
        return true;
    }

    // time isn't proper
    if NaiveTime::parse_from_str(edit_start.last().unwrap().as_str(), "%H:%M").is_err() {
        writeln!(writer, "ERROR: The time you provided, '{}', wasn't proper. It must be in the following 24-hour format: H:M, e.g., 13:08.", edit_start.last().unwrap()).expect("writeln failed");

        // error = true
        return true;
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

    // error = false
    false
}

pub fn repeating_tasks_edit_end(edit_end: Vec<String>) -> bool {
    // housekeeping
    repeating_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut repeating_tasks = open_repeating_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if repeating_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The repeating todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // chartodo rp-ee 1 2100-12-24 13:08

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_end.len() != 3 {
        writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the repeating task's ending datetime to. A proper example would be: chartodo rp-ee 4 2100-12-14 13:08").expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if edit_end.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: The position you provided, '{}', wasn't valid. Try something between 1 and {}.",
            edit_end.first().unwrap(),
            repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if edit_end.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = treu
        return true;
    }

    // position not in range of todo list len
    if edit_end.first().unwrap().parse::<usize>().unwrap() > repeating_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceeds the repeating todo list's length. Try something between 1 and {}.",
            edit_end.first().unwrap(), repeating_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = treu
        return true;
    }

    // date isn't proper
    if NaiveDate::parse_from_str(edit_end.get(1).unwrap().as_str(), "%Y-%m-%d").is_err() {
        writeln!(writer, "ERROR: The date you provided, '{}', wasn't proper. It must be in the following format: Year-Month-Day, e.g., 2000-01-01.", edit_end.get(1).unwrap()).expect("writeln failed");

        // error = treu
        return true;
    }

    // time isn't proper
    if NaiveTime::parse_from_str(edit_end.last().unwrap().as_str(), "%H:%M").is_err() {
        writeln!(writer, "ERROR: The time you provided, '{}', wasn't proper. It must be in the following 24-hour format: H:M, e.g., 13:08.", edit_end.last().unwrap()).expect("writeln failed");

        // error = true
        return true;
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

    // error = false
    false
}
