#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Unknown,
}

fn get_opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Unknown => Direction::Unknown,
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Can't read line");
    println!("{:?}" , match buf.trim().to_lowercase().as_str() {
        "left" => get_opposite_direction(Direction::Left),
        "right" => get_opposite_direction(Direction::Right),
        "up" => get_opposite_direction(Direction::Up),
        "down" => get_opposite_direction(Direction::Down),
        _ => Direction::Unknown,
    });
}