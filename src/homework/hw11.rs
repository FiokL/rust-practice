use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(n);
    let mut seed = calculate_seed();
    
    for _ in 0..n {
        seed = next_random(seed);
        result.push(10 + (seed % 90) as i32);
    }
    
    result
}

fn calculate_seed() -> u64 {
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .hash(&mut hasher);
    hasher.finish()
}

fn next_random(seed: u64) -> u64 {
    seed.wrapping_mul(1664525).wrapping_add(1013904223)
}

pub fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    data.windows(2)
        .enumerate()
        .map(|(i, pair)| (i, i + 1, pair[0] + pair[1]))
        .min_by_key(|&(_, _, sum)| sum)
        .unwrap_or((0, 0, 0))
}

pub fn print_result(data: &[i32], min_index: usize) {
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>2}. ", i);
    }
    println!();
    
    print!("data:  [");
    for (i, &num) in data.iter().enumerate() {
        if i > 0 { print!(", ") }
        print!("{}", num);
    }
    println!("]");
    
    print!("indexes:");
    for i in 0..data.len() {
        if i == min_index || i == min_index + 1 {
            print!("\\__");
        } else {
            print!("   ");
        }
        print!(" ");
    }
    println!();
    
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        data[min_index] + data[min_index + 1],
        min_index,
        min_index + 1
    );
}

fn main() {
    let data = gen_random_vector(20);
    
    let (min_i, _, _) = min_adjacent_sum(&data);
    
    print_result(&data, min_i);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_min_adjacent_sum() {
        let test_cases = vec![
            (vec![45, 87, 49, 64, 50, 37, 45, 72], (4, 5, 87)),
            (vec![29, 92, 14, 65, 57, 98, 10, 45], (6, 7, 55)),
            (vec![19, 86, 66, 95, 40, 24, 90, 74], (4, 5, 64)),
        ];
        
        for (data, expected) in test_cases {
            assert_eq!(min_adjacent_sum(&data), expected);
        }
    }
    
    #[test]
    fn test_gen_random_vector() {
        let v = gen_random_vector(20);
        assert_eq!(v.len(), 20);
        for &num in &v {
            assert!(num >= 10 && num <= 99);
        }
    }
}
