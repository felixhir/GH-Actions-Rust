fn main() {
    println!("Hello, world!");
}

struct Calculator {
    x: i32,
    y: i32,
}

impl Calculator {
    fn sum(&self) -> i32 {
        self.x + self.y
    }

    fn difference(&self) -> i32 {
        self.x - self.y
    }

    fn product(&self) -> i32 {
        self.x * self.y
    }

    fn quotient(&self) -> f32 {
        self.x as f32 / self.y as f32
    }
}

#[test]
fn it_adds() {
    let calc = Calculator {
        x: 1,
        y: 5
    };
    assert_eq!(calc.sum(), 6);
}

#[test]
fn it_subtracts() {
    let calc = Calculator {
        x: 1,
        y: 5
    };
    assert_eq!(calc.difference(), -4);
}

#[test]
fn it_multiplies() {
    let calc = Calculator {
        x: 2,
        y: 5
    };
    assert_eq!(calc.product(), 10);
}

#[test]
fn it_divides() {
    let calc = Calculator {
        x: 1,
        y: 5
    };
    assert_eq!(calc.quotient(), 0.2);
}