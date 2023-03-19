// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::{collections::HashMap, hash::Hash};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map_magazine = HashMap::new();
    for word in magazine.iter() {
        if map_magazine.get_key_value(word) == None {
            map_magazine.insert(word, 1);
            continue;
        }
        *map_magazine.get_mut(word).unwrap() += 1;
    }
    let mut map_note = HashMap::new();
    for word in note.iter() {
        if map_note.get_key_value(word) == None {
            map_note.insert(word, 1);
            continue;
        }
        *map_note.get_mut(word).unwrap() += 1;
    } 
    for (word, count) in &map_note {
        let mut current_tuple = map_magazine.get_key_value(word);
        if current_tuple != None {
            if current_tuple.unwrap().1 < count {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}
