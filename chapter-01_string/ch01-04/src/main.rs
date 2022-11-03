// 文字列の文字数が偶数の時：全ての出現文字の出現回数が偶数なら回文だろう
// 文字列の文字数が奇数の時：1つの出現文字の出現回数が奇数、残り全ての出現文字の出現回数が偶数なら回文だろう
// 上記の前提で考えて確認する

use std::collections::HashMap;

fn main() {
    let s = "Tact Coa".to_string();
    println!("{} is permutation? -> {}", s, is_permutation_set(&s));
}

fn is_permutation_set(s: &String) -> bool {
    let mut hashmap = HashMap::new();
    let s = s.replace(" ", "").to_lowercase();
    for c in s.chars() {
        hashmap
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    println!("{}", s);
    let mut odd = false;
    for (_, count) in hashmap {
        if count % 2 == 1 {
            if odd || s.len() % 2 == 0 {
                return false;
            }
            odd = true;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(is_permutation_set(&"Tact Coa".to_string()), true);
        assert_eq!(is_permutation_set(&". Dog is si God.".to_string()), true);
    }
    #[test]
    fn test2() {
        assert_eq!(is_permutation_set(&"Tact Coai".to_string()), false);
        assert_eq!(is_permutation_set(&"Inu is nui".to_string()), false);
    }
    #[test]
    fn test3() {
        assert_eq!(is_permutation_set(&"".to_string()), true);
    }
}
