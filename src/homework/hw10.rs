pub fn is_palindrome(x: u32) -> bool {
    if x < 10 {
        return true;
    }

    let mut num = x;
    let mut reversed = 0;
    let original = x;

    while num > 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }

    original == reversed
}

fn test() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
        (1, true), 
        (11, true),   
        (12, false),    
        (12321, true),  
        (123321, true),  
        (1234321, true),
        (123456, false), 
    ];

    data.iter().for_each(|(n, exp)| {
        assert_eq!(is_palindrome(*n), *exp);
    });
}

fn main() {
    println!("123 {}", is_palindrome(123));
    println!("121 {}", is_palindrome(121));
    
    test();
}
