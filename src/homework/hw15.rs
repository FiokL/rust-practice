fn solve_alphametic() {
    let mut solutions = Vec::new();
    
    for m in 1..=8 {
        for u in 1..=8 {
            if u == m { continue; }
            for x in 1..=8 {
                if x == m || x == u { continue; }
                for a in 1..=8 {
                    if a == m || a == u || a == x { continue; }
                    for s in 1..=8 {
                        if s == m || s == u || s == x || s == a { continue; }
                        for l in 1..=8 {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for o in 1..=8 {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for n in 1..=8 {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }
                                    
                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let xa = x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;
                                    
                                    // Check if this is a valid division
                                    if xa != 0 && muxa / xa == slon && muxa % xa == 0 {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("Found {} solutions:", solutions.len());
    for (i, sol) in solutions.iter().enumerate() {
        let (m, u, x, a, s, l, o, n) = sol;
        println!("Solution {}:", i + 1);
        println!("  {}{}{}{}", m, u, x, a);
        println!("    {} {}", x, a);
        println!("  -----");
        println!("  {}{}{}{}\n", s, l, o, n);
    }
}

fn main() {
    solve_alphametic();
}
