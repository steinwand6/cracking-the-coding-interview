fn main() {
    let mut str: [char; 30] = [' '; 30];
    let mut length = 0;
    for c in "hello world ! ".chars() {
        str[length] = c;
        length += 1;
    }
    // not index. It is length.
    length -= 1;
    println!("{}, {}", str.iter().collect::<String>(), length);
    urlify(&mut str, length);
    println!("{}", str.iter().collect::<String>());
}

fn urlify(str: &mut [char; 30], mut length: usize) {
    if length == 0 {
        return;
    }
    let mut after_len = length as usize;
    for i in 0..=length {
        if str[i as usize] == ' ' {
            after_len += 2;
        }
    }
    while length != after_len {
        if str[length] != ' ' {
            str[after_len] = str[length];
            if after_len == 0 {
                return;
            }
            after_len -= 1;
            length -= 1;
        } else {
            str[after_len] = '0';
            after_len -= 1;
            str[after_len] = '2';
            after_len -= 1;
            str[after_len] = '%';
            if after_len == 0 {
                return;
            }
            after_len -= 1;
            length -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::urlify;

    #[test]
    fn test1() {
        let mut str: [char; 30] = [' '; 30];
        let mut length = 0;
        for c in " Mr John Smith ".chars() {
            str[length] = c;
            length += 1;
        }
        let expect = str[0..length]
            .iter()
            .collect::<String>()
            .replace(" ", "%20");
        // not index. It is length.
        length -= 1;
        urlify(&mut str, length);
        assert_eq!(str.iter().collect::<String>().trim(), expect);
    }
    #[test]
    fn test2() {
        let mut str: [char; 30] = [' '; 30];
        let mut length = 0;
        for c in "  ".chars() {
            str[length] = c;
            length += 1;
        }
        let expect = str[0..length]
            .iter()
            .collect::<String>()
            .replace(" ", "%20");
        // not index. It is length.
        length -= 1;
        urlify(&mut str, length);
        assert_eq!(str.iter().collect::<String>().trim(), expect);
    }
    #[test]
    fn test3() {
        let mut str: [char; 30] = [' '; 30];
        let length = 0;
        let expect = str[0..length]
            .iter()
            .collect::<String>()
            .replace(" ", "%20");
        urlify(&mut str, length);
        assert_eq!(str.iter().collect::<String>().trim(), expect);
    }
}
