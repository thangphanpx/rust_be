#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}