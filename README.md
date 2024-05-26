# CHARTODO

This is a simple TODO cli application that's of my own design. It's currently a WIP and is far from usable. For more information, please look at planning.txt. I wrote it before I started on the program and it's what I use for, well, planning. 

## Why the name CHARTODO

I needed a unique name, and I like Pokemon and I also like Charmander and Charizard, and there's a joke about Rust being 'blazingly fast', so it seemed perfect to combine CHAR and TODO.

# How to install and run

First, make sure you have Rust installed. There are several ways to install:

1.  
```sh-session
$ cargo install --git https://github.com/DashikiBulbasaur/chartodo.git --branch main
```
2. clone the repo
3. (upcoming)
```sh-session
$ cargo install chartodo
```

Then, to run, either

1. if you cloned the repo, go to it using your terminal, and type cargo run [ACTION], e.g., `cargo run list`
2. (upcoming) on your terminal, type chartodo [ACTION], e.g., `chartodo list`

# Milestones

The following functionalities are done
- [x] list 
- [x] add 'x'
- [ ] done 'x'
- [ ] rmtodo 'x'
- [ ] cleartodo
- [ ] rmdone 'x'
- [ ] notdone 'x'
- [ ] cleardone
- [ ] clearall
- [ ] edit 'x' 'abc'
- [ ] changeprio 'x-y'
- [ ] switchprio 'x-y'

Some major milestones 
- [ ] add integration testing 
- [ ] available on crates.io
