fn main() {
    let s = String::from("hello");
    borrow(&s);
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
