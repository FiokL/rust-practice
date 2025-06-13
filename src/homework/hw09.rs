pub fn rotate(s: String, n: isize) -> String {
    if s.is_empty() {
        return s;
    }

    let len = s.len() as isize;
    let n = n % len;
    
    if n == 0 {
        return s;
    }

    let n = if n < 0 {
        len + n 
    } else {
        n
    };

    let split_point = (len - n) as usize;
    let (left, right) = s.split_at(split_point);
    format!("{}{}", right, left)
}

fn test() {
    let s = "abcdefgh".to_string();
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate(s.clone(), *n), 
                exp.to_string()
            )
        );
    println!("All tests passed!");
}

fn main() {
    let example = "Rust".to_string();
    println!("0: {}", example);
    println!("1: {}", rotate(example.clone(), 1));
    println!("-1: {}", rotate(example.clone(), -1));
    test();
}
