pub fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }
    
    let previous = gray(n - 1);
    let mut result = Vec::with_capacity(2 * previous.len());
    
    for code in &previous {
        result.push(format!("0{}", code));
    }
    
    for code in previous.iter().rev() {
        result.push(format!("1{}", code));
    }
    
    result
}

#[test]
fn test_gray_code() {
    let test_data = [
        (0, vec![""]),
        (1, vec!["0", "1"]),
        (2, vec!["00", "01", "10", "11"]),
        (3, vec!["000", "001", "010", "011", 
                 "100", "101", "110", "111"]),
        (4, vec!["0000", "0001", "0010", "0011", 
                 "0100", "0101", "0110", "0111", 
                 "1000", "1001", "1010", "1011",
                 "1100", "1101", "1110", "1111"]),
    ];

    for (n, expected) in test_data.iter() {
        assert_eq!(gray(*n), *expected);
    }
}

fn main() {
    println!("Gray code n=2: {:?}", gray(2));
    println!("Gray code n=3: {:?}", gray(3));
}
