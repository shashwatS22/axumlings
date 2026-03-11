fn double_result(res: Result<i32, &'static str>) -> Result<i32, String> {
    res.map_err(str::to_string).map(|v| v * 2)
}

fn main() {
    println!("{:?}", double_result(Ok(5)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_double() {
        assert_eq!(super::double_result(Ok(5)), Ok(10));
    }
}
