use super::repeating_helpers::*;
use crate::functions::json_file_structs::*;
use chrono::{Days, Duration, Local, Months};
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
            _ => return writeln!(writer, "ERROR: You didn't provide a proper time unit. It has to be one of the following: minutes, hours, days, weeks, months, years.").expect("writeln failed")
        }

        // check if the interval is proper
        let interval: u32 = match add.get(counter * 3 - 2).unwrap().parse() {
            Ok(number) => number,
            Err(_) => return writeln!(writer, "ERROR: You didn't provide a proper interval for your repeating task. It can't be negative and can't be above 4294967295. Proper example: chartodo rp-a gym 2 days").expect("writeln failed"),
        };

        // check if interval is 0
        if add.get(counter * 3 - 2).unwrap().parse::<u32>().unwrap() == 0 {
            return writeln!(writer, "ERROR: You can't have an interval of 0, otherwise why are you even making a new repeating task?").expect("writeln failed");
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
            return writeln!(writer, "ERROR: Your specified repeating task in argument set {} was over 40 characters long, which is not allowed.", counter).expect("writeln failed");
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

    // one by one, add new deadline tasks
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
    if dones.len() >= repeating_tasks.todo.len() {
        return writeln!(
            writer,
            "ERROR: You've specified the entire list. Might as well do chartodo repeating-doneall"
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
