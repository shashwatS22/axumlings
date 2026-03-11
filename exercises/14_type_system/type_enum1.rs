// Data-carrying enums: define Shape with Circle(f64) and Rect(f64, f64).
//
// Hint: enum Shape { Circle(f64), Rect(f64, f64) }

enum Shape {
    Circle(f64),
    Rect(f64, f64), // TODO: Add this variant and the match arm
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rect(w, h) => w * h,
        }
    }
}

fn main() {
    let c = Shape::Circle(1.0);
    println!("{}", c.area());
}

#[cfg(test)]
mod tests {
    use super::Shape;

    #[test]
    fn test_area() {
        assert!((Shape::Circle(1.0).area() - 3.14159).abs() < 0.01);
        assert_eq!(Shape::Rect(2.0, 3.0).area(), 6.0);
    }
}
