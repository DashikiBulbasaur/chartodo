# CHARTODO

This is an all-in-one TODO cli application that's of my own design. I wanted to make a todo list that I and other people would actually use. It's currently a WIP.

## Why the name CHARTODO

I needed a unique name, and I like Pokemon and I also like Charmander and Charizard, and there's a joke about Rust being 'blazingly fast', so it seemed perfect to combine CHAR and TODO.

## How to install and run

First, make sure you have [Rust installed](https://doc.rust-lang.org/book/ch01-01-installation.html). After that, there are several ways to install the program (ordered by recommendation):

1. (upcoming, not yet available on crates.io)
```sh-session
cargo install chartodo
```
2.
```sh-session
cargo install --git https://github.com/DashikiBulbasaur/chartodo.git --branch master
```
3. clone the repo


Then, to run, either

1. on your terminal, type chartodo [COMMAND], e.g., `chartodo list`
2. if you cloned the repo, go to it using your terminal, and type cargo run [COMMAND], e.g., `cargo run list`

## Usage
```sh-session
Commands (argument chaining is available where it makes sense):

    help | h
        show help
        example: chartodo help
    list | l
        show the todo list
        example: chartodo list
    clearall | ca
        clear everything (TODO, DEADLINE, REPEATING)
        example: chartodo ca

    TODO:
        add | a
            add an item to the todo list. To add a multi-word item, replace space with something like -
            example: chartodo add item
            example: chartodo add new-item
            example: chartodo add 1st-item 2nd-item 3rd-item
        done | d
            change a todo item to done, using a numbered position to specify which one
            example: chartodo done 3
            example: chartodo d 5 1 3 2
        rmtodo | rmt
            remove a todo item from the list, using a numbered position to specify which one
            example: chartodo rmt 4
            example: chartodo rmt 4 3 2
        cleartodo | ct
            clear the todo list
            example: chartodo cleartodo
        doneall | da
            change all todo items to done
            example: chartodo da
        cleardone | cd
            clear the done list
            example: chartodo cd
        clearboth | cb
            clear both todo and done lists
            example: chartodo clearall
        rmdone | rmd
            removes a done item at the specified position
            example: chartodo rmd 4
            exmaple: chartodo rmdone 1 2 3
        notdone | nd
            reverses a done item back to a todo item
            example: chartodo nd 3
            example: chartodo notdone 3 2 1 5
        edit | e
            changes a todo item, with its position specified, to what you want
            example: chartodo edit 3 change-item-to-this
        notdoneall | nda
            reverses all done items back to todo
            example: chartodo nda

    DEADLINE:
        deadline-add | dl-a
            adds a task with a day and time limit. date format: yy-mm-dd. time format: 24-hour
            example: chartodo dl-a go-on-a-run 2099-01-01 08:00
            example: chartodo dl-a go-shopping 2030-12-01 13:00 go-bowling 2030-12-01 15:30
            note that there is no space in the time format
        deadline-addonlydate | dl-aod
            adds a deadline task. only the date is specified and time defaults to 00:00
            example: chartodo dl-aod midnight 2099-12-12
            example: chartodo dl-aod homework1-due 2100-01-01 homework2 2200-01-01
        deadline-addonlytime | dl-aot
            adds a deadline task. only the time is specified and date defaults to current date
            example: chartodo dl-aot homework-due-today 23:59
            example: chartodo dl-aot essay-due-today 23:59
        deadline-done | dl-d
            mark one/several deadline task(s) as done
            example: chartodo dl-d 1
            example: chartodo dl-d 1 2 3 4 5
        deadline-rmtodo | dl-rmt
            remove one or several todo item(s)
            example: chartodo dl-rmt 1
            example: chartodo dl-rmt 1 2 3 4 5
        deadline-cleartodo | deadline-ct
            clear the deadline todo list
            example: chartodo dl-ct
        deadline-doneall | dl-da
            mark the entire deadline todo list as done
            example: chartodo dl-da
        deadline-editall | dl-ea
            edit all the parameters of a deadline todo task
            example: chartodo dl-ea 1 new-item 2150-01-01 00:00
        deadline-edittask | dl-eta
            edit the task parameter of a deadline todo task
            example: chartodo dl-eta 1 new-item
        deadline-editdate | dl-ed
            edit the date parameter of a deadline todo task
            example: chartodo dl-eta 1 2150-01-1
        deadline-edittime | dl-eti
            edit the time parameter of a deadline todo task
            example: chartodo dl-eta 1 23:59
        deadline-clearboth | dl-cb
            clears both of the deadline todo and done lists
            example: chartodo dl-cb
        deadline-rmdone | dl-rmd
            removes a deadline done item
            example: chartodo dl-rmd 1
            example: chartodo dl-rmd 1 2 3 4 5
        deadline-notdone | dl-nd
            reverses a deadline done item back to todo
            example: chartodo dl-nd 1
            example: chartodo dl-nd 1 2 3 4 5
        deadline-cleardone | dl-cd
            clears the deadline done list
            example: chartodo dl-cd
        deadline-notdoneall | dl-nda
            reverses all deadline done items back to todo
            example: chartodo dl-nda

    REPEATING:
        repeating-add | rp-a
            add a repeating task. the task starts from your current date and time
            note that for the repeating time interval, only the following units are allowed:
                seconds, minutes, hours, days, weeks, months, years
            example: chartodo rp-a gym 2 days
            example: chartood rp-a gym 2 days mow 1 week
        repeating-addstart | rp-as
            add a repeating task that starts on your specified datetime
            example: chartodo rp-as task 3 days 2099-01-01 00:00
            example: charotodo rp-as task 3 days 2099-01-01 00:00 task2 4 days 2100-01-01 03:03
        repeating-reset | rp-r
            reset the starting datetime of a repeating task to your current date and time
            example: chartodo rp-r 1
            example: chartodo rp-r 1 2 3 4 5
        repeating-done | rp-d
            mark repeating todos as done
            example: chartodo rp-d 1
            example: chartodo rp-d 1 2 3 4 5
        repeating-notdone | rp-nd
            reverse repeating dones back to todo
            example: chartodo rp-nd 1
            example: chartodo rp-nd 1 2 3 4 5
        repeating-rmtodo | rp-rmt
            remove a repeating todo task
            example: chartodo rp-rmt 1
            example: chartodo rp-rmt 1 2 3 4 5
        repeating-rmdone | rp-rmd
            remove one/several repeating done task(s)
            example: chartodo rp-rmd 1
            example: chartodo rp-rmd 1 2 3 4 5
        repeating-clearboth | rp-cb
            clear the repeating todo and done lists
            example: chartodo rp-cb
```

### Tips on usage

1. Commands that take positions as arguments will ignore/reject invalid inputs such as a) non-numbers, b) 0, c) empty strings (if you can somehow do that in the terminal), and d) bigger index than the todo/done list you're trying to access
2. Max character len for todo items is 15, and the max len for done lists is 10. This is arbitrary and can be increased upon a user's request.
3. Max character len for tasks is 40. This isn't arbitrary as I don't want to encourage super-long tasks that wrap in the terminal. It can be increased upon request.
4. Spaces in the program are used to differentiate separate arguments, so multi-word tasks can instead be separated by a character such as -, e.g., multi-word-task-item
5. For the repeating tasks, the maximum interval for the repeating time is 4294967295.
6. When marking any todo (regular, deadline, repeating) task as done and it would exceed the done list's current set max len, the done list automatically deletes everything it has. This is so users don't have to worry about the done list being too full when marking todos as done. This can be changed upon request.

## Milestones

REGULAR advanced:
- [ ] addtoplace 'item' 'position' (may no longer be under consideration)
- [ ] changeprio 'x-y' (may no longer be under consideration)
- [ ] switchprio 'x-y' (may no longer be under consideration)
---
REPEATING advanced:
- [ ] add repeating task that starts on a given datetime, along with being able to later edit that given starting datetime
- [ ] add repeating task that ends on a given datetime, along with being able to edit that given ending datetime. The main advantage of this is if a user wants to add a monthly task that's due at the end of every month, but it's already the middle of the month
---
- [x] regular task commands (basic)
- [x] deadline task commands (basic)
- [ ] repeating task commands (basic)
---
Some major milestones
- [x] finish the regular task commands
- [x] finish argument chaining
- [x] switch file format to json
- [x] add deadline-based todo items
- [ ] add repeating todo items
- [ ] testing
- [ ] available on crates.io
- [ ] switch to proper argument chaining (if possible), i.e., -a -d -rmt
- [ ] finish the advanced functionalities?
