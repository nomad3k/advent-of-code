#[cfg(tests)]
mod tests {
    use utils::*;

    #[test]
    fn highest_is_three() {
        let x = highest([1,2,3]);
        assert_eq!(x, 2);
    }
}
