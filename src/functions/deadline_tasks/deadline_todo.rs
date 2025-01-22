use super::deadline_helpers::*;
use crate::functions::json_file_structs::*;
use chrono::{Local, NaiveDate, NaiveTime};
use std::io::Write;

// chartodo dl-a new-item 2025-01-01 00:00 > len = 3
// chartodo dl-a new-item 2025-01-01 00:00 2nd-item 2025-01-02 00:00 > len = 6

// chartodo dl-ant new-item 2025-01-01 > len = 2

// chartodo dl-and new-item 00:00 > len = 2

pub fn deadline_tasks_add(add: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if we have the right # of args
    // note/potential todo: i'd like to remove division here but idk what else to do lol
    if add.len() % 3 != 0 {
        writeln!(
            writer,
            "ERROR: You don't have the right amount of \
            arguments when adding a deadline task.\n\tThere should be 3, 6, 9, etc. \
            (i.e., divisible by 3) arguments after 'chartodo deadline-add'. You \
            provided {} argument(s).\n\tFormat: chartodo deadline-add ~task ~date ~time \
            [...].\n\t\tDate must be in a yy-mm-dd format. Time must be in a 24-hour \
            format.\n\tExample: chartodo dl-a new-item 2099-01-01 00:00\n\tAnother example: \
            chartodo dl-a new-item 2099-01-01 00:00 another-item 2199-01-01 23:59",
            add.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // the logic for this is that the arguments will be divisible by three if the user did them correctly,
    // thus the arguments will have sets of threes.
    // knowing this, just know how many sets there are and you can access each part of the set using indices
    let mut counter: usize = 1;
    // loop thru the deadline args and parse for correctness
    while counter <= add.len() / 3 {
        // note for interviews: my first instinct was to keep accessing last elem and delete as i go
        // this also used to access from reverse

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
            repeat_original_date: None,
            repeat_original_time: None,
        };

        // check time. if correct, add to struct
        // note that, for micro-optimization purposes, i'm choosing to access the index multiple times instead of assigning
        // it to a variable. con: this has less readability
        match NaiveTime::parse_from_str(add.get(counter * 3 - 1).unwrap().as_str(), "%H:%M") {
            Ok(_) => deadline_task.time = Some(add.get(counter * 3 - 1).unwrap().to_string()),
            Err(_) => {
                writeln!(
                    writer,
                    "ERROR: Your specified time for a new \
                    deadline task in argument set {}, '{}', was invalid. Please provide a correct \
                    time in a 24-hour format, e.g. 20:05.",
                    counter,
                    add.get(counter * 3 - 1).unwrap().as_str()
                )
                .expect("writeln failed");

                // error = true
                return true;
            }
        }

        // check date and add to struct
        match NaiveDate::parse_from_str(add.get(counter * 3 - 2).unwrap().as_str(), "%Y-%m-%d") {
            Ok(_) => deadline_task.date = Some(add.get(counter * 3 - 2).unwrap().to_string()),
            Err(_) => {
                writeln!(
                    writer,
                    "ERROR: Your specified date for a new deadline \
                    task in argument set {}, '{}', was invalid. Please provide a correct time in \
                    a year-month-day format, e.g. 2099-12-12.",
                    counter,
                    add.get(counter * 3 - 2).unwrap().as_str()
                )
                .expect("writeln failed");

                // error = true
                return true;
            }
        }

        // add task
        deadline_task.task = add.get(counter * 3 - 3).unwrap().to_string();

        // push new correct Task to deadline tasks
        deadline_tasks.todo.push(deadline_task);

        counter += 1;
    }

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_add_no_time(add_no_time: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if right # of arguments
    if add_no_time.len() % 2 != 0 {
        writeln!(
            writer,
            "ERROR: You don't have the right amount of arguments \
            when adding a deadline task w/ no time.\n\tThere should be 2, 4, 6, etc. \
            (i.e., divisible by 2) arguments after 'chartodo deadline-addonlydate'. \
            You provided {} argument(s).\n\tFormat: chartodo deadline-addonlydate ~task \
            ~date [...].\n\t\tDate must be in a yy-mm-dd format. The time defaults \
            to 00:00.\n\tExample: chartodo dl-aod new-item 2099-01-01\n\tAnother \
            example: chartodo dl-aod new-item 2099-01-01 another-item 2199-01-01",
            add_no_time.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    let mut counter: usize = 1;
    // loop thru the deadline args and parse for correctness
    while counter <= add_no_time.len() / 2 {
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
            repeat_original_date: None,
            repeat_original_time: None,
        };

        // check date and add to struct
        match NaiveDate::parse_from_str(
            add_no_time.get(counter * 2 - 1).unwrap().as_str(),
            "%Y-%m-%d",
        ) {
            Ok(_) => {
                deadline_task.date = Some(add_no_time.get(counter * 2 - 1).unwrap().to_string())
            }
            Err(_) => {
                writeln!(
                    writer,
                    "ERROR: Your specified date in argument \
                    set {}, '{}', was invalid. Please provide a correct time in a year-month-day \
                    format, e.g. 2099-12-12.",
                    counter,
                    add_no_time.get(counter * 2 - 1).unwrap()
                )
                .expect("writeln failed");

                // error = true
                return true;
            }
        }

        // add task
        deadline_task.task = add_no_time.get(counter * 2 - 2).unwrap().to_string();

        // default time: 00:00
        deadline_task.time = Some("00:00".to_string());

        // push new correct Task to a vec
        deadline_tasks.todo.push(deadline_task);

        counter += 1;
    }

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_add_no_date(add_no_date: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if right # of arguments
    if add_no_date.len() % 2 != 0 {
        writeln!(
            writer,
            "ERROR: You don't have the right amount of arguments \
            when adding a deadline task w/ no time.\n\tThere should be 2, 4, 6, etc. \
            (i.e., divisible by 2) arguments after 'chartodo deadline-addonlytime'. You \
            provided {} argument(s).\n\tFormat: chartodo deadline-addonlytime ~task ~time \
            [...].\n\t\tTime must be in a 24-hour format. The date defaults to your current \
            date.\n\tExample: chartodo dl-aot new-item 00:00\n\tAnother example: chartodo \
            dl-aot new-item 23:59 another-item 23:59",
            add_no_date.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    let mut counter: usize = 1;
    // loop thru the deadline args and parse for correctness
    while counter <= add_no_date.len() / 2 {
        // note for interviews: my first instinct was to keep accessing last elem and delete as i go

        // time: get counter * 2 - 1
        // task: get counter * 2 - 2

        // create new Task struct
        let mut deadline_task = Task {
            task: "".to_string(),
            date: None,
            time: None,
            repeat_number: None,
            repeat_unit: None,
            repeat_done: None,
            repeat_original_date: None,
            repeat_original_time: None,
        };

        // check that time is proper
        match NaiveTime::parse_from_str(add_no_date.get(counter * 2 - 1).unwrap().as_str(), "%H:%M")
        {
            Ok(_) => {
                deadline_task.time = Some(add_no_date.get(counter * 2 - 1).unwrap().to_string())
            }
            Err(_) => {
                writeln!(
                    writer,
                    "ERROR: Your specified time for a new deadline \
                    task in argument set {}, '{}', was invalid. Please provide a correct time \
                    in a 24-hour format, e.g. 20:05.",
                    counter,
                    add_no_date.get(counter * 2 - 1).unwrap()
                )
                .expect("writeln failed");

                // error = true
                return true;
            }
        }

        // add task
        deadline_task.task = add_no_date.get(counter * 2 - 2).unwrap().to_string();

        // default day: Local::now
        deadline_task.date = Some(Local::now().date_naive().to_string());

        // push new correct Task to a vec
        deadline_tasks.todo.push(deadline_task);

        counter += 1;
    }

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_done(mut done: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline todo list is currently empty. Try adding items to it first."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable positions
    for i in (0..done.len()).rev() {
        if done.get(i).unwrap().parse::<usize>().is_err()
            || done.get(i).unwrap().is_empty() // will never trigger
            || done.get(i).unwrap().parse::<usize>().unwrap() == 0
            || done.get(i).unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len()
        {
            done.swap_remove(i);
        }
    }

    // check if none of the args were valid
    if done.is_empty() {
        writeln!(
            writer,
            "ERROR: None of the positions you provided were viable \
            -- they were all either negative, zero, or exceeded the deadline todo list's \
            length."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // sort and dedup
    let mut done: Vec<usize> = done.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    done.sort();
    done.dedup();

    // check if the user basically specified the entire list
    if done.len() >= deadline_tasks.todo.len() && deadline_tasks.todo.len() > 5 {
        writeln!(
            writer,
            "WARNING: You've specified the entire list. Might as well do \
            chartodo deadline-doneall"
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // change todos to dones one by one
    done.iter().rev().for_each(|position| {
        deadline_tasks
            .done
            .push(deadline_tasks.todo.get(position - 1).unwrap().to_owned());
        deadline_tasks.todo.remove(position - 1);
    });

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_rmtodo(mut rmtodo: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline todo list is currently empty. Try adding items \
            to it first."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // filter for viable positions
    for i in (0..rmtodo.len()).rev() {
        if rmtodo.get(i).unwrap().parse::<usize>().is_err()
            || rmtodo.get(i).unwrap().is_empty() // will never trigger
            || rmtodo.get(i).unwrap().parse::<usize>().unwrap() == 0
            || rmtodo.get(i).unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len()
        {
            rmtodo.swap_remove(i);
        }
    }

    // check if none of the args were valid
    if rmtodo.is_empty() {
        writeln!(
            writer,
            "ERROR: None of the positions you provided were viable \
            -- they were all either negative, zero, or exceeded the deadline \
            todo list's length."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // reverse sort
    let mut rmtodo: Vec<usize> = rmtodo.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    rmtodo.sort();
    rmtodo.dedup();

    // check if user wants to remove all of the items
    if rmtodo.len() >= deadline_tasks.todo.len() && deadline_tasks.todo.len() > 5 {
        writeln!(
            writer,
            "WARNING: You might as well do deadline-cleartodo since you want to \
            remove all of the items."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // remove each item one by one
    rmtodo.iter().rev().for_each(|position| {
        deadline_tasks.todo.remove(position - 1);
    });

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_clear_todo() -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline todo list is currently empty. Try \
            adding items to it first before removing any."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // clear todo list
    deadline_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_done_all() -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline todo list is currently empty, so you can't \
            change any todos to done."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // push all todos to done
    deadline_tasks
        .todo
        .iter()
        .for_each(|item| deadline_tasks.done.push(item.to_owned()));
    deadline_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

// TODO: I can technically give this and all edit commands argument chaining. I think why I haven't yet is just my own discretion
pub fn deadline_tasks_edit_all(position_task_date_time: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline todo list is currently empty, so there are no \
            todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // chartodo dl-ea 1 new_item 2150-01-01 00:01

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_task_date_time.len() != 4 {
        writeln!(
            writer,
            "ERROR: You must specify the deadline todo's position \
            and all the parameters that will be edited.\n\tThere should be 4 arguments after \
            'chartodo deadline-editall'. You provided {} argument(s).\n\tFormat: chartodo \
            deadline-editall ~position ~task ~date ~time\n\t\tDate must be in a yy-mm-dd \
            format. Time must be in a 24-hour format.\n\tExample: chartodo dl-ea 4 new-item \
            2150-01-01 00:00",
            position_task_date_time.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if position_task_date_time
        .first()
        .unwrap()
        .parse::<usize>()
        .is_err()
    {
        writeln!(
            writer,
            "ERROR: '{}' isn't a valid position. Try something between 1 and {}.",
            position_task_date_time.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if position_task_date_time
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
    if position_task_date_time
        .first()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        > deadline_tasks.todo.len()
    {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceed's the todo list's length. \
            Try something between 1 and {}",
            position_task_date_time.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // date isn't proper
    if NaiveDate::parse_from_str(position_task_date_time.get(2).unwrap().as_str(), "%Y-%m-%d")
        .is_err()
    {
        writeln!(
            writer,
            "ERROR: The date provided, '{}', isn't \
            proper. It must be in a yy-mm-dd format, e.g., 2001-12-13",
            position_task_date_time.get(2).unwrap()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // time isn't proper
    if NaiveTime::parse_from_str(position_task_date_time.last().unwrap().as_str(), "%H:%M").is_err()
    {
        writeln!(
            writer,
            "ERROR: The time provided, '{}', \
            isn't proper. It must be in a 24-hour format, e.g., 23:08",
            position_task_date_time.last().unwrap()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // edit todo item
    let position: usize = position_task_date_time.first().unwrap().parse().unwrap();
    deadline_tasks.todo.get_mut(position - 1).unwrap().task =
        position_task_date_time.get(1).unwrap().to_string();
    deadline_tasks.todo.get_mut(position - 1).unwrap().date =
        Some(position_task_date_time.get(2).unwrap().to_owned());
    deadline_tasks.todo.get_mut(position - 1).unwrap().time =
        Some(position_task_date_time.last().unwrap().to_owned());

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

// note that I refuse to do all the combinations for editing a deadline task, and will do the same for repeating tasks
// the only combinations i'm going to do are a) editing all the params, and b) editing only one param

pub fn deadline_tasks_edit_task(position_task: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline todo list is currently empty, so there are no \
            todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_task.len() != 2 {
        writeln!(
            writer,
            "ERROR: You must specify the deadline todo's position \
            that will be edited and what to edit the task to.\n\tThere should be 2 arguments \
            after 'chartodo deadline-edittask'. You provided {} argument(s).\n\tFormat: \
            chartodo deadline-edittask ~position ~task.\n\tExample: chartodo dl-eta 4 \
            new-item",
            position_task.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if position_task.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: '{}' isn't a valid position. Try something between 1 and {}.",
            position_task.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if position_task.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if position_task.first().unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceed's the todo list's \
            length. Try something between 1 and {}.",
            position_task.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // edit todo item
    let position: usize = position_task.first().unwrap().parse().unwrap();
    deadline_tasks.todo.get_mut(position - 1).unwrap().task =
        position_task.last().unwrap().to_string();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_edit_date(position_date: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline todo list is currently empty, so there are no \
            todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_date.len() != 2 {
        writeln!(
            writer,
            "ERROR: You must specify the deadline todo's \
            position that will be edited and what to edit the date to.\n\tThere should \
            be two arguments after 'chartodo deadline-editdate'. You provided {} \
            argument(s).\n\tFormat: chartodo deadline-editdate ~position ~date.\n\t\tDate \
            must be in a yy-mm-dd format.\n\tExample: chartodo dl-ed 4 2150-01-01",
            position_date.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if position_date.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: '{}' isn't a valid position. Try something between 1 and {}.",
            position_date.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if position_date.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if position_date.first().unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceeds the todo list's length. Try \
            something between 1 and {}",
            position_date.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // date isn't proper
    if NaiveDate::parse_from_str(position_date.last().unwrap().as_str(), "%Y-%m-%d").is_err() {
        writeln!(
            writer,
            "ERROR: The date provided, '{}', isn't proper. It must be in a \
            yy-mm-dd format, e.g., 2021-12-24.",
            position_date.last().unwrap()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // edit todo item
    let position: usize = position_date.first().unwrap().parse().unwrap();
    deadline_tasks.todo.get_mut(position - 1).unwrap().date =
        Some(position_date.last().unwrap().to_owned());

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_edit_time(position_time: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline todo list is currently empty, so there are no \
            todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_time.len() != 2 {
        writeln!(
            writer,
            "ERROR: You must specify the deadline todo's \
            position that will be edited and what to edit the time to.\n\tThere \
            should be 2 arguments after 'chartodo deadline-edittime'. You provided {} \
            argument(s).\n\tFormat: chartodo deadline-edittime ~position ~time.\n\t\tTime \
            must be in a 24-hour format.\n\tExample: chartodo dl-eti 4 23:59",
            position_time.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if position_time.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: '{}' isn't a valid position. Try something between 1 and {}.",
            position_time.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if position_time.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if position_time.first().unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceeds the todo list's \
            length. Try something between 1 and {}",
            position_time.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // time isn't proper
    if NaiveTime::parse_from_str(position_time.last().unwrap().as_str(), "%H:%M").is_err() {
        writeln!(
            writer,
            "ERROR: The time provided, '{}', isn't proper. It must be in a \
            24-hour format, e.g., 23:08",
            position_time.last().unwrap()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // edit todo item
    let position: usize = position_time.first().unwrap().parse().unwrap();
    deadline_tasks.todo.get_mut(position - 1).unwrap().time =
        Some(position_time.last().unwrap().to_owned());

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

pub fn deadline_tasks_edit_datetime(edit_date_time: Vec<String>) -> bool {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        writeln!(
            writer,
            "ERROR: The deadline todo list is currently empty, so there are no \
            todos that can be edited."
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // chartodo rp-edt 1 2001-01-01 00:00

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_date_time.len() != 3 {
        writeln!(
            writer,
            "ERROR: You must specify the deadline todo's position \
            and what to edit the datetime to.\n\tThere should be 3 arguments after 'chartodo \
            deadline-editdatetime'. You provided {} argument(s).\n\tFormat: chartodo \
            deadline-editdatetime ~position ~date ~time.\n\t\tDate should be in a yy-mm-dd \
            format. Time should be in a 24-hour format.\n\tExample: chartodo dl-edt 4 \
            2150-01-01 00:00",
            edit_date_time.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // check if position is a valid number
    if edit_date_time.first().unwrap().parse::<usize>().is_err() {
        writeln!(
            writer,
            "ERROR: '{}' isn't a valid position. Try something between 1 and {}.",
            edit_date_time.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // positions can't be zero
    if edit_date_time.first().unwrap().parse::<usize>().unwrap() == 0 {
        writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be 1 and above.",
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // position not in range of todo list len
    if edit_date_time.first().unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len() {
        writeln!(
            writer,
            "ERROR: Your position, '{}', exceeds the todo list's length. \
            Try something between 1 and {}.",
            edit_date_time.first().unwrap(),
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // date isn't proper
    if NaiveDate::parse_from_str(edit_date_time.get(1).unwrap().as_str(), "%Y-%m-%d").is_err() {
        writeln!(
            writer,
            "ERROR: '{}' isn't a proper date in a yy-mm-dd format, e.g., \
            2100-12-24.",
            edit_date_time.get(1).unwrap()
        )
        .expect("writeln failed");

        // error = true
        return true;
    }

    // time isn't proper
    if NaiveTime::parse_from_str(edit_date_time.last().unwrap().as_str(), "%H:%M").is_err() {
        writeln!(
            writer,
            "ERROR: '{}' isn't a proper time in a 24-hour format, e.g., 13:28",
            edit_date_time.last().unwrap()
        )
        .expect("writeln failed");

        // erorr = true
        return true;
    }

    // edit todo item
    let position: usize = edit_date_time.first().unwrap().parse().unwrap();
    deadline_tasks.todo.get_mut(position - 1).unwrap().date =
        Some(edit_date_time.get(1).unwrap().to_string());
    deadline_tasks.todo.get_mut(position - 1).unwrap().time =
        Some(edit_date_time.last().unwrap().to_owned());

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);

    // error = false
    false
}

// cargo test deadline_todo_unit_tests -- --test-threads=1
#[cfg(test)]
mod deadline_todo_unit_tests {
    use super::*;
    use anyhow::Context;
    use std::path::PathBuf;

    // these are taken from deadline_helpers
    fn path_to_deadline_tasks() -> PathBuf {
        // get the data dir XDG spec and return it with path to deadline_tasks.json
        let mut deadline_tasks_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        deadline_tasks_path.push("chartodo/deadline_tasks.json");

        deadline_tasks_path
    }

    fn deadline_tasks_copy_path() -> PathBuf {
        // get the path for deadline_tasks_copy.json, which will be used to hold the original contents
        // of deadline_tasks.json while it's getting modified
        let mut deadline_tasks_copy_path = dirs::data_dir()
            .context(
                "linux: couldn't get $HOME/.local/share/
                    windows: couldn't get C:/Users/your_user/AppData/Local/
                    mac: couldn't get /Users/your_user/Library/Application Support/

                    those directories should exist for your OS. please double check that they do.",
            )
            .expect("something went wrong with fetching the user's data dirs");
        deadline_tasks_copy_path.push("chartodo/deadline_tasks_copy.json");

        deadline_tasks_copy_path
    }

    // these have been tested in other fns, these are just included here as a sanity check
    #[test]
    fn deadline_tasks_path_is_correct() {
        let linux_path = "/.local/share/chartodo/deadline_tasks.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/deadline_tasks.json";
        let mac_path = "/Library/Application Support/chartodo/deadline_tasks.json";
        let mut got_deadline_tasks_path: bool = false;
        let deadline_path = path_to_deadline_tasks();
        let deadline_path = deadline_path.to_str().unwrap();

        if deadline_path.contains(linux_path)
            | deadline_path.contains(windows_path)
            | deadline_path.contains(mac_path)
        {
            got_deadline_tasks_path = true;
        }

        assert!(got_deadline_tasks_path);
    }

    #[test]
    fn deadline_tasks_copy_path_is_correct() {
        let linux_path = "/.local/share/chartodo/deadline_tasks_copy.json";
        // note: windows is supposed to have \
        let windows_path = "/AppData/Local/chartodo/deadline_tasks_copy.json";
        let mac_path = "/Library/Application Support/chartodo/deadline_tasks_copy.json";
        let mut got_deadline_tasks_copy_path: bool = false;
        let deadline_tasks_copy_path = deadline_tasks_copy_path();
        let deadline_tasks_copy_path = deadline_tasks_copy_path.to_str().unwrap();

        if deadline_tasks_copy_path.contains(linux_path)
            | deadline_tasks_copy_path.contains(windows_path)
            | deadline_tasks_copy_path.contains(mac_path)
        {
            got_deadline_tasks_copy_path = true;
        }

        assert!(got_deadline_tasks_copy_path);
    }

    #[test]
    fn aaaa_deadline_tasks_clone_file() {
        // name is aaaa so it's done first
        // since we will be modifying the original file to run a test, the original data must be
        // preserved first
        std::fs::File::create(deadline_tasks_copy_path())
            .context("failed to create deadline_tasks_copy.json")
            .expect("failed to create a copy during unit test");

        std::fs::copy(path_to_deadline_tasks(), deadline_tasks_copy_path())
            .context("failed to copy deadline_tasks.json to deadline_tasks_copy.json")
            .expect("failed to copy original file to copy file during unit test");
    }

    #[test]
    fn deadline_tasks_add_incorrect_num_of_args() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("2024-01-01"),
            String::from("00:00"),
            String::from("another"),
            String::from("2025-01-01"),
        ];
        let error_should_be_true = deadline_tasks_add(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_add_time_incorrect() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("2024-01-01"),
            String::from("00:00"),
            String::from("another"),
            String::from("2025-01-01"),
            String::from("25:08"),
        ];
        let error_should_be_true = deadline_tasks_add(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_add_date_incorrect() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("2024-01-01"),
            String::from("00:00"),
            String::from("another"),
            String::from("2025-14-12"),
            String::from("00:08"),
        ];
        let error_should_be_true = deadline_tasks_add(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_add_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("2024-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_false = deadline_tasks_add(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2024-01-01",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_add_multiple_args_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("2024-01-01"),
            String::from("00:00"),
            String::from("hi"),
            String::from("2025-01-01"),
            String::from("13:00"),
        ];
        let error_should_be_false = deadline_tasks_add(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2024-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "13:00",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_add_no_time_incorrect_num_of_args() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("2024-01-01"),
            String::from("00:00"),
            String::from("another"),
            String::from("2025-01-01"),
        ];
        let error_should_be_true = deadline_tasks_add_no_time(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_add_no_time_date_incorrect() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("2024-01-01"),
            String::from("another"),
            String::from("2025-14-12"),
        ];
        let error_should_be_true = deadline_tasks_add_no_time(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_add_no_time_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("2024-01-01"),
        ];
        let error_should_be_false = deadline_tasks_add_no_time(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2024-01-01",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_add_no_time_multiple_args_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("2024-01-01"),
            String::from("hi"),
            String::from("2025-01-01"),
        ];
        let error_should_be_false = deadline_tasks_add_no_time(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "this-is-the-todo-list",
                        "date": "2024-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_add_no_date_incorrect_num_of_args() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("13:00"),
            String::from("00:00"),
            String::from("another"),
            String::from("2025-01-01"),
        ];
        let error_should_be_true = deadline_tasks_add_no_date(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_add_no_date_time_incorrect() {
        // perform actions on file. multiple args so i'm more sure it catches errors
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("00:00"),
            String::from("another"),
            String::from("13:61"),
        ];
        let error_should_be_true = deadline_tasks_add_no_date(arguments);

        assert!(error_should_be_true);
    }

    // idk how to test the file success of add_no_date since that function's result is contingent on what date the fn is called
    // ig for now i can test whether it was successful or not, and just not check the contents
    #[test]
    fn deadline_tasks_add_no_date_is_correct() {
        // perform actions on file
        let arguments: Vec<String> =
            vec![String::from("this-is-the-todo-list"), String::from("13:00")];
        let error_should_be_false = deadline_tasks_add_no_date(arguments);

        assert!(!error_should_be_false);
    }

    #[test]
    fn deadline_tasks_add_no_date_multiple_args_is_correct() {
        // perform actions on file
        let arguments: Vec<String> = vec![
            String::from("this-is-the-todo-list"),
            String::from("13:00"),
            String::from("hi"),
            String::from("14:28"),
        ];
        let error_should_be_false = deadline_tasks_add_no_date(arguments);

        assert!(!error_should_be_false);
    }

    #[test]
    fn deadline_tasks_todo_to_done_todo_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that deadline todo list is correctly identified as empty
        let arguments = vec![String::from("1")];
        let error_should_be_true = deadline_tasks_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_todo_to_done_todo_no_valid_args() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that there are no valid args
        let arguments = vec![String::from("-11"), String::from("0"), String::from("2")];
        let error_should_be_true = deadline_tasks_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_todo_to_done_should_do_deadlinedoneall() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that user should do dl-da
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_should_be_true = deadline_tasks_done(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_todo_to_done_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1")];
        let error_should_be_false = deadline_tasks_done(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_todo_to_done_multiple_args_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "welcome",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("-1"),
            String::from("1"),
        ];
        let error_should_be_false = deadline_tasks_done(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "welcome",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ],
                "done": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    }
                ]
            }
        "#;
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_rmtodo_todo_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that deadline todo list is correctly identified as empty
        let arguments = vec![String::from("1")];
        let error_should_be_true = deadline_tasks_rmtodo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_rmtodo_no_valid_args() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that there are no valid args
        let arguments = vec![String::from("11"), String::from("0"), String::from("-1")];
        let error_should_be_true = deadline_tasks_rmtodo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_rmtodo_should_do_deadlinecleartodo() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // check that there are no valid args
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
            String::from("6"),
        ];
        let error_should_be_true = deadline_tasks_rmtodo(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_rmtodo_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1")];
        let error_should_be_false = deadline_tasks_rmtodo(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_rmtodo_multiple_args_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
                    },
                    {
                        "task": "kumusta",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
                    {
                        "task": "hi",
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("2"),
            String::from("-1"),
            String::from("1"),
        ];
        let error_should_be_false = deadline_tasks_rmtodo(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_cleartodo_todo_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let error_should_be_true = deadline_tasks_clear_todo();

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_cleartodo_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
                    },
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let error_should_be_false = deadline_tasks_clear_todo();
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_doneall_todo_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let error_should_be_true = deadline_tasks_done_all();

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_doneall_is_correct() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
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
                    },
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let error_should_be_false = deadline_tasks_done_all();
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [],
                "done": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "00:00",
                        "repeat_number": null,
                        "repeat_unit": null,
                        "repeat_done": null,
                        "repeat_original_date": null,
                        "repeat_original_time": null
                    },
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
                ]
            }
        "#;
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_editall_todo_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = deadline_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editall_invalid_num_of_args() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("don't matter"),
            String::from("don't matter"),
            String::from("don't matter"),
        ];
        let error_should_be_true = deadline_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editall_position_not_a_num() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("a"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = deadline_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editall_position_is_zero() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("0"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = deadline_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editall_position_isnt_in_range() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("2"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = deadline_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editall_invalid_date() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hello"),
            String::from("2099-21-01"),
            String::from("00:00"),
        ];
        let error_should_be_true = deadline_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editall_invalid_time() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hello"),
            String::from("2099-01-01"),
            String::from("25:00"),
        ];
        let error_should_be_true = deadline_tasks_edit_all(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editall_is_correct() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("hi"),
            String::from("2099-01-01"),
            String::from("13:00"),
        ];
        let error_should_be_false = deadline_tasks_edit_all(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
                        "date": "2099-01-01",
                        "time": "13:00",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_edittask_todo_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = deadline_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittask_invalid_num_of_args() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("don't matter"),
            String::from("don't matter"),
            String::from("don't matter"),
        ];
        let error_should_be_true = deadline_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittask_position_not_a_num() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("a"), String::from("hi")];
        let error_should_be_true = deadline_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittask_position_is_zero() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("0"), String::from("hi")];
        let error_should_be_true = deadline_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittask_position_isnt_in_range() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("2"), String::from("hi")];
        let error_should_be_true = deadline_tasks_edit_task(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittask_is_correct() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("hi")];
        let error_should_be_false = deadline_tasks_edit_task(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hi",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_editdate_todo_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = deadline_tasks_edit_date(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdate_invalid_num_of_args() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("don't matter"),
            String::from("don't matter"),
            String::from("don't matter"),
        ];
        let error_should_be_true = deadline_tasks_edit_date(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdate_position_not_a_num() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("a"), String::from("hi")];
        let error_should_be_true = deadline_tasks_edit_date(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdate_position_is_zero() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("0"), String::from("hi")];
        let error_should_be_true = deadline_tasks_edit_date(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdate_position_isnt_in_range() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("2"), String::from("hi")];
        let error_should_be_true = deadline_tasks_edit_date(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdate_invalid_date() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("2099-21-01")];
        let error_should_be_true = deadline_tasks_edit_date(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdate_is_correct() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("2099-01-01")];
        let error_should_be_false = deadline_tasks_edit_date(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2099-01-01",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_edittime_todo_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = deadline_tasks_edit_time(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittime_invalid_num_of_args() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("don't matter"),
            String::from("don't matter"),
            String::from("don't matter"),
        ];
        let error_should_be_true = deadline_tasks_edit_time(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittime_position_not_a_num() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("a"), String::from("hi")];
        let error_should_be_true = deadline_tasks_edit_time(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittime_position_is_zero() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("0"), String::from("hi")];
        let error_should_be_true = deadline_tasks_edit_time(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittime_position_isnt_in_range() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("2"), String::from("hi")];
        let error_should_be_true = deadline_tasks_edit_time(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittime_invalid_time() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("25:01")];
        let error_should_be_true = deadline_tasks_edit_time(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_edittime_is_correct() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("1"), String::from("23:00")];
        let error_should_be_false = deadline_tasks_edit_time(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2025-01-01",
                        "time": "23:00",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn deadline_tasks_editdatetime_todo_is_empty() {
        // write fresh to deadline tasks so content is known
        let fresh_deadline_tasks = r#"
            {
                "todo": [],
                "done": []
            }
        "#;
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter")];
        let error_should_be_true = deadline_tasks_edit_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdatetime_invalid_num_of_args() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![String::from("don't matter"), String::from("don't matter")];
        let error_should_be_true = deadline_tasks_edit_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdatetime_position_not_a_num() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("a"),
            String::from("2029-01-01"),
            String::from("01:01"),
        ];
        let error_should_be_true = deadline_tasks_edit_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdatetime_position_is_zero() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("0"),
            String::from("2029-01-01"),
            String::from("01:01"),
        ];
        let error_should_be_true = deadline_tasks_edit_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdatetime_position_isnt_in_range() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("2"),
            String::from("2029-01-01"),
            String::from("01:01"),
        ];
        let error_should_be_true = deadline_tasks_edit_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdatetime_invalid_date() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("2099-21-01"),
            String::from("doesn't matter"),
        ];
        let error_should_be_true = deadline_tasks_edit_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdatetime_invalid_time() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("2029-01-01"),
            String::from("25:01"),
        ];
        let error_should_be_true = deadline_tasks_edit_datetime(arguments);

        assert!(error_should_be_true);
    }

    #[test]
    fn deadline_tasks_editdatetime_is_correct() {
        // write fresh to deadline tasks so content is known. can't be empty
        let fresh_deadline_tasks = r#"
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
        let fresh_deadline_tasks: Tasks = serde_json::from_str(fresh_deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");
        write_changes_to_new_deadline_tasks(fresh_deadline_tasks);

        // perform actions
        let arguments = vec![
            String::from("1"),
            String::from("2029-01-01"),
            String::from("01:01"),
        ];
        let error_should_be_false = deadline_tasks_edit_datetime(arguments);
        let read_test_file = open_deadline_tasks_and_return_tasks_struct();

        // this should be the content of the file
        let deadline_tasks = r#"
            {
                "todo": [
                    {
                        "task": "hello",
                        "date": "2029-01-01",
                        "time": "01:01",
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
        let deadline_tasks: Tasks = serde_json::from_str(deadline_tasks)
            .context(
                "during testing: the fresh data to put in the new deadline_tasks \
                file wasn't correct. you should never be able to see this",
            )
            .expect("changing str to tasks struct failed");

        assert!(!error_should_be_false);
        assert_eq!(read_test_file, deadline_tasks);
    }

    #[test]
    fn zzzz_rename_copy_to_original() {
        // name is zzzz so it's done last
        // now that tests are done, remove the modified original and rename copy to original

        std::fs::remove_file(path_to_deadline_tasks())
            .context("failed delete modified deadline_tasks.json after running tests")
            .expect("failed to delete deadline_tasks.json after deadline_helpers unit tests");

        std::fs::rename(deadline_tasks_copy_path(), path_to_deadline_tasks())
            .context("failed to rename deadline_tasks_copy to deadline_tasks")
            .expect("failed to rename deadline_tasks_copy to deadline_tasks after tests were done");
    }
}
