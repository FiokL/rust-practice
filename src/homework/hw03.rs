pub fn draw_envelope() {
    const WIDTH: usize = 29;
    const HEIGHT: usize = 15;
    let mut output = String::with_capacity((WIDTH + 1) * HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {
                '*' 
            } else if x == y * 2 || x == WIDTH - 1 - y * 2 {
                '*' 
            } else {
                ' ' 
            };
            output.push(c);
        }
        output.push('\n');
    }

    print!("{}", output);
}

fn main() {
    draw_envelope();
}

#[test]
fn test_draw_envelope() {
    draw_envelope();
}
