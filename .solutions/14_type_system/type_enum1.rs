use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
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
        use std::f64::consts::PI;
        assert!((Shape::Circle(1.0).area() - PI).abs() < 0.01);
        assert_eq!(Shape::Rect(2.0, 3.0).area(), 6.0);
    }
}
