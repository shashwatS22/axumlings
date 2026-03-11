// Result combinators: use map_err, ok_or, or and_then.
//
// Hint: res.map_err(|e| e.to_string()).and_then(|v| Ok(v * 2))

fn double_result(res: Result<i32, &'static str>) -> Result<i32, String> {
    // TODO: res.map_err(str::to_string).map(|v| v * 2)
    Err("unimplemented".into())
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
