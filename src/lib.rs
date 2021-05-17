use fst::{Set, Streamer};
use std::str::from_utf8; // converts UTF-8 bytes to a Rust string

pub fn remove_prefix_words_vectors(list: &[String]) -> Vec<String> {
    let mut list_without_prefix_words = list.to_vec();
    list_without_prefix_words.retain(|potential_prefix_word| {
        for word in list {
            if word.starts_with(potential_prefix_word) && word != potential_prefix_word {
                // This is a prefix word, so we do NOT want to retain it. return false to the
                // retain
                return false;
            } else {
                // This particular word is not a prefix word of this potential_prefix_word.
                // keep looping
                continue;
            };
        }
        // If we've made it here, we can be sure that potential_prefix_word is NOT a
        // prefix word. So we want to retain it for the list_without_prefix_words.
        // To do this, we return true to the retain.
        true
    });
    list_without_prefix_words
}

pub fn remove_prefix_words_fst_sets(list: &[String]) -> Vec<String> {
    let mut list_without_prefix_words = vec![];
    let set = Set::from_iter(list).expect("Couldn't create set");
    // Ask the set for a stream of all of its keys.
    let mut outer_stream = set.stream();

    // Iterate over the elements
    while let Some(outer_key) = outer_stream.next() {
        let outer_key = from_utf8(outer_key).expect("Key is not utf8").to_string();
        let mut is_outer_key_a_prefix_word = false;

        let mut inner_stream = set.stream(); // wonder how costly this is to have inside even one while loop
        while let Some(inner_key) = inner_stream.next() {
            let inner_key = from_utf8(inner_key).expect("Key is not utf8").to_string();
            if inner_key.starts_with(&outer_key) && inner_key != outer_key {
                is_outer_key_a_prefix_word = true;
            }
        }
        if is_outer_key_a_prefix_word == false {
            list_without_prefix_words.push(outer_key.to_string());
        }
    }
    list_without_prefix_words
}

#[cfg(test)]
mod test {
    use super::*;
    // use std::fmt::Debug;
    fn make_lists() -> (Vec<String>, Vec<String>) {
        (
            vec![
                "zookeeper",
                "apple",
                "cHarLie",
                "app",
                "tea",
                "addiction",
                "zoo",
                "stationary",
                "tea",
                "station",
                "apple",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect(),
            vec![
                "  wizard  ",
                "ardoR",
                "tea",
                "   ",
                "11225	active",
                "11152	acclaim",
                "11156	word	tabs",
                "19-6-8 clad",
                "be",
                "I",
                "vAcation",
                "take",
                "",
                "mistake",
                "post-modern",
                "13910 word with spaces in it",
                "  h as spaces ",
            ]
            .iter()
            .map(|x| x.to_string())
            .collect(),
        )
    }

    #[test]
    fn can_remove_prefix_words() {
        let mut list = make_lists().0;
        list.sort();
        let new_list = remove_prefix_words_fst_sets(&list);
        println!("new list is {:?}", new_list);
        assert!(!new_list.contains(&"station".to_string()));
        assert!(new_list.contains(&"stationary".to_string()));
        assert!(!new_list.contains(&"zoo".to_string()));
        assert!(new_list.contains(&"zookeeper".to_string()));
        assert!(new_list.contains(&"apple".to_string()));
    }
}
