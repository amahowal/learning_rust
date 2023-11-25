pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// This is an attribute they are metadata about pieces of code
#[cfg(test)]
mod tests {

    // This indicates that this function is a test, it is an "annotation"
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
