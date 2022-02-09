pub fn collatz(n: u64) -> u64 {
    let mut i = n;
    let mut count = 0;
    while i != 1 {
        if i % 2 == 0 {
            i /= 2;
        }
        else {
            i = i * 3 + 1;
        }
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::collatz;

    #[test]
    fn test_20() {
        assert_eq!(collatz(20), 7);
    }

    #[test]
    fn test_1005() {
        assert_eq!(collatz(1005), 67);
    }

    #[test]
    #[ignore]
    fn test_long() {
        assert_eq!(collatz(9780657630), 1132);
    }
}
