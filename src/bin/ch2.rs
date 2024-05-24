#![allow(unused)]

use practice_questions::helpers::list::List;

use std::{collections::HashMap, hash::Hash};

fn main() {
    println!("hello!");
}

// 2.1 Remove Dups
// Remove duplicates from an unsorted linked list.
fn rm_dupes<T>(mut list: List<T>) -> List<T>
where
    T: Eq + PartialEq + Hash + Clone,
{
    // Naïve Solution, has to traverse list twice
    let mut tracker: HashMap<T, bool> = HashMap::with_capacity(list.len());
    let mut temp_list: List<T> = List::new();

    while let Some(elem) = list.pop() {
        if !tracker.contains_key(&elem) {
            tracker.insert(elem.clone(), true);
            temp_list.push(elem);
        }
    }

    let mut output: List<T> = List::new();
    while let Some(elem) = temp_list.pop() {
        output.push(elem);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rmd_empty() {
        let result = rm_dupes(List::<i32>::new());
        let ans = List::<i32>::new();
        assert_eq!(result.peek(), ans.peek());
    }

    #[test]
    fn rmd_no_dupes() {
        let mut result = List::<char>::new();
        let mut ans = List::<char>::new();

        result.push('h');
        result.push('a');
        result.push('i');

        ans.push('h');
        ans.push('a');
        ans.push('i');

        result = rm_dupes(result);

        assert_eq!(result.pop(), ans.pop());
        assert_eq!(result.pop(), ans.pop());
        assert_eq!(result.pop(), ans.pop());
        assert_eq!(result.pop(), ans.pop());
    }

    #[test]
    fn rmd_dupes() {
        let mut result = List::<char>::new();
        let mut ans = List::<char>::new();

        result.push('h');
        result.push('u');
        result.push('l');
        result.push('l');
        result.push('o');
        result.push('l');
        result.push('o');
        result.push('l');

        ans.push('h');
        ans.push('u');
        ans.push('l');
        ans.push('o');

        result = rm_dupes(result);

        dbg!(&result);

        assert_eq!(result.pop(), ans.pop());
        assert_eq!(result.pop(), ans.pop());
        assert_eq!(result.pop(), ans.pop());
        assert_eq!(result.pop(), ans.pop());
        assert_eq!(result.pop(), ans.pop());
    }
}
