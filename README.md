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