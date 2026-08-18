# Introduction

This is my implementation of the exercises from the book "Command-Line Rust, A Project-Based primer for Writing Rust CLIs", 2021 version.

## Exercises

The following CLI tools will be build:
- echo [done]
    - prints provided arguments to stdout.
    - FLAGS:
        - n     Don't print newline at the end.
- cat [done]
    - prints provided files given as arguments to stdout. If no files are given, defaults to stdin.
    - FLAGS:
        - n     print line numbers.
        - b     print non-blank line numbers.
- head [wip]
    - Prints the first lines of files given as arguments. If no files are given, default to stdin.
    - BSD version, i.e. doesn't allow negative values for flags.
    - FLAGS:
        - n     Lines to print (default 10).
        - c     Bytes to print.
- wc
- uniq
- find
- cut
- grep
- comm
- tail
- ls


