fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = "hello";
    let b = "world";
    println!("{}", longest(a, b));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_longest() {
        assert_eq!(super::longest("a", "ab"), "ab");
    }
}
