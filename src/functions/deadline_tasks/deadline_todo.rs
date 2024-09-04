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
        writeln!(writer, "ERROR: You don't have the right amount of arguments when adding a deadline task. You should have 3, 6, 9, etc. (i.e., divisible by 3) arguments. Proper example: chartodo dl-a new-item 2099-01-01 00:00. Another: chartodo dl-a new-item 2099-01-01 00:00 another-item 2199-01-01 23:59.").expect("writeln failed");

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
        if NaiveTime::parse_from_str(add.get(counter * 3 - 1).unwrap().as_str(), "%H:%M").is_err() {
            writeln!(writer, "ERROR: Your specified time for a new deadline task in argument set {}, '{}', was invalid. Please provide a correct time in a 24-hour format, e.g. 20:05.", counter, add.get(counter * 3 - 1).unwrap().as_str()).expect("writeln failed");

            // error = true
            return true;
        }
        deadline_task.time = Some(add.get(counter * 3 - 1).unwrap().to_string());

        // check date and add to struct
        if NaiveDate::parse_from_str(add.get(counter * 3 - 2).unwrap().as_str(), "%Y-%m-%d")
            .is_err()
        {
            writeln!(writer, "ERROR: Your specified date for a new deadline task in argument set {}, '{}', was invalid. Please provide a correct time in a year-month-day format, e.g. 2099-12-12.", counter, add.get(counter * 3 - 2).unwrap().as_str()).expect("writeln failed");

            // error = true
            return true;
        };
        deadline_task.date = Some(add.get(counter * 3 - 2).unwrap().to_string());

        // check task is not over 100 chars. add to struct
        if add.get(counter * 3 - 3).unwrap().len() > 100 {
            writeln!(writer, "The new deadline task you wanted to add in argument set {}, '{}',  was over 100 characters long, which is not allowed.", counter, add.get(counter * 3 - 3).unwrap().as_str()).expect("writeln failed");

            // error = true
            return true;
        };
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
            repeat_original_date: None,
            repeat_original_time: None,
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
    drop(add_no_time);

    // one by one, add new deadline tasks
    new_deadlines
        .iter()
        .for_each(|task| deadline_tasks.todo.push(task.to_owned()));
    drop(new_deadlines);

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_add_no_date(add_no_date: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if right # of arguments
    if add_no_date.len() % 2 != 0 {
        return writeln!(writer, "You don't have the right amount of arguments when adding a deadline task w/ no time. Proper example: chartodo dl-ant new-item 2099-01-01. Another: chartodo dl-a new-item 2099-01-01 another-item 2199-01-01. After the command dl-ant, there should be 2, 4, 6, etc. arguments.").expect("writeln failed");
    }

    // check how many sets of arguments there are
    let mut counter = add_no_date.len() / 2;

    // loop thru the deadline args and parse for correctness
    // i'm looping from back to front, and that's the order that the new deadline tasks are gonna be added
    let mut new_deadlines: Vec<Task> = vec![];
    while counter > 0 {
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

        // check time. if correct, change format and add to struct
        let time: NaiveTime = match add_no_date.get(counter * 2 - 1).unwrap().parse() {
            Ok(yes) => yes,
            Err(_) => return writeln!(writer, "Your specified time in argument set {} was invalid. Please provide a correct time in a 24-hour format, e.g. 20:05.", counter).expect("writeln failed"),
        };
        deadline_task.time = Some(format!("{}", time.format("%H:%M")));

        // check task is not over 40 chars. add to struct
        if add_no_date.get(counter * 2 - 2).unwrap().len() > 40 {
            return writeln!(writer, "Your specified deadline task in argument set {} was over 40 characters long, which is not allowed.", counter).expect("writeln failed");
        };
        deadline_task.task = add_no_date.get(counter * 2 - 2).unwrap().to_string();

        // default day: Local::now
        deadline_task.date = Some(Local::now().date_naive().to_string());

        // push new correct Task to a vec
        new_deadlines.push(deadline_task);

        counter -= 1;
    }
    drop(add_no_date);

    // one by one, add new deadline tasks
    new_deadlines
        .iter()
        .for_each(|task| deadline_tasks.todo.push(task.to_owned()));
    drop(new_deadlines);

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_done(done: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "The deadline todo list is currently empty. Try adding items to it first."
        )
        .expect("writeln failed");
    }

    // filter for viable positions
    let mut dones: Vec<usize> = vec![];
    done.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= deadline_tasks.todo.len()
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
    if dones.len() >= deadline_tasks.todo.len() {
        return writeln!(
            writer,
            "You've specified the entire list. Might as well do chartodo deadline-doneall"
        )
        .expect("writeln failed");
    }

    // if changing todos to done means the done list overflows, clear done list
    if dones.len() + deadline_tasks.done.len() > 10 {
        deadline_tasks.done.clear();
    }

    // change todos to dones one by one
    dones.iter().for_each(|position| {
        deadline_tasks
            .done
            .push(deadline_tasks.todo.get(*position - 1).unwrap().to_owned());
        deadline_tasks.todo.remove(*position - 1);
    });

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_rmtodo(rmtodo: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "The deadline todo list is currently empty. Try adding items to it first."
        )
        .expect("writeln failed");
    }

    // filter for viable positions
    let mut rmtodos: Vec<usize> = vec![];
    rmtodo.iter().for_each(|item| {
        if item.parse::<usize>().is_ok()
        && !item.is_empty() // this will never trigger smh
        && item.parse::<usize>().unwrap() != 0
        && item.parse::<usize>().unwrap() <= deadline_tasks.todo.len()
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
    if rmtodos.len() >= deadline_tasks.todo.len() {
        return writeln!(
            writer,
            "You might as well do deadline-cleartodo since you want to remove all of the items."
        )
        .expect("writeln failed");
    }

    // remove each item one by one
    rmtodos.iter().for_each(|position| {
        deadline_tasks.todo.remove(*position - 1);
    });
    drop(rmtodos);

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_clear_todo() {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        return writeln!(writer, "The deadline todo list is currently empty. Try adding items to it first before removing any.").expect("writeln failed");
    }

    // clear todo list
    deadline_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_done_all() {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "The deadline todo list is currently empty, so you can't change any todos to done."
        )
        .expect("writeln failed");
    }

    // clear done list if it will overflow
    if deadline_tasks.todo.len() + deadline_tasks.done.len() > 10 {
        deadline_tasks.done.clear();
    }

    // push all todos to done
    deadline_tasks
        .todo
        .iter()
        .for_each(|item| deadline_tasks.done.push(item.to_owned()));
    deadline_tasks.todo.clear();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

// TODO: I can technically give this and all edit commands argument chaining. I think why I haven't yet is just my own discretion
pub fn deadline_tasks_edit_all(position_task_date_time: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "The deadline todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");
    }

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_task_date_time.len() != 4 {
        return writeln!(writer, "You must specify the deadline todo's position and all the parameters that will be edited. A proper example would be: chartodo deadline-editall 4 new-item 2150-01-01 00:00.").expect("writeln failed");
    }

    // check if position is a valid number
    if position_task_date_time
        .first()
        .unwrap()
        .parse::<usize>()
        .is_err()
    {
        return writeln!(
            writer,
            "You must provide a viable position. Try something between 1 and {}",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if position_task_date_time
        .first()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        == 0
    {
        return writeln!(
            writer,
            "Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if position_task_date_time
        .first()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        > deadline_tasks.todo.len()
    {
        return writeln!(
            writer,
            "Your position exceed's the todo list's length. Try something between 1 and {}",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // new item can't be more than 40 chars
    if position_task_date_time.get(1).unwrap().len() > 40 {
        return writeln!(
            writer,
            "Editing a todo item to be more than 40 characters is not allowed"
        )
        .expect("writeln failed");
    }

    // date isn't proper
    if position_task_date_time
        .get(2)
        .unwrap()
        .parse::<NaiveDate>()
        .is_err()
    {
        return writeln!(
            writer,
            "The date provided isn't proper. It must be in a yy-mm-dd format."
        )
        .expect("writeln failed");
    }

    // time isn't proper
    if position_task_date_time
        .last()
        .unwrap()
        .parse::<NaiveTime>()
        .is_err()
    {
        return writeln!(
            writer,
            "The time provided isn't proper. It must be in a 24-hour format, e.g., 23:08"
        )
        .expect("writeln failed");
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
}

// note that I refuse to do all the combinations for editing a deadline task, and will do the same for repeating tasks
// the only combinations i'm going to do are a) editing all the params, and b) editing only one param

pub fn deadline_tasks_edit_task(position_task: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "The deadline todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");
    }

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_task.len() != 2 {
        return writeln!(writer, "You must specify the deadline todo's position that will be edited and what to edit it to. A proper example would be: chartodo dl-eta 4 new-item.").expect("writeln failed");
    }

    // check if position is a valid number
    if position_task.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "You must provide a viable position. Try something between 1 and {}",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if position_task.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if position_task.first().unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len() {
        return writeln!(
            writer,
            "Your position exceed's the todo list's length. Try something between 1 and {}",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // new item can't be more than 40 chars
    if position_task.last().unwrap().len() > 40 {
        return writeln!(
            writer,
            "Editing a todo item to be more than 40 characters is not allowed"
        )
        .expect("writeln failed");
    }

    // edit todo item
    let position: usize = position_task.first().unwrap().parse().unwrap();
    deadline_tasks.todo.get_mut(position - 1).unwrap().task =
        position_task.last().unwrap().to_string();

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_edit_date(position_date: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "The deadline todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");
    }

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_date.len() != 2 {
        return writeln!(writer, "You must specify the deadline todo's position that will be edited and what to edit it to. A proper example would be: chartodo dl-ed 4 2150-01-01.").expect("writeln failed");
    }

    // check if position is a valid number
    if position_date.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "You must provide a viable position. Try something between 1 and {}",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if position_date.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if position_date.first().unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len() {
        return writeln!(
            writer,
            "Your position exceed's the todo list's length. Try something between 1 and {}",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // date isn't proper
    if position_date.last().unwrap().parse::<NaiveDate>().is_err() {
        return writeln!(
            writer,
            "The date provided isn't proper. It must be in a yy-mm-dd format."
        )
        .expect("writeln failed");
    }

    // edit todo item
    let position: usize = position_date.first().unwrap().parse().unwrap();
    deadline_tasks.todo.get_mut(position - 1).unwrap().date =
        Some(position_date.last().unwrap().to_owned());

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_edit_time(position_time: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "The deadline todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");
    }

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if position_time.len() != 2 {
        return writeln!(writer, "You must specify the deadline todo's position that will be edited and what to edit it to. A proper example would be: chartodo dl-eti 4 23:59.").expect("writeln failed");
    }

    // check if position is a valid number
    if position_time.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "You must provide a viable position. Try something between 1 and {}",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if position_time.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "Positions can't be zero. They have to be 1 and above."
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if position_time.first().unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len() {
        return writeln!(
            writer,
            "Your position exceeds the todo list's length. Try something between 1 and {}",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // time isn't proper
    if position_time.last().unwrap().parse::<NaiveTime>().is_err() {
        return writeln!(
            writer,
            "The time provided isn't proper. It must be in a 24-hour format, e.g., 23:08"
        )
        .expect("writeln failed");
    }

    // edit todo item
    let position: usize = position_time.first().unwrap().parse().unwrap();
    deadline_tasks.todo.get_mut(position - 1).unwrap().time =
        Some(position_time.last().unwrap().to_owned());

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}

pub fn deadline_tasks_edit_datetime(edit_date_time: Vec<String>) {
    // housekeeping
    deadline_tasks_create_dir_and_file_if_needed();
    let writer = &mut std::io::stdout();

    // open file and parse
    let mut deadline_tasks = open_deadline_tasks_and_return_tasks_struct();

    // check if todo list is empty
    if deadline_tasks.todo.is_empty() {
        return writeln!(
            writer,
            "The deadline todo list is currently empty, so there are no todos that can be edited."
        )
        .expect("writeln failed");
    }

    // chartodo rp-edt 1 2001-01-01 00:00

    // the following ifs are the multitude of errors i have to check for

    // check if we have the right number of arguments
    if edit_date_time.len() != 3 {
        return writeln!(writer, "ERROR: You must specify the deadline todo's position and what to edit the datetime to. A proper example would be: chartodo deadline-editdatetime 4 2150-01-01 00:00.
            NOTE: nothing changed on the list below.").expect("writeln failed");
    }

    // check if position is a valid number
    if edit_date_time.first().unwrap().parse::<usize>().is_err() {
        return writeln!(
            writer,
            "ERROR: You must provide a viable position. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // positions can't be zero
    if edit_date_time.first().unwrap().parse::<usize>().unwrap() == 0 {
        return writeln!(
            writer,
            "ERROR: Positions can't be zero. They have to be between 1 and {}.
            NOTE: nothing changed on the list below.",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // position not in range of todo list len
    if edit_date_time.first().unwrap().parse::<usize>().unwrap() > deadline_tasks.todo.len() {
        return writeln!(
            writer,
            "ERROR: Your position exceed's the todo list's length. Try something between 1 and {}.
            NOTE: nothing changed on the list below.",
            deadline_tasks.todo.len()
        )
        .expect("writeln failed");
    }

    // date isn't proper
    match NaiveDate::parse_from_str(edit_date_time.get(1).unwrap().as_str(), "%Y-%m-%d") {
        Ok(_) => (),
        Err(_) => return writeln!(writer, "ERROR: You didn't provide a proper date in a year-month-day format. Proper example: 2100-12-24.
            NOTE: nothing changed on the list below.").expect("writeln failed")
    }

    // time isn't proper
    match NaiveTime::parse_from_str(edit_date_time.last().unwrap().as_str(), "%H:%M") {
        Ok(_) => (),
        Err(_) => {
            return writeln!(
            writer,
            "ERROR: You didn't provide a proper time in a 24-hour format. Proper example: 13:28.
            NOTE: nothing changed on the list below."
        )
            .expect("writeln failed")
        }
    }

    // edit todo item
    let position: usize = edit_date_time.first().unwrap().parse().unwrap();
    deadline_tasks.todo.get_mut(position - 1).unwrap().date =
        Some(edit_date_time.get(1).unwrap().to_string());
    deadline_tasks.todo.get_mut(position - 1).unwrap().time =
        Some(edit_date_time.last().unwrap().to_owned());

    // write changes to file
    write_changes_to_new_deadline_tasks(deadline_tasks);
}
