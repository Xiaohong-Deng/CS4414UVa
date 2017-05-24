ps2
===

[![Build Status](https://travis-ci.org/cs4414/ps2.svg?branch=master)](https://travis-ci.org/cs4414/ps2)
##Rust Shell Project
###problem 1: cd and history
To keep it simple, neither `cd` nor `history` is fully functional as their counterparts in bash are.

e.g. `cd ..` works but `cd ../some/path` doesn't.

As for `history` no files are used for storing history, just a buffer to store the most recent 20 entries. Entries longer than 128 bytes are truncated.

I almost got torn up by trying to implement `history` using an array, a circular array to be precise. Because I thought a vector for this task would be an overkill. Finally I took the kind advice from folks on [#rust-beginners](https://client00.chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners) channel and used a vector instead. Implementation was finished in a minute. Hope RUST would make life easier in the future when it comes to large array initialization.

###problem 2: run program in the background
I chose to use `spawn()` from process::Command to generate a child process and use `gedit ./path/to/file &` as a test. This functionality was implemented fairly quickly, not as tough as I expected, according to the comment below the problem description.

###problem 3 - 4: I/O redirection and pipes
With IO redirection introduced to gash, running program in the background needs to be rewritten and there is compatibility issue to be considered regarding IO, bg and pipes.

1. Redirection has higher priority than pipes in bash, e.g. `ls > filename | grep zip` redirects the output of ls to `filename`, and `grep`is not executed after that. In `ls | grep zip < filename` the output of ls is ignored and grep takes inpout from `filename`.
2. In order to avoid unsfae block incured by converting `Rawfd` to `Childio`, I chose to do redirection outside of a child process. Take `ls` as an example, pass `piped()` method to the process running `ls`, write the result to the destination file handler in the parent process, which is likely to be the `main()`.
3. The redirection and pipes in `bg` need to be handled because if a process hasn't finished the redirection and pipes are not over yet. It's a good idea to put all these code in a child thread. As long as gash, a.k.a `main()`, is still running, a child thread handler may be dropped if out of scope but the thread itself would remain being alive. That's good because once gash is exited all bg processes will be terminated.