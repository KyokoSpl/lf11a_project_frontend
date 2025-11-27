// Integration tests for API client
use lf11a_project_frontend::api::client::ApiClient;

#[test]
fn test_api_client_creation() {
    let client = ApiClient::new();
    
    // Client should be created successfully
    // This is a basic smoke test
    drop(client); // Ensure it's droppable
}

#[test]
fn test_api_client_clone() {
    let client1 = ApiClient::new();
    let client2 = client1.clone();
    
    // Both should be valid
    drop(client1);
    drop(client2);
}

#[test]
fn test_api_client_multiple_instances() {
    // Test that we can create multiple client instances
    let client1 = ApiClient::new();
    let client2 = ApiClient::new();
    let client3 = ApiClient::new();
    
    // All should be valid
    drop(client1);
    drop(client2);
    drop(client3);
}

// Note: Testing actual API calls would require a mock server or test backend
// These tests focus on the client structure and basic functionality

#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_client_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ApiClient>();
    }
    
    #[test]
    fn test_client_is_clone() {
        let client = ApiClient::new();
        let _cloned = client.clone();
    }
}
