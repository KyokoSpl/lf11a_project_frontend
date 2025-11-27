// Integration tests for configuration module
use lf11a_project_frontend::config::Config;

#[test]
fn test_config_initialization() {
    // Test that config can be initialized
    let config = Config::get();
    
    // Config should have valid values
    assert!(!config.api_base_url.is_empty());
    assert!(!config.api_prefix.is_empty());
}

#[test]
fn test_config_api_url() {
    let config = Config::get();
    let api_url = config.api_url();
    
    // API URL should combine base URL and prefix
    assert!(api_url.contains(&config.api_base_url), "API URL should contain base URL");
    assert!(api_url.contains(&config.api_prefix), "API URL should contain API prefix");
}

#[test]
fn test_config_routes() {
    let config = Config::get();
    
    // All routes should be configured
    assert!(!config.route_departments.is_empty());
    assert!(!config.route_employees.is_empty());
    assert!(!config.route_salary_grades.is_empty());
    
    // Routes should start with /
    assert!(config.route_departments.starts_with('/'));
    assert!(config.route_employees.starts_with('/'));
    assert!(config.route_salary_grades.starts_with('/'));
}

#[test]
fn test_config_log_levels() {
    let config = Config::get();
    
    // Log level should be valid - just verify it exists
    // The actual value depends on environment variables
    let level_str = format!("{:?}", config.log_level);
    assert!(!level_str.is_empty());
}

#[test]
fn test_config_app_metadata() {
    let config = Config::get();
    
    // App name and version should be set
    assert!(!config.app_name.is_empty());
    assert!(!config.app_version.is_empty());
}

#[test]
fn test_config_singleton() {
    // Config should return the same instance
    let config1 = Config::get();
    let config2 = Config::get();
    
    // Both should have the same values (singleton pattern)
    assert_eq!(config1.api_base_url, config2.api_base_url);
    assert_eq!(config1.app_name, config2.app_name);
}
