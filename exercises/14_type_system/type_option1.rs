// Option combinators: use map, and_then, or unwrap_or_default.
//
// Hint: opt.map(|x| x + 1).unwrap_or(0)

fn double_opt(opt: Option<i32>) -> Option<i32> {
    // TODO: opt.map(|x| x * 2)
    None
}

fn main() {
    println!("{:?}", double_opt(Some(5)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_double() {
        assert_eq!(super::double_opt(Some(5)), Some(10));
        assert_eq!(super::double_opt(None), None);
    }
}
