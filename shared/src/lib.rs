pub fn f(a: u64) -> u64 {
    ((a.wrapping_mul(12345) as f64).powi(3).log2().round() % (a as f64)) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(f(1), 0);
        assert_eq!(f(5), 3);
        assert_eq!(f(69), 59);
    }
}
