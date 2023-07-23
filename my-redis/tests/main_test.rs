use mini_redis::{client, Result};

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_main() -> Result<()> {
        let mut client = client::connect("127.0.0.1:6379").await?;
        client.set("hello", "world".into()).await?;
        let result = client.get("hello").await?;
        assert_eq!(result, Some("world".into()));
        Ok(())
    }
}
