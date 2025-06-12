pub fn draw_diamond() {
    const HEIGHT: usize = 11;
    const HALF: usize = HEIGHT / 2;
    let mut output = String::new();

    for y in 0..HEIGHT {
        let spaces = if y <= HALF { HALF - y } else { y - HALF };
        let stars = HEIGHT - 2 * spaces;

        for _ in 0..spaces {
            output.push(' ');
        }
        for _ in 0..stars {
            output.push('*');
        }
        output.push('\n');
    }

    print!("{}", output);
}

#[test]
fn test_draw_diamond() {
    draw_diamond();
}

fn main() {
    draw_diamond();
}
