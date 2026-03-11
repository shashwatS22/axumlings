// Iterator adapters: use map, filter, collect, or fold.
//
// Hint: (0..10).filter(|x| x % 2 == 0).map(|x| x * 2).collect::<Vec<_>>()

fn sum_of_squares(v: Vec<i32>) -> i32 {
    // TODO: v.into_iter().map(|x| x * x).sum()
    0
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
