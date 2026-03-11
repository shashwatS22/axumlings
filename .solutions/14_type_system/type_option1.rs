fn double_opt(opt: Option<i32>) -> Option<i32> {
    opt.map(|x| x * 2)
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
