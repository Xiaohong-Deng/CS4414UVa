ps2
===

[![Build Status](https://travis-ci.org/cs4414/ps2.svg?branch=master)](https://travis-ci.org/cs4414/ps2)

###problem 1: cd and history
To keep it simple, neither `cd` nor `history` is fully functional as their counterparts in bash are.

e.g. `cd ..` works but `cd ../some/path` doesn't.

As for `history` no files are used for storing history, just a buffer to store the most recent 20 entries. Entries longer than 128 bytes are truncated.

I almost got torn up by trying to implement `history` using an array, a circular array to be precise. Because I thought a vector for this task would be an overkill. Finally I took the kind advice from folks on #rust-beginners channel and used a vector instead. Implementation was finished in a minute. Hope RUST would make life easier in the future when it comes to large array initialization.

###problem 2: run program in the background
I chose to use `spawn()` from process::Command to generate a child process and use `gedit ./path/to/file &` as a test. This functionality was implemented fairly quickly, not as tough as I expected, according to the comment below the problem description.

###problem 3 - 4: I/O redirection and pipes
For redirection, given that it's a feature that takes effect on a single command, the cmd_line parsing part is not too complicated. The format of redirection is as follows: `cmd [args] > filename1 [< filename2]` so the number of ">" or "<" is limited to single one. If `filename1` doesn't exist it will be created.