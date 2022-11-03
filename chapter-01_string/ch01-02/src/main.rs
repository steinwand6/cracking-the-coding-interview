fn main() {
    proconio::input! {
        s1: String,
        s2: String
    };
    println!("{}", is_reorder(s1, s2));
}

fn is_reorder(s1: String, s2: String) -> bool {
    let mut ascii_vec: [i32; 128] = [0; 128];
    for c in s1.chars() {
        let pos = c as usize;
        ascii_vec[pos] += 1;
    }
    for c in s2.chars() {
        let pos = c as usize;
        ascii_vec[pos] -= 1;
        if ascii_vec[pos] < 0 {
            return false;
        }
    }
    ascii_vec
        .into_iter()
        .filter(|val| *val != 0)
        .collect::<Vec<i32>>()
        .len()
        == 0
}

#[cfg(test)]
mod test {
    use crate::is_reorder;

    #[test]
    fn test1() {
        assert_eq!(
            is_reorder("123456789".to_string(), "987654321".to_string()),
            true
        );
        assert_eq!(
            is_reorder("912345679".to_string(), "976594321".to_string()),
            true
        );
        assert_eq!(is_reorder("]a@[".to_string(), "[]@a".to_string()), true);
    }

    #[test]
    fn test2() {
        assert_eq!(
            is_reorder("123456789".to_string(), "997654321".to_string()),
            false
        );
        assert_eq!(
            is_reorder("91234679".to_string(), "976594321".to_string()),
            false
        );
        assert_eq!(is_reorder("]b@[".to_string(), "[]@a".to_string()), false);
    }
    #[test]
    fn test3() {
        assert_eq!(is_reorder("".to_string(), "".to_string()), true);
        assert_eq!(is_reorder("".to_string(), " ".to_string()), false);
    }
}
