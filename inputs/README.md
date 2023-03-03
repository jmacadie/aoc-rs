# Inputs Script

Small script to download any missing input files from the [Advent Of Code](https://adventofcode.com) website.

Because [we're not meant to commit our input files](https://www.reddit.com/r/adventofcode/comments/zh2hk0/2022friendly_reminder_dont_commit_your_input/)
my repo is inputless & downloading all the missing inputs is annoying. So I wrote this little script
to automatically find any days with missing input and download it to the right place.

It also means that anyone else who wants to run my code on their inputs can easily use this script to sync
every day to their inputs.

## Installation

For convenience, I have made this into a cargo extension command. After installation, you can run the download
script from anywhere in the repo with `cargo aoc-inputs`

To install run `cargo install --path .` from inside the inputs source code folder

## Requirements

This script does rely on a couple of things:

* It assumes that this script is in a subfolder from the root directory 
* It assumes that all days in in subfolders from root with the format `yyyy/day_dd`
* You have an environment variable `AOC_SESSION` set up with your AoC SessionID stored in it
* The script checks whether it is in the correct repo, by comparing the remote repo with my github link.
This will be brittle if the repo is forked, or other remotes are added

The repo is already set up for the first two points, so you don't need to do anything special on that front.

The third point will require you to do something. You can run `$ export AOC_SESSION=x` (replacing x with your
actual session ID) to set the variable in bash or zsh.
Other shells may require something else.

## Warning

This is a very simple script that does not check the return status of the get request. If the website is down
or you've given a bad session ID you won't get a 200 back but the script will silently fail.

You've been warned!
