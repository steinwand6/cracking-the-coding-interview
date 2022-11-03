use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {s: String};
    println!("{}", is_unique_chars(s));
}

fn is_unique_chars(s: String) -> bool {
    let mut hashmap: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *hashmap.entry(c).or_insert(0) += 1;
    }
    for (_, value) in hashmap {
        if value != 1 {
            return false;
        }
    }
    true
}

fn answer(s: String) -> bool {
    if s.len() > 128 {
        return false;
    }
    let mut char_set = [false; 128];

    for c in s.chars() {
        println!("{}", c);
        let pos = c as usize;
        if char_set[pos] {
            return false;
        }
        char_set[pos] = true;
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample1() {
        assert_eq!(is_unique_chars("abcdefg".to_string()), true)
    }
    #[test]
    fn sample2() {
        assert_eq!(is_unique_chars("abcdefa".to_string()), false)
    }
    #[test]
    fn sample3() {
        assert_eq!(is_unique_chars("aaa".to_string()), false)
    }
    #[test]
    fn sample4() {
        assert_eq!(is_unique_chars("".to_string()), true)
    }
}

#[cfg(test)]
mod test2 {
    use super::*;
    #[test]
    fn sample1() {
        assert_eq!(answer("abcdefg".to_string()), true)
    }
    #[test]
    fn sample2() {
        assert_eq!(answer("abcdefa".to_string()), false)
    }
    #[test]
    fn sample3() {
        assert_eq!(answer("aaa".to_string()), false)
    }
    #[test]
    fn sample4() {
        assert_eq!(answer("".to_string()), true)
    }
}
