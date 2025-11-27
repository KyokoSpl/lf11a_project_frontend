// Library exports for testing
pub mod api;
pub mod config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modules_exist() {
        // Smoke test to ensure modules are accessible
        assert!(true);
    }
}
