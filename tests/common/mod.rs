// Common test utilities and helpers
use std::env;

/// Set up test environment variables
pub fn setup_test_env() {
    env::set_var("API_BASE_URL", "http://localhost:8082");
    env::set_var("API_PREFIX", "/api");
    env::set_var("LOG_LEVEL", "DEBUG");
    env::set_var("DEBUG_MODE", "true");
}

/// Clean up test environment
pub fn cleanup_test_env() {
    env::remove_var("API_BASE_URL");
    env::remove_var("API_PREFIX");
    env::remove_var("LOG_LEVEL");
    env::remove_var("DEBUG_MODE");
}

/// Mock department data for testing
pub fn mock_department() -> lf11a_project_frontend::api::models::Department {
    lf11a_project_frontend::api::models::Department {
        id: "dept-test-123".to_string(),
        name: "Test Department".to_string(),
        location: "Test Location".to_string(),
        budget: 100000.0,
        head_id: None,
    }
}

/// Mock employee data for testing
pub fn mock_employee() -> lf11a_project_frontend::api::models::Employee {
    lf11a_project_frontend::api::models::Employee {
        id: "emp-test-456".to_string(),
        first_name: "Test".to_string(),
        last_name: "Employee".to_string(),
        email: "test.employee@example.com".to_string(),
        phone: Some("555-0000".to_string()),
        hire_date: "2024-01-01".to_string(),
        salary: 60000.0,
        department_id: Some("dept-test-123".to_string()),
        salary_grade_id: Some("grade-test-789".to_string()),
        role: "Employee".to_string(),
    }
}

/// Mock salary grade data for testing
pub fn mock_salary_grade() -> lf11a_project_frontend::api::models::SalaryGrade {
    lf11a_project_frontend::api::models::SalaryGrade {
        id: "grade-test-789".to_string(),
        grade_name: "Test Grade".to_string(),
        min_salary: 50000.0,
        max_salary: 80000.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_department() {
        let dept = mock_department();
        assert_eq!(dept.name, "Test Department");
    }

    #[test]
    fn test_mock_employee() {
        let emp = mock_employee();
        assert_eq!(emp.first_name, "Test");
        assert_eq!(emp.last_name, "Employee");
    }

    #[test]
    fn test_mock_salary_grade() {
        let grade = mock_salary_grade();
        assert_eq!(grade.grade_name, "Test Grade");
        assert!(grade.min_salary < grade.max_salary);
    }
}
