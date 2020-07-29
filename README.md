# Rimpampa's quines in Rust
I've came across quines while watching [The Art of Code](https://www.youtube.com/watch?v=gdSlcxxYAA8) and after some time I decided to try making my own ones in Rust.

# List
* [first.rs](/first.rs): my first (*successful*) try at writing a quine
* [comment.rs](/comment.rs): the value of `t` when modified will generate a new quine with it's value added in the top and bottom comments
* [macro.rs](/macro.rs) (1): a quine that uses macros. Is this the first one that does it?


Notes:
1. I found out while trying to make it prettier that when using stringify! on functions it tries to indent them in a strange way, I wonder why
