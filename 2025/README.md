# WIP: Advent of Code 2025

![](https://img.shields.io/badge/stars%20⭐-8-yellow)
![](https://img.shields.io/badge/days%20completed-4-red)

This directory contains some solutions for advent of code 2025 as well as a CLI to make some things easier.

## CLI

The cli binary offers the following commands:

- `download <day>` -- downloads the problem input and description for the given day
- `scaffold <day>` -- create a new rust file from the template for the given day
- `solve <day> [-t,--test|-r,--release]` -- run the code for the day with the test or the real input

There are some cargo aliases configured so that they can be used as if they were cargo commands (`cargo download 1`)
