#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point, 
    b: Point,  
}

fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    let min_x = rectangles.iter().map(|r| r.a.x).min().unwrap_or(0);
    let min_y = rectangles.iter().map(|r| r.b.y).min().unwrap_or(0);
    let max_x = rectangles.iter().map(|r| r.b.x).max().unwrap_or(0);
    let max_y = rectangles.iter().map(|r| r.a.y).max().unwrap_or(0);

    let width = (max_x - min_x) as usize;
    let height = (max_y - min_y) as usize;
    let mut grid = vec![vec![false; width + 1]; height + 1];

    for rect in rectangles {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                let grid_x = (x - min_x) as usize;
                let grid_y = (y - min_y) as usize;
                grid[grid_y][grid_x] = true;
            }
        }
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&covered| covered)
        .count() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

#[test]
fn test_single_rectangle() {
    let rect = vec![Rectangle {
        a: Point { x: 0, y: 4 },
        b: Point { x: 4, y: 0 },
    }];
    assert_eq!(area_occupied(&rect), 16);
}

#[test]
fn test_overlapping_rectangles() {
    let rects = vec![
        Rectangle {
            a: Point { x: 0, y: 4 },
            b: Point { x: 4, y: 0 },
        },
        Rectangle {
            a: Point { x: 2, y: 6 },
            b: Point { x: 6, y: 2 },
        },
    ];
    assert_eq!(area_occupied(&rects), 28);
}

fn main() {
    let data = test_data();
    println!("зайняту площу: {}", area_occupied(&data));
}
