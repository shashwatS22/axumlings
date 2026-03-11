// Clone vs Copy: when to use .clone(). Fix the code.
//
// Hint: Use .clone() when you need a copy of heap data (String, Vec) without moving.

fn main() {
    let s = String::from("hello");
    // TODO: Change to let t = s.clone() so both s and t can be used
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
