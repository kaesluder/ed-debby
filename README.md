## ed-debby

A personal learning project rewriting the venerable `ed` line editor in Rust. 

## Why `ed`? 

- CLI editors are cool, and are useful for things like non-interactive text twiddling, big search/replace operations, and zero-distraction editing.
- Most of the behavior fits on one man page.
- Limited programming scope to focus on doing it right.
- Getting practice with parsing and domain-specific language implementation.


## Why is the project called ed-debby?

First commits were made during hurricane Debby, 2024.

## Differences so Far

It probably won't be possible to get full compatibility, particularly in the area of regular expression processing. Here are some differences so far: 

- Gnu `ed` uses a linked list for the buffer structure. Rust sources strongly recommend Rust's dynamic vector or other data structures instead. For now that seems like a safe bet.

