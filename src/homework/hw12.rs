pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let len = shipments.len() as u32;
    
    if total % len != 0 {
        return 0;
    }
    
    let average = total / len;
    let mut permutations = 0;
    let mut current_diff = 0;
    
    for &shipment in shipments {
        current_diff += shipment as i32 - average as i32;
        permutations += current_diff.abs() as usize;
    }
    
    permutations
}

pub fn gen_shipments(n: usize) -> Vec<u32> {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let mut rand = hasher.finish();

    let mut shipments = Vec::with_capacity(n);
    let base = ((rand % 99) + 1) as u32;
    
    for _ in 0..n {
        shipments.push(base);
    }

    let mut next_rand = || {
        rand = rand.wrapping_mul(1664525).wrapping_add(1013904223);
        rand
    };

    let variations = (next_rand() % (n as u64)) + 1;
    for _ in 0..variations {
        let idx = (next_rand() % (n as u64)) as usize;
        let adjustment = ((next_rand() % 4) + 1) * n as u64;
        
        if shipments[idx] >= adjustment as u32 {
            shipments[idx] -= adjustment as u32;
            let other_idx = (next_rand() % (n as u64)) as usize;
            shipments[other_idx] += adjustment as u32;
        }
    }
    
    shipments
}

fn main() {
    let shipments = vec![8, 2, 2, 4, 4];
    println!("Перестановки для {:?}: {}", shipments, count_permutation(&shipments));
    
    let generated = gen_shipments(5);
    println!("відвантаження: {:?}", generated);
    println!("Потрібні перестановки: {}", count_permutation(&generated));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count_permutation() {
        let shipments1 = vec![8, 2, 2, 4, 4];
        assert_eq!(count_permutation(&shipments1), 4);
        
        let shipments2 = vec![9, 3, 7, 2, 9];
        assert_eq!(count_permutation(&shipments2), 7);
        
        let shipments3 = vec![1, 2, 3];
        assert_eq!(count_permutation(&shipments3), 0);
    }
    
    #[test]
    fn test_gen_shipments() {
        for n in 1..10 {
            let shipments = gen_shipments(n);
            let total: u32 = shipments.iter().sum();
            assert_eq!(total % n as u32, 0, "Потрібно рівномірний розподіл");
        }
    }
}
