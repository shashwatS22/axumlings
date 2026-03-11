fn sum_of_squares(v: Vec<i32>) -> i32 {
    v.into_iter().map(|x| x * x).sum()
}

fn main() {
    println!("{}", sum_of_squares(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum_squares() {
        assert_eq!(super::sum_of_squares(vec![1, 2, 3]), 14);
    }
}
