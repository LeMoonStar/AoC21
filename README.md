# Advent of Code 2021

[![About](https://img.shields.io/badge/Advent%20of%20Code-2021-brightgreen)](https://adventofcode.com/2021/about)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://en.wikipedia.org/wiki/Rust_(programming_language))
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://mit-license.org/)
![Days completed](https://img.shields.io/badge/Days%20completed-3.5-red)
![Stars](https://img.shields.io/badge/Stars-7-yellow)

This repository contains my solutions for the Advent of Code 2021 challanges written in Rust.  
I am currently still learning rust so please dont jude the quality of my code.

## Usage

To use my solutions you have to have [Cargo](https://www.rust-lang.org/learn/get-started) installed.  
You can compile and run the project by executing `cargo run`, this will automatically download and install any dependencies, compile the project and run the resulting binary. If you don't supply any arguments the program will give you a very simple help message: `Usage: aoc21 day [part]`:

- The day argument is a required number agrument. It has to be in the range from 1-25.
- The part argument is optional, and is used if you want to compute only the first or second part of the day, or explicitly state that both should be conputed.

If you selected a day, the program checks if there is a input file for this day. The name of the file must be `input[DAY].txt`, where `[DAY]` is the number of the day with 2 digits, so `input03.txt` for day 3 or `input18.txt` for day 18. The file must be in the same folder where you run the program.  
If this file is not present, the program will ask you to paste the input manually and then send a EOF marker, so `CTRL+D` on linux and mac or `CTRL+Z` on windows.  
As soon as this is done, the program will start computing the solution and prints it soon after.

## Check out other solutions to AoC21

This is just a small selection of solutions from an small community I am part of.

| Repository                                                                            | Language  |
|---------------------------------------------------------------------------------------|-----------|
| [1Turtle's AdventOfCode](https://github.com/1Turtle/AdventOfCode)                     | Lua(2021) |
| [andi-make's aoc2021](https://github.com/andi-makes/aoc2021)                          | Rust      |
| [joblo2213's Advent-Of-Code-2021](https://github.com/joblo2213/Advent-Of-Code-2021)   | Kotlin    |
| [DerNiklaas's Advent-Of-Code-2021](https://github.com/derNiklaas/Advent-Of-Code-2021) | Kotlin    |
