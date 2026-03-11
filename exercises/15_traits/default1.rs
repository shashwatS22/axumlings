// Default derive: use #[derive(Default)] for a struct.
//
// Hint: #[derive(Default)] gives zero-like defaults (0, false, "", etc.)

// TODO: Add #[derive(Default)]
#[derive(Default)]
struct Config {
    timeout: u64,
    retries: u32,
}

fn main() {
    let cfg = Config::default();
    println!("{} {}", cfg.timeout, cfg.retries);
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_default() {
        let cfg = Config::default();
        assert_eq!(cfg.timeout, 0);
        assert_eq!(cfg.retries, 0);
    }
}
