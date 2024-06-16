# CHARTODO

This is a simple TODO cli application that's of my own design. It's currently a WIP and is far from usable. For more information, please look at planning.txt. I wrote it before I started on the program and it's what I use for planning.

## Why the name CHARTODO

I needed a unique name, and I like Pokemon and I also like Charmander and Charizard, and there's a joke about Rust being 'blazingly fast', so it seemed perfect to combine CHAR and TODO.

## How to install and run

First, make sure you have [Rust installed](https://doc.rust-lang.org/book/ch01-01-installation.html). After that, there are several ways to install the program (ordered by recommendation):

1. (upcoming)
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
Commands:
    help, h         
        show help
    list, l         
        show the todo list
        example: chartodo list
    add, a          
        add an item to the todo list. To add a multi-word item, replace space with something like _
        example: chartodo add item
        example: chartodo add new_item
    done, d         
        change a todo item to done, using a numbered position to specify which one
        example: 'chartodo done 3' would change the third todo item to done
    rmtodo, rmt     
        remove a todo item from the list, using a numbered position to specify which one
        example: 'chartodo rmt 4' would remove the fourth todo item
```

## Milestones

The following functionalities are done
- [x] list 
- [x] add 'x'
- [x] done 'x'
- [x] rmtodo 'x'
- [x] help
- [ ] cleartodo
- [ ] doneall
- [ ] rmdone 'x'
- [ ] notdone 'x'
- [ ] cleardone
- [ ] clearall
- [ ] edit 'x' 'abc'
- [ ] changeprio 'x-y'
- [ ] switchprio 'x-y'
- [ ] addtoplace 'item' 'position'

Some major milestones
- [ ] finish the basic functionalities
- [ ] add deadline-based todo items
- [ ] available on crates.io
- [ ] finish argument chaining
- [ ] finish the advanced functionalities
