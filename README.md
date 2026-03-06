# HOW TO MAKE WORKSHOP CHALLENGES


1. Make the file in the exercies directory
2. Make the exercise definition in info.toml

It must have this structure:
[[exercises]]
name = "bigTEST"
test = false
hint = """
To finish this exercise, you need to …
These links might help you …"""


This format is explained at the top of the file

3. run `rustlings dev update`



# How to get rustlings running

1. `cargo install rustlings`
2. `export PATH="/home/<username>/.cargo/bin:$PATH"`
3. `rustlings` in this directory

Will probably make some bash script that can do this for our participants
Can maybe also install rust analyzer?


# I have added a couple example exercises for now



# Actual plan

Ok the actual plan we will do is: (happy to change this)
- Introduce Rust in slides
- Introduce Cargo in slides

- Introduce rust syntax and immutability in slides (Loops, variables, mutability, functions, etc)
3 problems for basic syntax and immutability

- Introduce Vecs in slides
2 or 3 problems for Vecs

- Introduce Ownership and Borrowing in slides
2 or 3 problems for ownership and borrowing

- Introduce Structures and Enums in slides
2 or 3 problems for structures and enums

- Introduce Traits and Generics in slides
2 or 3 problems for traits and generics

- Introduce Options and Results in slides
2 or 3 problems for options and results

- Introduce Crates in slides
Idk if crates will work with rustlings but we can try to make some problems for using crates
