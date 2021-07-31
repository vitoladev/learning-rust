fn main() {
    let x = 1;
    let y = 4;
    let z = 5;

    let Point { x, .. } = new_point(x, y, z);
    println!("x: {}", x);
}

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

fn new_point(x: i64, y: i64, z: i64) -> Point {
    Point { x, y, z }
}
