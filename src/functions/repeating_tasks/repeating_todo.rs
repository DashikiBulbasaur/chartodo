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
        writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a repeating task.\n\tThere should be 3, 6, 9, etc. (i.e., divisible by 3) arguments after 'chartodo repeating-add'. You provided {} argument(s).\n\tFormat: chartodo repeating-add ~task ~interval ~time-unit [...].\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\tExample: chartodo rp-a do-a-backflip 2 days.\n\tAnother example: chartodo rp-a new-item 3 days another-item 4 years", add.len()).expect("writeln failed");

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
                writeln!(writer, "ERROR: Your provided time unit, '{}', in argument set '{}', wasn't proper. It has to be one of the following: minutes, hours, days, weeks, months, years.", add.get(counter * 3 - 1).unwrap(), counter).expect("writeln failed");

                // error = true
                return true;
            }
        }

        // does this u32 thing even make sense? it's just a micro-optimization, not making it usize, so it makes the program slightly faster
        // ah well. usize has a max too. u32 is fine and big enough (pause)
        // check if the interval is proper. has to be u32
        if add.get(counter * 3 - 2).unwrap().parse::<u32>().is_err() {
            writeln!(writer, "ERROR: Your provided interval, '{}', in argument set '{}', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-a gym 2 days.", add.get(counter * 3 - 2).unwrap(), counter).expect("writeln failed");

            // error = true
            return true;
        }

        // check if interval is 0
        if add.get(counter * 3 - 2).unwrap().parse::<u32>().unwrap() == 0 {
            writeln!(writer, "ERROR: You had an interval of 0 in argument set '{}'. You can't have an interval of 0, otherwise why are you even making a new repeating task?", counter).expect("writeln failed");

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
        writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific starting datetime.\n\tThere should be 5, 10, 15, etc. (i.e., divisible by 5) arguments after 'chartodo repeating-addstart'. You provided {} argument(s).\n\tFormat: chartodo repeating-addstart ~task ~interval ~time-unit ~date ~time [...].\n\t\tDate should be in a yy-mm-dd format. Time should be in a 24-hour format.\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\tExample: chartodo rp-as new-item 3 days 2099-01-01 00:00.\n\tAnother example: chartodo rp-as new-item 3 days 2099-01-01 00:00 another-item 4 years 23:59", start.len()).expect("writeln failed");

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
        if NaiveDate::parse_from_str(start.get(counter * 5 - 2).unwrap().as_str(), "%Y-%m-%d")
            .is_err()
        {
            writeln!(writer, "ERROR: Your provided starting date, '{}', in argument set '{}', wasn't proper. Please provide a correct starting date in a year-month-day format, e.g., 2024-05-13.", start.get(counter * 5 - 2).unwrap(), counter).expect("writeln failed");

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
        writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a repeating task with a specific ending datetime.\n\tThere should be 5, 10, 15, etc. (i.e., divisible by 5) arguments after 'chartodo repeating-addend'. You provided {} argument(s).\n\tFormat: chartodo repeating-addend ~task ~interval ~time-unit ~date ~time [...].\n\t\tDate must be in a yy-mm-format. Time must be in a 24-hour format.\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\tExample: chartodo rp-ae new-item 3 days 2099-01-01 00:00.\n\tAnother example: chartodo rp-ae new-item 3 days 2099-01-01 00:00 another-item 4 years 23:59", add_end.len()).expect("writeln failed");

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
            writeln!(writer, "ERROR: Your provided interval, '{}', in argument set '{}', wasn't proper. It can't be negative and can't be above 4294967295 (i.e., it has to be u32). Proper example: chartodo rp-ae gym 2 days 2000-01-01 00:00", add_end.get(counter * 5 - 4).unwrap(), counter).expect("writeln failed");

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
    // get an ending datetime to subtract from
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
    let mut done: Vec<usize> = done.iter().map(|x| x.parse::<usize>().unwrap()).collect();
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
            .get_mut(position - 1)
            .unwrap()
            .repeat_done = Some(true);
    });

    // change todos to dones one by one
    done.iter().rev().for_each(|position| {
        repeating_tasks
            .done
            .push(repeating_tasks.todo.get(position - 1).unwrap().to_owned());
        repeating_tasks.todo.remove(position - 1);
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
    let mut rmtodo: Vec<usize> = rmtodo.iter().map(|x| x.parse::<usize>().unwrap()).collect();
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
        repeating_tasks.todo.remove(position - 1);
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
    let mut start: Vec<usize> = start.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    start.sort();
    start.dedup();

    // check if user wants to show starts for all of the items
    if start.len() >= repeating_tasks.todo.len() && repeating_tasks.todo.len() > 5 {
        return String::from("WARNING: You want to show the start times for an entire list that's relatively long, You should do repeating-startall.");
    }

    let mut show_starts = String::from("");
    start.iter().for_each(|position| {
        let task_and_start = format!(
            "task: {}\n\tstart: {} {}\n",
            repeating_tasks.todo.get(position - 1).unwrap().task,
            repeating_tasks
                .todo
                .get(position - 1)
                .unwrap()
                .repeat_original_date
                .as_ref()
                .unwrap(),
            repeating_tasks
                .todo
                .get(position - 1)
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
        writeln!(writer, "ERROR: You must specify the repeating todo's position and all the parameters that will be edited.\n\tThere should be 7 arguments after 'chartodo repeating-editall'. You provided {} argument(s).\n\tExample: chartodo repeating-editall ~position ~task ~interval ~time-unit ~start/end ~date ~time.\n\t\tDate must be in a yy-mm-dd format. Time must be in a 24-hour format.\n\t\tOnly the following time-units are allowed: minute(s), hour(s), day(s), week(s), month(s), and year(s).\n\t\tYou must specify if you're editing the ending or starting datetime by using the keywords 'start' or 'end'.\n\tExample (with end): chartodo rp-ea 4 new-item 3 days end 2150-01-01 00:00.\n\tExample (with start): chartodo rp-ea 4 new-item 3 days start 2150-01-01 00:00", edit_all.len()).expect("writeln failed");

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
        writeln!(writer, "ERROR: You must specify the repeating todo's position and the new task to change it to.\n\tThere should be 2 arguments after 'chartodo repeating-edittask'. You provided {} argument(s).\n\tFormat: chartodo repeating-edittask ~position ~task.\n\tExample: chartodo rp-eta 4 new-item.", edit_task.len()).expect("writeln failed");

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
        writeln!(writer, "ERROR: You must specify the repeating todo's position and what to edit the interval to.\n\tThere should be 2 arguments after 'chartodo repeating-editinterval'. You provided {} argument(s).\n\tFormat: chartodo repeating-editinterval ~position ~interval.\n\tExample: chartodo rp-ei 4 3.\n\t\t'4' would be the todo task's position and '3' would be the new interval, i.e., repeating task 4 would now have an interval of '3 days'.", edit_interval.len()).expect("writeln failed");

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
        writeln!(writer, "ERROR: You must specify the repeating todo's position and what to edit the time unit to.\n\tThere should be 2 arguments after 'chartodo repeating-eu'. You provided {} arguments().\n\tFormat: chartodo repeating-editunit ~position ~time-unit.\n\tExample: chartodo rp-eu 4 weeks.\n\t\tThat would change repeating task #4's time unit to 'weeks'.", edit_unit.len()).expect("writeln failed");

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
            writeln!(writer, "ERROR: The time unit you provided, '{}', wasn't proper. Proper examples: minutes, hours, days, weeks, months or years.", edit_unit.last().unwrap()).expect("writeln failed");

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
        writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the interval and time unit to.\n\tThere should be 3 arguments after 'chartodo repeating-editintervalunit'. You provided {} argument(s).\n\tFormat: chartodo repeating-editintervalunit ~position ~interval ~time-unit.\n\tExample: chartodo rp-ea 4 3 days.", edit_interval_unit.len()).expect("writeln failed");

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
            writeln!(writer, "ERROR: The time unit you provided, '{}', wasn't proper. Proper examples: minutes, hours, days, weeks, months or years.", edit_interval_unit.last().unwrap()).expect("writeln failed");

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
        writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the repeating task's starting datetime to.\n\tThere should be 3 arguments after 'chartodo repeating-editstart'. You provided {} argument(s).\n\tFormat: chartodo repeating-editstart ~position ~date ~time.\n\t\tDate should be in a yy-mm-dd format. Time should be in a 24-hour format.\n\tExample: chartodo rp-es 4 2100-12-24 13:08", edit_start.len()).expect("writeln failed");

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
        writeln!(writer, "ERROR: You must specify the repeating todo's position and what to change the repeating task's ending datetime to.\n\tThere should be 3 arguments after 'chartodo repeating-editend'. You provided {} argument(s).\n\tFormat: chartodo repeating-editend ~position ~date ~time.\n\t\tDate should be in a yy-mm-dd format. Time should be in a 24-hour format.\n\tExample: chartodo rp-ee 4 2100-12-14 13:08", edit_end.len()).expect("writeln failed");

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

// note that it's starting to dawn on me that this style of design + testing is too restrictive and not flexible enough
// however, since this is an open source software that I want other people to use, I need it to be robust and reliable
// and for it to be both those things, i need to test it, even if the method is stupid (at least for now)

// note that I want it to be more flexible in the future

// cargo test repeating_todo_unit_tests -- --test-threads=1
#[cfg(test)]
mod repeating_todo_unit_tests {
    use super::*;
    use anyhow::Context;
    use std::path::PathBuf;

    // these are taken from repeating_helpers
    fn path_to_repeating_tasks() -> PathBuf {
        // get the data dir XDG spec and return it with path to repeating_tasks.json
        let mut repeating_tasks_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        repeating_tasks_path.push("chartodo/repeating_tasks.json");

        repeating_tasks_path
    }

    fn repeating_tasks_copy_path() -> PathBuf {
        // get the path for repeating_tasks_copy.json, which will be used to hold the original contents
        // of repeating_tasks.json while it's getting modified
        let mut repeating_tasks_copy_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        repeating_tasks_copy_path.push("chartodo/repeating_tasks_copy.json");

        repeating_tasks_copy_path
    }

    // these have been tested in other fns, these are just included here as a sanity check
    #[test]
    fn repeating_tasks_path_is_correct() {
        let linux_path = "/.local/share/chartodo/repeating_tasks.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/repeating_tasks.json";
        let mac_path = "/Library/Application Support/chartodo/repeating_tasks.json";
        let mut got_repeating_tasks_path: bool = false;
        let repeating_path = path_to_repeating_tasks();
        let repeating_path = repeating_path.to_str().unwrap();

        if repeating_path.contains(linux_path) {
            got_repeating_tasks_path = true;
        } else if repeating_path.contains(windows_path) {
            got_repeating_tasks_path = true;
        } else if repeating_path.contains(mac_path) {
            got_repeating_tasks_path = true;
        }

        assert!(got_repeating_tasks_path);
    }

    #[test]
    fn repeating_tasks_copy_path_is_correct() {
        let linux_path = "/.local/share/chartodo/repeating_tasks_copy.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/repeating_tasks_copy.json";
        let mac_path = "/Library/Application Support/chartodo/repeating_tasks_copy.json";
        let mut got_repeating_tasks_copy_path: bool = false;
        let repeating_tasks_copy_path = repeating_tasks_copy_path();
        let repeating_tasks_copy_path = repeating_tasks_copy_path.to_str().unwrap();

        if repeating_tasks_copy_path.contains(linux_path) {
            got_repeating_tasks_copy_path = true;
        } else if repeating_tasks_copy_path.contains(windows_path) {
            got_repeating_tasks_copy_path = true;
        } else if repeating_tasks_copy_path.contains(mac_path) {
            got_repeating_tasks_copy_path = true;
        }

        assert!(got_repeating_tasks_copy_path);
    }

    #[test]
    fn aaaa_repeating_tasks_clone_file() {
        // name is aaaa so it's done first
        // since we will be modifying the original file to run a test, the original data must be
        // preserved first
        std::fs::File::create(repeating_tasks_copy_path())
            .context("failed to create repeating_tasks_copy.json")
            .expect("failed to create a copy during unit test");

        std::fs::copy(path_to_repeating_tasks(), repeating_tasks_copy_path())
            .context("failed to copy repeating_tasks.json to repeating_tasks_copy.json")
            .expect("failed to copy original file to copy file during unit test");
    }

    #[test]
    fn repeating_tasks_add_incorrect_num_of_args() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("another"),
            String::from("2"),
        ];
        let error_should_be_true = repeating_tasks_add(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_time_unit_invalid() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("another"),
            String::from("2"),
            String::from("seconds"),
        ];
        let error_should_be_true = repeating_tasks_add(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_interval_not_u32() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("another"),
            String::from("4294967296"), // one more than max of u32, 4294967295
            String::from("days"),
        ];
        let error_should_be_true = repeating_tasks_add(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_interval_is_zero() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("another"),
            String::from("0"),
            String::from("days"),
        ];
        let error_should_be_true = repeating_tasks_add(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
        ];
        let error_should_be_false = repeating_tasks_add(arguments);
        // impossible to test the contents since the result of the fn is dependent on the current day and time
        // i can however test the content results of rp-as and rp-ae

        assert!(!error_should_be_false);
    }

    #[test]
    fn repeating_tasks_add_multiple_args_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("hello"),
            String::from("21"),
            String::from("years"),
        ];
        let error_should_be_false = repeating_tasks_add(arguments);
        // impossible to test the contents since the result of the fn is dependent on the current day and time
        // i can however test the content results of rp-as and rp-ae

        assert!(!error_should_be_false);
    }

    // note that it's impossible to test add_to_local_now since that fn's result is dependent on the current date and time

    #[test]
    fn repeating_tasks_add_start_datetime_incorrect_num_of_args() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("2021-01-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("days"),
            String::from("2022-12-24"),
        ];
        let error_should_be_true = repeating_tasks_add_start_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_start_datetime_time_invalid() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("2021-01-01"),
            String::from("25:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("days"),
            String::from("2022-12-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_start_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_start_datetime_date_invalid() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("2021-13-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("days"),
            String::from("2022-12-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_start_datetime(arguments);

        assert!(error_should_be_true);
    }

    // note that I've thought about ignoring invalid args and just not adding them to the file and only adding the valid ones.
    // would maybe be more convenient to the user, or maybe not

    #[test]
    fn repeating_tasks_add_start_datetime_time_unit_invalid() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("second"),
            String::from("2021-01-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("days"),
            String::from("2022-12-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_start_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_start_datetime_interval_not_u32() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("4294967296"),
            String::from("minutes"),
            String::from("2021-01-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("days"),
            String::from("2022-12-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_start_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_start_datetime_interval_is_zero() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("0"),
            String::from("minutes"),
            String::from("2021-01-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("days"),
            String::from("2022-12-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_start_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_start_datetime_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("3"),
            String::from("minutes"),
            String::from("2021-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_false = repeating_tasks_add_start_datetime(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:03",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2021-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_add_start_datetime_multiple_args_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("3"),
            String::from("weeks"),
            String::from("2021-01-01"),
            String::from("00:00"),
            String::from("hi"),
            String::from("276"),
            String::from("minutes"),
            String::from("2099-12-05"),
            String::from("13:26"),
        ];
        let error_should_be_false = repeating_tasks_add_start_datetime(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-22",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2021-01-01",
                        "repeat_original_time": "00:00"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-05",
                        "time": "18:02",
                        "repeat_number": 276,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2099-12-05",
                        "repeat_original_time": "13:26"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn add_to_given_starting_datetime_is_correct() {
        let (date, time, repeat_original_date, repeat_original_time) =
            add_to_given_starting_datetime(
                "2021-01-28".to_string(),
                "13:13".to_string(),
                5,
                "months".to_string(),
            );

        assert_eq!(date, "2021-06-28".to_string());
        assert_eq!(time, "13:13".to_string());
        assert_eq!(repeat_original_date, "2021-01-28".to_string());
        assert_eq!(repeat_original_time, "13:13".to_string());
    }

    #[test]
    fn repeating_tasks_add_end_incorrect_num_of_args() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("2021-01-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("days"),
            String::from("2022-12-24"),
        ];
        let error_should_be_true = repeating_tasks_add_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_end_time_invalid() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("2099-01-01"),
            String::from("25:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("days"),
            String::from("2030-12-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_end_date_invalid() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("2099-01-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("days"),
            String::from("2030-13-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_end_time_unit_invalid() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("2099-01-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("2"),
            String::from("decades"),
            String::from("2030-12-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_end_interval_not_u32() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("2099-01-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("a"),
            String::from("years"),
            String::from("2030-12-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_end_interval_is_zero() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("1"),
            String::from("day"),
            String::from("2099-01-01"),
            String::from("00:00"),
            String::from("this-is-the-todo-list"),
            String::from("0"),
            String::from("years"),
            String::from("2030-12-24"),
            String::from("13:26"),
        ];
        let error_should_be_true = repeating_tasks_add_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_add_end_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("3"),
            String::from("minutes"),
            String::from("2021-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_false = repeating_tasks_add_end(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_add_end_multiple_args_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("3"),
            String::from("minutes"),
            String::from("2021-01-01"),
            String::from("00:00"),
            String::from("yello"),
            String::from("50"),
            String::from("years"),
            String::from("2223-01-03"),
            String::from("13:13"),
        ];
        let error_should_be_false = repeating_tasks_add_end(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "yello",
                        "date": "2223-01-03",
                        "time": "13:13",
                        "repeat_number": 50,
                        "repeat_unit": "years",
                        "repeat_done": false,
                        "repeat_original_date": "2173-01-03",
                        "repeat_original_time": "13:13"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn subtract_from_given_ending_datetime_is_correct() {
        let (date, time, repeat_original_date, repeat_original_time) =
            subract_from_given_ending_datetime(
                "2013-12-13".to_string(),
                "00:00".to_string(),
                100,
                "months".to_string(),
            );

        assert_eq!(date, "2013-12-13".to_string());
        assert_eq!(time, "00:00".to_string());
        assert_eq!(repeat_original_date, "2005-08-13".to_string());
        assert_eq!(repeat_original_time, "00:00".to_string());
    }

    #[test]
    fn repeating_tasks_done_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that repeating todo list is correctly identified as empty
        let arguments = vec![String::from("a")];
        let error_should_be_true = repeating_tasks_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_done_no_valid_args() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("a"),
            String::from("0"),
            String::from("-1"),
            String::from("2"),
        ];
        let error_should_be_true = repeating_tasks_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_done_should_do_repeatingdoneall() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("1"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_should_be_true = repeating_tasks_done(arguments);

        assert!(error_should_be_true);
    }

    // notice that the responsibility of changing a done rp task back to todo if its date and/or time has been passed by the current datetime
    // falls on the function printing the list, not on rp-d which only moves tasks to the done list

    // the general flow is this:
    // 1. rp task is moved to done list by rp-d
    // 2. the function to print the list is called
    // 3. if an rp task's (in the done list) date and/or time has been passed by the current date and/or time,
    //  it is moved back to todo, and new date + time + repeat_original_date + repeat_original_time are calculated
    //  and repeat_done is changed back to false.
    //  if its date and/or time are still beyond the current date and/or time, nothing happens and it stays in the done list.
    // 4. note that this only happens once, and if the rp task's (in the todo list) new date and/or time are still behind the current date and/or time,
    //  it is not moved to the done list again, but instead is tagged with 'MISSED'
    // 5. the list is printed

    #[test]
    fn repeating_tasks_done_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![String::from("1"), String::from("2")];
        let error_should_be_false = repeating_tasks_done(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": true,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ]
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    // note that I had completely forgotten that before repeating.todo is written to the file, it is first sorted
    // this is done so that when the file is read and printed, it can check if it has to sort or not before printing.
    // this means that the order that the tasks are pushed to done is not necessarily the order that will come out once the file is read

    // note that the benefit of the list print checking if it has to sort and write to the file
    // (before it would just sort and write automatically) is that I think it speeds it up a little bit
    #[test]
    fn repeating_tasks_done_multiple_args_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("4"),
            String::from("1"),
        ];
        let error_should_be_false = repeating_tasks_done(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": true,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_reset_datetime_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that repeating todo list is correctly identified as empty
        let arguments = vec![String::from("a")];
        let error_should_be_true = repeating_tasks_reset_original_datetime_to_now(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_reset_datetime_no_valid_args() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("a"),
            String::from("0"),
            String::from("-1"),
            String::from("2"),
        ];
        let error_should_be_true = repeating_tasks_reset_original_datetime_to_now(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_reset_datetime_should_do_repeatingresetall() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("1"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_should_be_true = repeating_tasks_reset_original_datetime_to_now(arguments);

        assert!(error_should_be_true);
    }

    // impossible (at least I think) to test the contents of the file after repeating-reset
    // since its results are based on the current date and time
    // therefore, just like rp-a, i will just check for the bool

    #[test]
    fn repeating_tasks_reset_datetime_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![String::from("1"), String::from("2")];
        let error_should_be_false = repeating_tasks_reset_original_datetime_to_now(arguments);

        assert!(!error_should_be_false);
    }

    #[test]
    fn repeating_tasks_reset_datetime_multiple_args_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("4"),
            String::from("1"),
        ];
        let error_should_be_false = repeating_tasks_reset_original_datetime_to_now(arguments);

        assert!(!error_should_be_false);
    }

    #[test]
    fn repeating_tasks_rmtodo_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that repeating todo list is correctly identified as empty
        let arguments = vec![String::from("a")];
        let error_should_be_true = repeating_tasks_rmtodo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_rmtodo_no_valid_args() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("a"),
            String::from("0"),
            String::from("-1"),
            String::from("2"),
        ];
        let error_should_be_true = repeating_tasks_rmtodo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_rmtodo_should_do_repeatingcleartodo() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("1"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_should_be_true = repeating_tasks_rmtodo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_rmtodo_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![String::from("1"), String::from("3")];
        let error_should_be_false = repeating_tasks_rmtodo(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_rmtodo_multiple_args_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hello",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![String::from("1"), String::from("3"), String::from("-1")];
        let error_should_be_false = repeating_tasks_rmtodo(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_doneall_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that repeating todo list is correctly identified as empty
        let error_should_be_true = repeating_tasks_doneall();

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_doneall_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let error_should_be_false = repeating_tasks_doneall();
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": true,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_cleartodo_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that repeating todo list is correctly identified as empty
        let error_should_be_true = repeating_tasks_clear_todo();

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_cleartodo_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hello",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let error_should_be_false = repeating_tasks_clear_todo();
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_showstart_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that repeating todo list is correctly identified as empty
        let arguments = vec![String::from("a")];
        let error_msg = repeating_tasks_show_start(arguments);

        assert_eq!(
            error_msg,
            String::from(
                "ERROR: the repeating todo list is currently empty. try adding items to it first.",
            )
        );
    }

    #[test]
    fn repeating_tasks_showstart_no_valid_args() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("a"),
            String::from("0"),
            String::from("-1"),
            String::from("2"),
        ];
        let error_msg = repeating_tasks_show_start(arguments);

        assert_eq!(error_msg, String::from("ERROR: None of the positions you provided were viable -- they were all either negative, zero, or exceeded the repeating todo list's length."));
    }

    #[test]
    fn repeating_tasks_showstart_should_do_repeatingshowstartall() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("1"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_msg = repeating_tasks_show_start(arguments);

        assert_eq!(error_msg, String::from("WARNING: You want to show the start times for an entire list that's relatively long, You should do repeating-startall."));
    }

    #[test]
    fn repeating_tasks_showstart_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:56"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![String::from("1"), String::from("3")];
        let error_msg = repeating_tasks_show_start(arguments);

        assert_eq!(
            error_msg,
            String::from("task: this-is-the-todo-list\n\tstart: 2020-12-31 23:57")
        );
    }

    #[test]
    fn repeating_tasks_showstart_multiple_args_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:56"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let arguments = vec![String::from("1"), String::from("3"), String::from("2")];
        let error_msg = repeating_tasks_show_start(arguments);

        assert_eq!(
            error_msg,
            String::from("task: this-is-the-todo-list\n\tstart: 2020-12-31 23:57\ntask: hi\n\tstart: 2020-12-31 23:56")
        );
    }

    #[test]
    fn repeating_tasks_resetall_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that repeating todo list is correctly identified as empty
        let error_should_be_true = repeating_tasks_resetall();

        assert!(error_should_be_true);
    }

    // once again, i can't test for the content of the file since
    // the fn's result is dependent on the current date and time
    #[test]
    fn repeating_tasks_resetall_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "nyah",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    },
                    {
                        "task": "yellow",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": false,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ],
                "done": [
                    {
                        "task": "blue",
                        "date": "2099-12-12",
                        "time": "23:46",
                        "repeat_number": 15,
                        "repeat_unit": "weeks",
                        "repeat_done": true,
                        "repeat_original_date": "2098-09-12",
                        "repeat_original_time": "23:46"
                    }
                ]
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let error_should_be_false = repeating_tasks_resetall();

        assert!(!error_should_be_false);
    }

    #[test]
    fn repeating_tasks_showstartall_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // check that repeating todo list is correctly identified as empty
        let error_msg = repeating_tasks_showstartall();

        assert_eq!(
            error_msg,
            String::from(
                "ERROR: the repeating todo list is currently empty. try adding items to it first.",
            )
        );
    }

    #[test]
    fn repeating_tasks_showstartall_is_correct() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 3,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:57"
                    },
                    {
                        "task": "hi",
                        "date": "2021-01-01",
                        "time": "00:00",
                        "repeat_number": 4,
                        "repeat_unit": "minutes",
                        "repeat_done": false,
                        "repeat_original_date": "2020-12-31",
                        "repeat_original_time": "23:56"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // no valid args
        let error_msg = repeating_tasks_showstartall();

        assert_eq!(
            error_msg,
            String::from("task: this-is-the-todo-list\n\tstart: 2020-12-31 23:57\ntask: hi\n\tstart: 2020-12-31 23:56")
        );
    }

    #[test]
    fn repeating_tasks_editall_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_invalid_num_of_args() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("don't matter"),
            String::from("don't matter"),
            String::from("don't matter"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_position_not_a_num() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("a"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_position_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("0"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_position_isnt_in_range() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("2"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    // chartodo rp-ea 1 task 3 days start/end 2000-01-01 00:00. note that the user has to specify if the datetime is the start or end

    #[test]
    fn repeating_tasks_editall_interval_isnt_u32() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_interval_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hello"),
            String::from("0"),
            String::from("00:00"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_time_unit_invalid() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hello"),
            String::from("1"),
            String::from("00:00"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_must_be_start_or_end() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hello"),
            String::from("2"),
            String::from("years"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_date_invalid() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hello"),
            String::from("2"),
            String::from("years"),
            String::from("start"),
            String::from("2023-13-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_time_invalid() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hello"),
            String::from("2"),
            String::from("years"),
            String::from("start"),
            String::from("2023-1-01"),
            String::from("25:00"),
        ];
        let error_should_be_true = repeating_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editall_is_correct() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hello"),
            String::from("2"),
            String::from("years"),
            String::from("start"),
            String::from("2023-1-01"),
            String::from("00:00"),
        ];
        let error_should_be_false = repeating_tasks_edit_all(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "years",
                        "repeat_done": false,
                        "repeat_original_date": "2023-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_edittask_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_edittask_invalid_num_of_args() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("don't matter"),
            String::from("don't matter"),
            String::from("don't matter"),
        ];
        let error_should_be_true = repeating_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_edittask_position_not_a_num() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("a"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_edittask_position_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("0"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_edittask_position_isnt_in_range() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("2"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_edittask_is_correct() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("hi")];
        let error_should_be_false = repeating_tasks_edit_task(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_editinterval_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_interval(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editinterval_invalid_num_of_args() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("don't matter"),
            String::from("don't matter"),
            String::from("don't matter"),
        ];
        let error_should_be_true = repeating_tasks_edit_interval(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editinterval_position_not_a_num() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("a"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_interval(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editinterval_position_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("0"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_interval(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editinterval_position_isnt_in_range() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("2"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_interval(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editinterval_interval_isnt_u32() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_interval(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editinterval_interval_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("0")];
        let error_should_be_true = repeating_tasks_edit_interval(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editinterval_is_correct() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("2")];
        let error_should_be_false = repeating_tasks_edit_interval(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2026-01-01",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_editunit_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_time_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editunit_invalid_num_of_args() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("don't matter"),
            String::from("don't matter"),
            String::from("don't matter"),
        ];
        let error_should_be_true = repeating_tasks_edit_time_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editunit_position_not_a_num() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("a"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_time_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editunit_position_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("0"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_time_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editunit_position_isnt_in_range() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("2"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_time_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editunit_time_unit_invalid() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("hello")];
        let error_should_be_true = repeating_tasks_edit_time_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editunit_is_correct() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("month")];
        let error_should_be_false = repeating_tasks_edit_time_unit(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2024-02-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "month",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_editintervalunit_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_interval_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editintervalunit_invalid_num_of_args() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter"), String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_interval_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editintervalunit_position_not_a_num() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("a"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_interval_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editintervalunit_position_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("0"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_interval_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editintervalunit_position_isnt_in_range() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("2"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_interval_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editintervalunit_interval_isnt_u32() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_interval_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editintervalunit_interval_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("0"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_interval_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editintervalunit_time_unit_invalid() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("2"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_interval_unit(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editintervalunit_is_correct() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("2"), String::from("months")];
        let error_should_be_false = repeating_tasks_edit_interval_unit(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2024-03-01",
                        "time": "00:00",
                        "repeat_number": 2,
                        "repeat_unit": "months",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_editstart_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_start(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editstart_invalid_num_of_args() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter"), String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_start(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editstart_position_not_a_num() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("a"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_start(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editstart_position_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("0"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_start(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editstart_position_isnt_in_range() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("2"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_start(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editstart_date_invalid() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("hello"), String::from("2")];
        let error_should_be_true = repeating_tasks_edit_start(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editstart_time_invalid() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("2023-01-01"),
            String::from("2"),
        ];
        let error_should_be_true = repeating_tasks_edit_start(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editstart_is_correct() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("2023-01-01"),
            String::from("01:01"),
        ];
        let error_should_be_false = repeating_tasks_edit_start(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2024-01-01",
                        "time": "01:01",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2023-01-01",
                        "repeat_original_time": "01:01"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn repeating_tasks_editend_todo_is_empty() {
        // write fresh to repeating tasks so content is known
        let fresh_repeating_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editend_invalid_num_of_args() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter"), String::from("don't matter")];
        let error_should_be_true = repeating_tasks_edit_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editend_position_not_a_num() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("a"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editend_position_is_zero() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("0"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editend_position_isnt_in_range() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("2"), String::from("hello"), String::from("hi")];
        let error_should_be_true = repeating_tasks_edit_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editend_date_invalid() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("hello"), String::from("2")];
        let error_should_be_true = repeating_tasks_edit_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editend_time_invalid() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("2023-01-01"),
            String::from("2"),
        ];
        let error_should_be_true = repeating_tasks_edit_end(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn repeating_tasks_editend_is_correct() {
        // write fresh to repeating tasks so content is known. can't be empty
        let fresh_repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2024-01-01",
                        "repeat_original_time": "00:00"
                    }
                ],
                "done": []
            }
        "#;
        let fresh_repeating_tasks: Tasks = serde_json::from_str(fresh_repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");
        write_changes_to_new_repeating_tasks(fresh_repeating_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("2023-01-01"),
            String::from("01:01"),
        ];
        let error_should_be_false = repeating_tasks_edit_end(arguments);
        let read_test_file = open_repeating_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let repeating_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2023-01-01",
                        "time": "01:01",
                        "repeat_number": 1,
                        "repeat_unit": "year",
                        "repeat_done": false,
                        "repeat_original_date": "2022-01-01",
                        "repeat_original_time": "01:01"
                    }
                ],
                "done": []
            }
        "#;
        let repeating_tasks: Tasks = serde_json::from_str(repeating_tasks).
            context(
                "during testing: the fresh data to put in the new repeating_tasks file wasn't correct. you should never be able to see this"
            ).
            expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, repeating_tasks);
    }

    #[test]
    fn zzzz_rename_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, remove the modified original and rename copy to original

        std::fs::remove_file(path_to_repeating_tasks())
            .context("failed delete modified repeating_tasks.json after running tests")
            .expect("failed to delete repeating_tasks.json after repeating_helpers unit tests");

        std::fs::rename(repeating_tasks_copy_path(), path_to_repeating_tasks())
            .context("failed to rename repeating_tasks_copy to repeating_tasks")
            .expect(
                "failed to rename repeating_tasks_copy to repeating_tasks after tests were done",
            );
    }
}
