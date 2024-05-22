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

// 1.5 One Away
// Check if two strings are 1 or 0 edits (add, remove, change char) away.
fn one_away(sa: &str, sb: &str) -> bool {
    let len_diff = sa.len() as i32 - sb.len() as i32;
    if len_diff.unsigned_abs() as usize > 1 {
        return false;
    }
    let ca: Vec<char> = sa.chars().collect();
    let cb: Vec<char> = sb.chars().collect();

    let mut edited: bool = false;
    let mut a_shift: usize = 0;
    let mut b_shift: usize = 0;

    for i in 0..(match len_diff {
        1 => ca.len(),
        _ => cb.len(),
    }) {
        if ca[i - a_shift] != cb[i - b_shift] {
            if edited {
                return false;
            } else {
                edited = true;
                match len_diff {
                    1 => b_shift = 1,
                    -1 => a_shift = 1,
                    _ => (),
                }
            }
        }
    }
    true
}

// 1.6 String Compression
// Condense repeated characters
fn string_compression(s: String) -> String {
    struct CharCount {
        char: char,
        count: usize,
    }
    impl CharCount {
        fn as_string(&self) -> String {
            let mut string = String::from(self.char);
            string.push_str(&self.count.to_string());
            string
        }
    }

    let mut tracker: Vec<CharCount> = Vec::with_capacity(s.len());
    let mut i: usize = 0;
    let cs: Vec<char> = s.chars().collect();

    while i < s.len() {
        let char = cs[i];
        let mut count: usize = 1;
        while ((i + 1) < cs.len()) && (cs[i] == cs[i + 1]) {
            count += 1;
            i += 1;
        }
        tracker.push(CharCount { char, count });
        i += 1;
    }
    let mut output = String::new();
    tracker
        .iter()
        .for_each(|cc| output.push_str(&cc.as_string()));
    if output.len() < s.len() {
        output
    } else {
        s
    }
}

