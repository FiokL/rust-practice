pub fn draw_christmas_tree(sections: usize) {
    let max_width = 2 * sections + 1;
    let mut output = String::new();

    (1..=sections).for_each(|section| {
        (1..=section + 1).for_each(|row| {
            let stars = 2 * row - 1;
            let spaces = (max_width - stars) / 2;
            
            (0..spaces).for_each(|_| output.push(' '));
            (0..stars).for_each(|_| output.push('*'));
            output.push('\n');
        });
    });

    let trunk_spaces = (max_width - 1) / 2;
    (0..trunk_spaces).for_each(|_| output.push(' '));
    output.push('*');
    output.push('\n');

    print!("{}", output);
}

#[test]
fn test_draw_christmas_tree() {
    draw_christmas_tree(3);
}

fn main() {
    draw_christmas_tree(5);
}
