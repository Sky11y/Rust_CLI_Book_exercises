# Introduction

This is my implementation of the exercises from the book "Command-Line Rust, A Project-Based primer for Writing Rust CLIs", 2021 version.

## Exercises

The following CLI tools will be build:
- echo [done]
    - Prints provided arguments to stdout.
    - FLAGS:
        - n &emsp;                      Don't print newline at the end.
- cat [done]
    - Prints provided files given as arguments to stdout. If no files are given, defaults to stdin.
    - Both flags are not allowed to be set at the same time.
    - FLAGS:
        - n | number &emsp;             print line numbers.
        - b | number-nonblank &emsp;    print non-blank line numbers.
- head [done]
    - Prints the first lines of files given as arguments. If no files are given, default to stdin.
    - BSD version, i.e. doesn't allow negative values for flags.
    - Both flags are not allowed to be set at the same time.
    - FLAGS:
        - n | lines &emsp;              Lines to print (default 10).
        - c | bytes &emsp;              Bytes to print.
- wc [done]
    - Prints the number of lines, words, bytes, and characters in each file given as argument, to stdout.
    - If no flag is set, defaults to -c -l -w
    - If at least one flag is set, prints only the flags set.
    - Flags -c and -m can't be set at the same time.
    - FLAGS:
        - c | bytes &emsp;              Display number of bytes. 
        - l | lines &emsp;              Display number of lines. 
        - w | words &emsp;              Display number of words.
        - m | chars &emsp;              Display number of characters.
    - Note. I used GNU version of wc, which seem to have different padding for the values depending on the input. So to pass the tests I had to implement some padding functionality.
- uniq
- find
- cut
- grep
- comm
- tail
- ls