// 1.7 Rotate Matrix
// Rotate NxN matrix with pixel = [u8; 4] 90deg CW
// Try to do it in place!
fn rotate_matrix(mut mat: Vec<Vec<[u8; 4]>>) -> Vec<Vec<[u8; 4]>> {
    // For i in 0..(N/2).ceil()
    //  For j in (0+i)..((N-1)-i)
    //      "Follow trail"
    let n = mat.len();
    mat.iter().for_each(|row| {
        if row.len() != mat.len() {
            panic!("Matrix height and width must be equal.");
        }
    });

    fn rotated_coords((i, j): (usize, usize), n: usize) -> (usize, usize) {
        (((n - 1) - j), i)
    }

    for j in 0_usize..((n as f32 / 2.0_f32).round() as usize) {
        for i in j..((n - 1) - j) {
            let (i2, j2) = rotated_coords((i, j), n);
            let (i3, j3) = rotated_coords((i2, j2), n);
            let (i4, j4) = rotated_coords((i3, j3), n);
            let temp_val = mat[j2][i2];
            mat[j2][i2] = mat[j][i];
            let temp_val_2 = mat[j3][i3];
            mat[j3][i3] = temp_val;
            let temp_val = mat[j4][i4];
            mat[j4][i4] = temp_val_2;
            mat[j][i] = temp_val;
        }
    }
    mat
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

    #[test]
    fn oa_same() {
        assert_eq!(one_away("fish", "fish"), true);
    }

    #[test]
    fn oa_1_in() {
        assert_eq!(one_away("ple", "pale"), true);
    }

    #[test]
    fn oa_2_in() {
        assert_eq!(one_away("ple", "paled"), false);
    }

    #[test]
    fn oa_1_rm() {
        assert_eq!(one_away("fish", "fsh"), true);
    }

    #[test]
    fn oa_2_rm() {
        assert_eq!(one_away("fish", "is"), false);
    }

    #[test]
    fn oa_1_sw() {
        assert_eq!(one_away("fish", "fist"), true);
    }

    #[test]
    fn oa_2_sw() {
        assert_eq!(one_away("fish", "list"), false);
    }

    #[test]
    fn oa_sp() {
        assert_eq!(one_away("fish", "fi sh"), true);
    }

    #[test]
    fn oa_sw_rm() {
        assert_eq!(one_away("fish", "lis"), false);
    }

    #[test]
    fn oa_sw_in() {
        assert_eq!(one_away("ist", "fish"), false);
    }

    #[test]
    fn sc_same() {
        assert_eq!(string_compression(String::from("babbbb")), "babbbb");
    }

    #[test]
    fn sc_bigger() {
        assert_eq!(string_compression(String::from("fish")), "fish");
    }

    #[test]
    fn sc_compress() {
        assert_eq!(string_compression(String::from("dooooooom")), "d1o7m1");
    }

    #[test]
    fn sc_10_bigger() {
        assert_eq!(
            string_compression(String::from("doooooooooomnbvcxzlakjh")),
            "doooooooooomnbvcxzlakjh"
        );
    }

    #[test]
    fn sc_alternating() {
        assert_eq!(
            string_compression(String::from("ooobbbboooooooobbooooo")),
            "o3b4o8b2o5"
        );
    }

    #[test]
    #[should_panic]
    fn rm_panic() {
        rotate_matrix(vec![vec![[1, 2, 3, 4], [255, 255, 255, 255]]]);
    }

    #[test]
    #[should_panic]
    fn rm_panic_2() {
        rotate_matrix(vec![vec![[0, 0, 0, 0]], vec![[255, 255, 255, 255]]]);
    }

    #[test]
    fn rm_1x1() {
        #[allow(clippy::useless_vec)]
        let ans = vec![vec![[0, 0, 0, 0]]];
        let result = rotate_matrix(vec![vec![[0, 0, 0, 0]]]);
        for i in 0..1 {
            for j in 0..1 {
                assert_eq!(ans[i][j], result[i][j]);
            }
        }
    }

    #[test]
    fn rm_2x2() {
        #[allow(clippy::useless_vec)]
        let ans = vec![
            vec![[3, 3, 3, 3], [1, 1, 1, 1]],
            vec![[4, 4, 4, 4], [2, 2, 2, 2]],
        ];
        #[allow(clippy::useless_vec)]
        let result = rotate_matrix(vec![
            vec![[1, 1, 1, 1], [2, 2, 2, 2]],
            vec![[3, 3, 3, 3], [4, 4, 4, 4]],
        ]);
        assert_eq!(result, ans);
    }

    #[test]
    fn rm_3x3() {
        #[allow(clippy::useless_vec)]
        let ans = vec![
            vec![[1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3]],
            vec![[4, 4, 4, 4], [5, 5, 5, 5], [6, 6, 6, 6]],
            vec![[7, 7, 7, 7], [8, 8, 8, 8], [9, 9, 9, 9]],
        ];
        #[allow(clippy::useless_vec)]
        let result = rotate_matrix(vec![
            vec![[3, 3, 3, 3], [6, 6, 6, 6], [9, 9, 9, 9]],
            vec![[2, 2, 2, 2], [5, 5, 5, 5], [8, 8, 8, 8]],
            vec![[1, 1, 1, 1], [4, 4, 4, 4], [7, 7, 7, 7]],
        ]);
        assert_eq!(result, ans);
    }

    #[test]
    fn rm_4x4() {
        #[allow(clippy::useless_vec)]
        let ans = vec![
            vec![[1, 1, 1, 1], [2, 2, 2, 2], [3, 3, 3, 3], [4, 4, 4, 4]],
            vec![[5, 5, 5, 5], [6, 6, 6, 6], [7, 7, 7, 7], [8, 8, 8, 8]],
            vec![
                [9, 9, 9, 9],
                [10, 10, 10, 10],
                [11, 11, 11, 11],
                [12, 12, 12, 12],
            ],
            vec![
                [13, 13, 13, 13],
                [14, 14, 14, 14],
                [15, 15, 15, 15],
                [16, 16, 16, 16],
            ],
        ];
        #[allow(clippy::useless_vec)]
        let result = rotate_matrix(vec![
            vec![
                [4, 4, 4, 4],
                [8, 8, 8, 8],
                [12, 12, 12, 12],
                [16, 16, 16, 16],
            ],
            vec![
                [3, 3, 3, 3],
                [7, 7, 7, 7],
                [11, 11, 11, 11],
                [15, 15, 15, 15],
            ],
            vec![
                [2, 2, 2, 2],
                [6, 6, 6, 6],
                [10, 10, 10, 10],
                [14, 14, 14, 14],
            ],
            vec![[1, 1, 1, 1], [5, 5, 5, 5], [9, 9, 9, 9], [13, 13, 13, 13]],
        ]);
        assert_eq!(ans, result);
    }
}
