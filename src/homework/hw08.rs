fn is_prime(n: &u32) -> bool {
    let num = *n;
    
    if num <= 1 {
        return false;
    }
    
    if num == 2 {
        return true;
    }
    
    if num % 2 == 0 {
        return false;
    }
    
    let sqrt_num = (num as f64).sqrt() as u32 + 1;
    for i in (3..sqrt_num).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    
    true
}

fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    test_data
        .iter()
        .for_each(|(n, prime)| 
            assert_eq!(is_prime(n), *prime)
        );
}

fn main() {
    let numbers = [7, 10, 13, 20];
    for n in numbers.iter() {
        println!("{} prime? {}", n, is_prime(n));
    }
    
    test_is_prime();
}
