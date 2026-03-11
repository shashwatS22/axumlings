fn main() {
    let s = String::from("hello");
    let t = s.clone();
    println!("s: {}, t: {}", s, t);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_clone() {
        let s = String::from("hi");
        let t = s.clone();
        assert_eq!(s, "hi");
        assert_eq!(t, "hi");
    }
}
