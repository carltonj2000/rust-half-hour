struct Number {
    odd: bool,
    value: i32,
}

impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl std::marker::Copy for Number {}

fn invert(n: &mut Number) {
    n.value = -n.value;
}

fn print_number_ori(n: Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

fn print_number(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

fn main() {
    let m = Number {
        odd: false,
        value: 42,
    };
    print_number_ori(m); // moved
                         // print_number_ori(m); // moved so will error

    let mut n = Number {
        odd: true,
        value: 51,
    };
    print_number(&n); // borrowed
    invert(&mut n); // borrowed mutably
    print_number(&n);

    let mut o = n.clone();
    let mut o2 = std::clone::Clone::clone(&n);

    o.value += 100;
    o2.value += 200;

    print_number(&n);
    print_number(&o);
    print_number(&o2);

    let n1 = n;
    let n2 = n;

    print_number(&n1); // without the copy trail this would not work
    print_number(&n2);
}
