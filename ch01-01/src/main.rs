use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {s: String};
    println!("{}", judge_contain_only_uniqu_chars(s));
}

fn judge_contain_only_uniqu_chars(s: String) -> bool {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample1() {
        assert_eq!(judge_contain_only_uniqu_chars("abcdefg".to_string()), true)
    }
    #[test]
    fn sample2() {
        assert_eq!(judge_contain_only_uniqu_chars("abcdefa".to_string()), false)
    }
    #[test]
    fn sample3() {
        assert_eq!(judge_contain_only_uniqu_chars("aaa".to_string()), false)
    }
    #[test]
    fn sample4() {
        assert_eq!(judge_contain_only_uniqu_chars("".to_string()), true)
    }
}
