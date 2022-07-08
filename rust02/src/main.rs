struct Point {
    x: f64,
    y: f64,
}

fn point_to_string(point: &Point) -> String {
    format!("x: {}, y: {}", point.x, point.y)
}

impl Point {
    fn to_string(&self) -> String {
        point_to_string(self)
    }
}
trait Hash {
    fn hash(&self) -> u64;
}

trait Eq {
    fn eq(&self, other: &Self) -> bool;
}

impl Hash for bool {
    fn hash(&self) -> u64 {
        if *self {
            0
        } else {
            1
        }
    }
}

impl Hash for i64 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

fn print_hash<T: Hash>(t: &T) {
    println!("The hash is {}", t.hash())
}
fn main() {
    let p = Point { x: 1.2, y: -3.7 };
    let s1 = point_to_string(&p);
    let s2 = p.to_string();
    print!("{s1} {s2}");

    print_hash(&true);
    print_hash(&(10 - 1));
    print_hash(&(10 - 11));
}
