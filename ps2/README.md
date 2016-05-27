ps2
===

[![Build Status](https://travis-ci.org/cs4414/ps2.svg?branch=master)](https://travis-ci.org/cs4414/ps2)

###problem 1
To keep it simple, neither `cd` nor `history` is fully functional as their counterparts in bash are.

e.g. `cd ..` works but `cd ../some/path` doesn't.

As for `history` no files are used for storing history, just a buffer to store the most recent 20 entries. Entries longer than 128 bytes are truncated.

I almost got torn up by trying to implement `history` using an array, a circular array to be precise. Because I thought a vector for this task would be an overkill. Finally I took the kind advice from folks on #rust-beginners channel and used a vector instead. Implementation was finished in a minute.