// Implement Display (and Debug) for a custom type.
//
// Hint: impl std::fmt::Display for MyType { fn fmt(&self, f: &mut Formatter) -> Result }

use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

// TODO: impl fmt::Display for Point { fn fmt(&self, f: &mut Formatter) -> Result { write!(f, "({}, {})", self.x, self.y) } }
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{}", p);
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_display() {
        let p = Point { x: 3, y: 4 };
        assert_eq!(format!("{}", p), "(3, 4)");
    }
}
