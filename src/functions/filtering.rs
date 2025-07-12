// check if something is ranged position. several fail states:
// 1) if there is more than one - in the range, i.e., can't be 6--10 or -6-10
// 2) no - in item, i.e., it should be 6-10
// 3) if the numbers in the item are invalid, e.g., shouldn't be a-b or a-9
// 4) if the first number is equal to or bigger than the second number.
// if this is the case, i could just reverse it, but i don't want to
// 5) if the second bound is bigger than the todo list's len
// this is mostly to just stop big numbers that might slow down the program
fn check_if_range_positioning(range: String, list_len: usize) -> (bool, usize, usize) {
    let mut passed_line: bool = false;
    let mut first_bound: String = String::from("");
    let mut second_bound: String = String::from("");
    let mut error: bool = false;
    let mut line_counter: u8 = 0;

    for character in range.chars() {
        if character == '-' {
            line_counter += 1;
            passed_line = true;
        }
        if line_counter > 1 {
            error = true;
            return (error, 1, 1);
        }
        if character != '-' {
            if !passed_line {
                first_bound.push(character);
            // this is disgusting bruh
            } else {
                second_bound.push(character);
            }
        }
    }

    // due to how it adds numbers to the bounds in the loop, this also automatically
    // checks if there was a - in the range
    if first_bound.parse::<usize>().is_err() | second_bound.parse::<usize>().is_err() {
        error = true;
        return (error, 1, 1);
    }

    let first_bound = first_bound.parse::<usize>().unwrap();
    let second_bound = second_bound.parse::<usize>().unwrap();

    if first_bound >= second_bound {
        error = true;
        return (error, 1, 1);
    }

    if second_bound > list_len {
        error = true;
        return (error, 1, 1);
    }

    (error, first_bound, second_bound)
}

// following a check that the item is indeed a ranged, unwrap the ranged and return all of them
fn unwrap_range_positioning(mut bound1: usize, bound2: usize) -> Vec<usize> {
    let mut unwrap_bounds: Vec<usize> = vec![];

    while bound1 <= bound2 {
        unwrap_bounds.push(bound1);
        bound1 += 1;
    }

    unwrap_bounds
}

// for the record, i hate that this is a separate iteration
// go thru list and check if an item is ranged. if yes, unwrap it and push to original list
pub fn ranged_positioning_filter(mut arg_list: Vec<String>, task_list_len: usize) -> Vec<String> {
    for i in (0..arg_list.len()).rev() {
        let (error_or_not, bound1, bound2) =
            check_if_range_positioning(arg_list.get(i).unwrap().to_string(), task_list_len);

        if !error_or_not {
            let unwrapped_range = unwrap_range_positioning(bound1, bound2);
            unwrapped_range
                .iter()
                .for_each(|number| arg_list.push(number.to_string()));
            // this is not good
        }
    }
    arg_list
}

pub fn positioning_filter(mut arg_list: Vec<String>, task_list_len: usize) -> Vec<String> {
    for i in (0..arg_list.len()).rev() {
        if arg_list.get(i).unwrap().parse::<usize>().is_err()
            || arg_list.get(i).unwrap().is_empty() // this will never trigger smh
            || arg_list.get(i).unwrap().parse::<usize>().unwrap() == 0
            || arg_list.get(i).unwrap().parse::<usize>().unwrap() > task_list_len
        {
            arg_list.swap_remove(i);
        }
    }

    arg_list
}

pub fn sort_and_dedup(arg_list: Vec<String>) -> Vec<usize> {
    let mut arg_list: Vec<usize> = arg_list
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    arg_list.sort();
    arg_list.dedup();

    arg_list
}

#[cfg(test)]
mod filtering_unit_tests {
    use super::*;

    #[test]
    fn more_than_one_dash_in_range() {
        let more_than_one_dash = check_if_range_positioning(String::from("6--10"), 11);

        assert_eq!(more_than_one_dash, (true, 1, 1));
    }

    #[test]
    fn no_dash_in_rage() {
        let no_dash = check_if_range_positioning(String::from("610"), 11);

        assert_eq!(no_dash, (true, 1, 1));
    }

    #[test]
    fn numbers_invalid_in_range() {
        let numbers_invalid_1 = check_if_range_positioning(String::from("a-b"), 11);
        let numbers_invalid_2 = check_if_range_positioning(String::from("a-10"), 11);
        let numbers_invalid_3 = check_if_range_positioning(String::from("6-b"), 11);
        let numbers_invalid_4 = check_if_range_positioning(String::from("6-"), 11);
        let numbers_invalid_5 = check_if_range_positioning(String::from("-10"), 11);

        assert_eq!(numbers_invalid_1, (true, 1, 1));
        assert_eq!(numbers_invalid_2, (true, 1, 1));
        assert_eq!(numbers_invalid_3, (true, 1, 1));
        assert_eq!(numbers_invalid_4, (true, 1, 1));
        assert_eq!(numbers_invalid_5, (true, 1, 1));
    }

    #[test]
    fn first_bound_higher_than_second() {
        let first_higher_than_second = check_if_range_positioning(String::from("3-1"), 4);

        assert_eq!(first_higher_than_second, (true, 1, 1));
    }

    #[test]
    fn first_equal_to_second() {
        let first_equal_to_second = check_if_range_positioning(String::from("1-1"), 2);

        assert_eq!(first_equal_to_second, (true, 1, 1));
    }

    #[test]
    fn second_higher_than_len() {
        let second_higher_than_len = check_if_range_positioning(String::from("1-3"), 2);

        assert_eq!(second_higher_than_len, (true, 1, 1));
    }

    #[test]
    fn range_positioning_is_correct() {
        let correct = check_if_range_positioning(String::from("6-10"), 12);

        assert_eq!(correct, (false, 6, 10));
    }

    #[test]
    fn range_unwrap_correct() {
        let unwrap_range = unwrap_range_positioning(6, 10);

        assert_eq!(unwrap_range, vec![6, 7, 8, 9, 10]);
    }
}
