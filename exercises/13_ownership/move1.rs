// Move vs borrow: fix the code so it compiles. Use borrowing where appropriate.
//
// Hint: fn take(s: String) consumes; fn borrow(s: &str) borrows

fn main() {
    let s = String::from("hello");
    // TODO: Change take(s) to borrow(&s) so s is not moved
    take(s.clone()); // temporary: compiles but wastes a clone
    println!("{}", s);
}

fn take(s: String) {
    println!("took: {}", s);
}

fn borrow(s: &str) {
    println!("borrowed: {}", s);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_borrow() {
        let s = String::from("test");
        super::borrow(&s);
        assert_eq!(s, "test");
    }
}
