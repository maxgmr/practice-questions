#![allow(unused)]
use std::collections::HashMap;

fn main() {
    println!("hello!");
}

// 1.1 Is Unique:
// Determine if string has all unique chars
fn is_unique(input_string: &str) -> bool {
    let mut found_chars: HashMap<char, bool> = HashMap::new();
    for c in input_string.chars() {
        match found_chars.get(&c) {
            Some(_) => {
                return false;
            }
            None => {
                found_chars.insert(c, true);
            }
        }
    }
    true
}

// No extra data structs
fn is_unique_2(input_string: &str) -> bool {
    for i in 0..input_string.len() {
        for j in 0..input_string.len() {
            if i != j
                && input_string.chars().nth(i).unwrap() == input_string.chars().nth(j).unwrap()
            {
                return false;
            }
        }
    }
    true
}

// 1.2 Check Permutation:
// Given two strings, check if one is a permutation of the other.
fn check_perm(string_a: &str, string_b: &str) -> bool {
    if string_a.len() != string_b.len() {
        return false;
    }
    let mut sorted_a: Vec<char> = String::from(string_a).chars().collect();
    let mut sorted_b: Vec<char> = String::from(string_b).chars().collect();
    sorted_a.sort();
    sorted_b.sort();
    sorted_a == sorted_b
}

// 1.3 URLify:
// Replace all spaces in string with '%20'
fn urlify(input_string: String) -> String {
    input_string.chars().fold(String::new(), |mut acc, char| {
        if char == ' ' {
            acc.push_str("%20");
            acc
        } else {
            acc.push(char);
            acc
        }
    })
}

// 1.4 Palindrome Permutation
// Check if string is permutation of a palindrome (case-insensitive, ignore spaces)
fn palin_perm(input_string: String) -> bool {
    let input_string = input_string.to_lowercase();
    let mut letter_counter: HashMap<char, usize> = HashMap::with_capacity(input_string.len());

    let mut len_no_whitespace: usize = 0;
    for char in input_string.chars() {
        if !char.is_whitespace() {
            len_no_whitespace += 1;
            if let Some(val) = &mut letter_counter.get(&char) {
                letter_counter.insert(char, *val + 1);
            } else {
                letter_counter.insert(char, 1_usize);
            }
        }
    }

    if len_no_whitespace % 2 == 0 {
        // Even-numbered-length string; palindrome means count of every letter is divisible by 2
        for (_, count) in letter_counter {
            if count % 2 != 0 {
                return false;
            }
        }
        true
    } else {
        // Odd-numbered-length string; palindrome will have 1 odd-counted number and all others
        // will be even-counted

        let mut found_odd: bool = false;
        for (_, count) in letter_counter {
            if count % 2 != 0 {
                if !found_odd {
                    found_odd = true
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn iu_yes() {
        assert_eq!(is_unique("phloam"), true);
    }

    #[test]
    fn iu_no() {
        assert_eq!(is_unique("zzxjoanw"), false);
    }

    #[test]
    fn iu2_yes() {
        assert_eq!(is_unique_2("phloam"), true);
    }

    #[test]
    fn iu2_no() {
        assert_eq!(is_unique_2("zzxjoanw"), false);
    }

    #[test]
    fn cp_yes() {
        assert_eq!(check_perm("doG", "God"), true);
    }

    #[test]
    fn cp_no_case() {
        assert_eq!(check_perm("dog", "God"), false);
    }

    #[test]
    fn cp_no() {
        assert_eq!(check_perm("blah", "foo"), false);
    }

    #[test]
    fn url() {
        assert_eq!(
            urlify(String::from("this is a test")),
            "this%20is%20a%20test"
        );
    }

    #[test]
    fn url2() {
        assert_eq!(urlify(String::from("oo   o  ")), "oo%20%20%20o%20%20");
    }

    #[test]
    fn pp_yes() {
        assert_eq!(palin_perm(String::from("Tact Coa")), true);
    }

    #[test]
    fn pp_no() {
        assert_eq!(palin_perm(String::from("snafu")), false);
    }
}
