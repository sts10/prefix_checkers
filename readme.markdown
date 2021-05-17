# Prefix Checkers

This project benchmarks multiple implementations of a function to remove prefix words from a list of words.

Practically speaking, I'm trying to see if I can improve the speed of [this function from another project](https://github.com/sts10/tidy/blob/main/src/lib.rs#L134-L154).

## Approaches used or still need to explore

- [X] [fst](https://github.com/BurntSushi/fst)
- [ ] [suffix](https://github.com/BurntSushi/suffix)
- [ ] [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
