# Introduction

This is my implementation of the exercises from the book "Command-Line Rust, A Project-Based primer for Writing Rust CLIs", 2021 version.

"old" clap (v. 2.33) with Arg:: syntax is used for commands: echo, cat, head, and wc.
"new" clap (v. 4^) is used for everything else (uniq, find, cut, grep, comm, tail, ls).

## Exercises

The following CLI tools will be build:
- echo [done]
    - Prints provided arguments to stdout.
    - FLAGS

    | Short | Long | About |
    | ----|----|----|
    | n | n/a | Don't print newline at the end |
- cat [done]
    - Prints provided files given as arguments to stdout. If no files are given, defaults to stdin.
    - Both flags are not allowed to be set at the same time.
    - FLAGS

    | Short | Long | About |
    | ----|----|----|
    | n | number | print line numbers |
    | b | number-nonblank | print non-blank line numbers |
- head [done]
    - Prints the first lines of files given as arguments. If no files are given, default to stdin.
    - BSD version, i.e. doesn't allow negative values for flags.
    - Both flags are not allowed to be set at the same time.
    - FLAGS

    | Short | Long | About |
    | ----|----|----|
    | n | lines | Lines to print (default 10) |
    | c | bytes | Bytes to print |
- wc [done]
    - Prints the number of lines, words, bytes, and characters in each file given as argument, to stdout.
    - If no flag is set, defaults to -c -l -w
    - If at least one flag is set, prints only the flags set.
    - Flags -c and -m can't be set at the same time.
    - FLAGS

    | Short | Long | About |
    | ----|----|----|
    | c | bytes | Display number of bytes |
    | l | lines | Display number of lines |
    | w | words | Display number of words |
    | m | chars | Display number of characters |

    - Note. I used GNU version of wc, which seem to have different padding for the values depending on the input. So to pass the tests I had to implement some padding functionality.
- uniq
- find
- cut
- grep
- comm
- tail
- ls


