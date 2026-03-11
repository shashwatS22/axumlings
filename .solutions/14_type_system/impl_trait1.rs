fn evens() -> impl Iterator<Item = i32> {
    (0..10).filter(|x| x % 2 == 0)
}

fn main() {
    for n in evens() {
        println!("{}", n);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_evens() {
        let v: Vec<_> = super::evens().collect();
        assert_eq!(v, [0, 2, 4, 6, 8]);
    }
}
