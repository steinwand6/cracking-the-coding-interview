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
        let pos = c as usize - 'a' as usize;
        ascii_vec[pos] += 1;
    }
    for c in s2.chars() {
        let pos = c as usize - 'a' as usize;
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
