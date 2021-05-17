# Prefix Checkers

This project benchmarks multiple implementations of a function to remove prefix words from a list of words.

Practically speaking, I'm trying to see if I can improve the speed of [this function from another project](https://github.com/sts10/tidy/blob/main/src/lib.rs#L134-L154).

## What are we trying to do here?

Given a list of words, we need to remove any words that are prefixes of other words. So our inputted list had the words "boy" and "boyhood" on it, our outputted list would have "boyhood" but _not_ "boy", since "boy" is a prefix of "boyhood".

## Approaches used or still need to explore

- [X] [fst](https://github.com/BurntSushi/fst) (though it's messy and probably can be improved)
- [ ] [suffix](https://github.com/BurntSushi/suffix)
- [ ] [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)
- [ ] Something I haven't thought of! Create an issue or PR.
