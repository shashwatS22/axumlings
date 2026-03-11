#[derive(Debug)]
struct Wrapper(u32);

impl From<u32> for Wrapper {
    fn from(n: u32) -> Self {
        Wrapper(n)
    }
}

fn main() {
    let w: Wrapper = 42.into();
    println!("{:?}", w.0);
}

#[cfg(test)]
mod tests {
    use super::Wrapper;

    #[test]
    fn test_from() {
        let w: Wrapper = 10.into();
        assert_eq!(w.0, 10);
    }
}
