//! Tests for the egui-based Personnel Management System
//!
//! This module contains unit tests for the GUI components including:
//! - Tab navigation
//! - Material3 color system
//! - App state management
//! - API models
//! - Configuration

use crate::api::models::*;
use crate::config::Config;
use crate::gui::{Material3Colors, Tab};
use egui::{Color32, Vec2};

// ========== Tab Tests ==========

#[test]
fn test_tab_enum_equality() {
    assert_eq!(Tab::Departments, Tab::Departments);
    assert_eq!(Tab::Employees, Tab::Employees);
    assert_eq!(Tab::SalaryGrades, Tab::SalaryGrades);
    assert_ne!(Tab::Departments, Tab::Employees);
    assert_ne!(Tab::Employees, Tab::SalaryGrades);
}

#[test]
fn test_tab_clone() {
    let tab = Tab::Departments;
    let cloned = tab;
    assert_eq!(tab, cloned);
}

#[test]
fn test_tab_copy() {
    let tab1 = Tab::Employees;
    let tab2 = tab1;
    assert_eq!(tab1, tab2);
}

// ========== Material3Colors Tests ==========

#[test]
fn test_material3_colors_default() {
    let colors = Material3Colors::default();

    // Verify primary colors
    assert_eq!(colors.primary, Color32::from_rgb(208, 188, 255));
    assert_eq!(colors.on_primary, Color32::from_rgb(56, 30, 114));

    // Verify surface colors
    assert_eq!(colors.surface, Color32::from_rgb(20, 18, 24));
    assert_eq!(colors.on_surface, Color32::from_rgb(230, 225, 230));

    // Verify error colors
    assert_eq!(colors.error, Color32::from_rgb(242, 184, 181));

    // Verify success color
    assert_eq!(colors.success, Color32::from_rgb(129, 199, 132));
}

#[test]
fn test_material3_colors_clone() {
    let colors = Material3Colors::default();
    let cloned = colors; // Copy trait is implemented

    assert_eq!(colors.primary, cloned.primary);
    assert_eq!(colors.surface, cloned.surface);
    assert_eq!(colors.error, cloned.error);
}

#[test]
fn test_material3_colors_copy() {
    let colors = Material3Colors::default();
    let copied = colors;

    assert_eq!(colors.primary, copied.primary);
    assert_eq!(colors.secondary, copied.secondary);
}

// ========== Department Model Tests ==========

#[test]
fn test_department_creation() {
    let dept = Department {
        id: "dept-1".to_string(),
        name: "Engineering".to_string(),
        head_id: Some("emp-1".to_string()),
        created_at: None,
        updated_at: None,
    };

    assert_eq!(dept.id, "dept-1");
    assert_eq!(dept.name, "Engineering");
    assert_eq!(dept.head_id, Some("emp-1".to_string()));
}

#[test]
fn test_department_without_head() {
    let dept = Department {
        id: "dept-2".to_string(),
        name: "Marketing".to_string(),
        head_id: None,
        created_at: None,
        updated_at: None,
    };

    assert!(dept.head_id.is_none());
}

#[test]
fn test_create_department_request() {
    let req = CreateDepartmentRequest {
        name: "Sales".to_string(),
        head_id: Some("emp-5".to_string()),
    };

    assert_eq!(req.name, "Sales");
    assert_eq!(req.head_id, Some("emp-5".to_string()));
}

#[test]
fn test_create_department_request_default() {
    let req = CreateDepartmentRequest::default();

    assert!(req.name.is_empty());
    assert!(req.head_id.is_none());
}

#[test]
fn test_update_department_request() {
    let req = UpdateDepartmentRequest {
        name: Some("Updated Dept".to_string()),
        head_id: None,
    };

    assert_eq!(req.name, Some("Updated Dept".to_string()));
    assert!(req.head_id.is_none());
}

// ========== Employee Model Tests ==========

#[test]
fn test_employee_creation() {
    let emp = Employee {
        id: "emp-1".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        role: "Developer".to_string(),
        active: true,
        department_id: Some("dept-1".to_string()),
        manager_id: None,
        salary_grade_id: Some("grade-1".to_string()),
        hire_date: None,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };

    assert_eq!(emp.first_name, "John");
    assert_eq!(emp.last_name, "Doe");
    assert!(emp.active);
    assert_eq!(emp.department_id, Some("dept-1".to_string()));
}

#[test]
fn test_employee_full_name() {
    let emp = Employee {
        id: "emp-1".to_string(),
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        email: "jane.smith@example.com".to_string(),
        role: "Manager".to_string(),
        active: true,
        department_id: None,
        manager_id: None,
        salary_grade_id: None,
        hire_date: None,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };

    let full_name = format!("{} {}", emp.first_name, emp.last_name);
    assert_eq!(full_name, "Jane Smith");
}

#[test]
fn test_create_employee_request() {
    let req = CreateEmployeeRequest {
        first_name: "Alice".to_string(),
        last_name: "Johnson".to_string(),
        email: "alice@example.com".to_string(),
        department_id: Some("dept-1".to_string()),
        manager_id: Some("emp-1".to_string()),
        role: Some("Analyst".to_string()),
        salary_grade_id: None,
        hire_date: None,
    };

    assert_eq!(req.first_name, "Alice");
    assert_eq!(req.role, Some("Analyst".to_string()));
}

#[test]
fn test_create_employee_request_default() {
    let req = CreateEmployeeRequest::default();

    assert!(req.first_name.is_empty());
    assert!(req.last_name.is_empty());
    assert!(req.email.is_empty());
    assert!(req.department_id.is_none());
}

#[test]
fn test_update_employee_request() {
    let req = UpdateEmployeeRequest {
        first_name: Some("Updated".to_string()),
        last_name: Some("Name".to_string()),
        email: None,
        role: Some("Senior Developer".to_string()),
        active: Some(true),
        department_id: None,
        manager_id: None,
        salary_grade_id: None,
        hire_date: None,
    };

    assert_eq!(req.first_name, Some("Updated".to_string()));
    assert_eq!(req.role, Some("Senior Developer".to_string()));
    assert!(req.email.is_none());
}

// ========== Salary Grade Model Tests ==========

#[test]
fn test_salary_grade_creation() {
    let grade = SalaryGrade {
        id: "grade-1".to_string(),
        code: "A1".to_string(),
        base_salary: 50000.0,
        description: Some("Entry level".to_string()),
        created_at: None,
    };

    assert_eq!(grade.code, "A1");
    assert_eq!(grade.base_salary, 50000.0);
    assert_eq!(grade.description, Some("Entry level".to_string()));
}

#[test]
fn test_salary_grade_formatting() {
    let grade = SalaryGrade {
        id: "grade-2".to_string(),
        code: "B2".to_string(),
        base_salary: 75000.50,
        description: None,
        created_at: None,
    };

    let formatted = format!("{} (${:.2})", grade.code, grade.base_salary);
    assert_eq!(formatted, "B2 ($75000.50)");
}

#[test]
fn test_create_salary_grade_request() {
    let req = CreateSalaryGradeRequest {
        code: "C3".to_string(),
        base_salary: 100000.0,
        description: Some("Senior level".to_string()),
    };

    assert_eq!(req.code, "C3");
    assert_eq!(req.base_salary, 100000.0);
}

#[test]
fn test_create_salary_grade_request_default() {
    let req = CreateSalaryGradeRequest::default();

    assert!(req.code.is_empty());
    assert_eq!(req.base_salary, 0.0);
    assert!(req.description.is_none());
}

#[test]
fn test_update_salary_grade_request() {
    let req = UpdateSalaryGradeRequest {
        code: Some("D4".to_string()),
        base_salary: Some(120000.0),
        description: Some("Director level".to_string()),
    };

    assert_eq!(req.code, Some("D4".to_string()));
    assert_eq!(req.base_salary, Some(120000.0));
}

// ========== Config Tests ==========

#[test]
fn test_config_default() {
    let config = Config::default();

    assert_eq!(config.api_base_url, "http://localhost:8082");
    assert_eq!(config.api_prefix, "/api");
    assert_eq!(config.route_departments, "/departments");
    assert_eq!(config.route_employees, "/employees");
    assert_eq!(config.route_salary_grades, "/salary-grades");
}

#[test]
fn test_config_api_url() {
    let config = Config::default();
    let api_url = config.api_url();

    assert_eq!(api_url, "http://localhost:8082/api");
}

#[test]
fn test_config_clone() {
    let config = Config::default();
    let cloned = config.clone();

    assert_eq!(config.api_base_url, cloned.api_base_url);
    assert_eq!(config.api_prefix, cloned.api_prefix);
}

// ========== UI State Tests ==========

#[test]
fn test_selected_index_none_by_default() {
    // When no item is selected, Option<usize> should be None
    let selected: Option<usize> = None;
    assert!(selected.is_none());
}

#[test]
fn test_selected_index_some() {
    let selected: Option<usize> = Some(5);
    assert!(selected.is_some());
    assert_eq!(selected, Some(5));
}

// ========== Form Field Tests ==========

#[test]
fn test_empty_string_is_empty() {
    let field = String::new();
    assert!(field.is_empty());
}

#[test]
fn test_non_empty_string() {
    let field = "Department Name".to_string();
    assert!(!field.is_empty());
    assert_eq!(field.len(), 15);
}

#[test]
fn test_string_clear() {
    let mut field = "Some value".to_string();
    field.clear();
    assert!(field.is_empty());
}

// ========== Employee Initials Tests ==========

#[test]
fn test_employee_initials() {
    let first_name = "John";
    let last_name = "Doe";

    let initials = format!(
        "{}{}",
        first_name.chars().next().unwrap_or('?'),
        last_name.chars().next().unwrap_or('?')
    );

    assert_eq!(initials, "JD");
}

#[test]
fn test_employee_initials_empty_names() {
    let first_name = "";
    let last_name = "";

    let initials = format!(
        "{}{}",
        first_name.chars().next().unwrap_or('?'),
        last_name.chars().next().unwrap_or('?')
    );

    assert_eq!(initials, "??");
}

// ========== Department Employee Count Tests ==========

#[test]
fn test_count_employees_in_department() {
    let dept_id = "dept-1".to_string();
    let employees = [
        Employee {
            id: "emp-1".to_string(),
            first_name: "A".to_string(),
            last_name: "A".to_string(),
            email: "a@example.com".to_string(),
            role: "Dev".to_string(),
            active: true,
            department_id: Some("dept-1".to_string()),
            manager_id: None,
            salary_grade_id: None,
            hire_date: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        },
        Employee {
            id: "emp-2".to_string(),
            first_name: "B".to_string(),
            last_name: "B".to_string(),
            email: "b@example.com".to_string(),
            role: "Dev".to_string(),
            active: true,
            department_id: Some("dept-1".to_string()),
            manager_id: None,
            salary_grade_id: None,
            hire_date: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        },
        Employee {
            id: "emp-3".to_string(),
            first_name: "C".to_string(),
            last_name: "C".to_string(),
            email: "c@example.com".to_string(),
            role: "Dev".to_string(),
            active: true,
            department_id: Some("dept-2".to_string()),
            manager_id: None,
            salary_grade_id: None,
            hire_date: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        },
    ];

    let count = employees
        .iter()
        .filter(|e| e.department_id.as_ref() == Some(&dept_id))
        .count();
    assert_eq!(count, 2);
}

// ========== Salary Parsing Tests ==========

#[test]
fn test_salary_parsing_valid() {
    let salary_str = "50000.0";
    let salary: f64 = salary_str.parse().unwrap_or(0.0);
    assert_eq!(salary, 50000.0);
}

#[test]
fn test_salary_parsing_invalid() {
    let salary_str = "invalid";
    let salary: f64 = salary_str.parse().unwrap_or(0.0);
    assert_eq!(salary, 0.0);
}

#[test]
fn test_salary_parsing_empty() {
    let salary_str = "";
    let salary: f64 = salary_str.parse().unwrap_or(0.0);
    assert_eq!(salary, 0.0);
}

// ========== Head Name Resolution Tests ==========

#[test]
fn test_find_department_head() {
    let head_id = Some("emp-1".to_string());
    let employees = [Employee {
        id: "emp-1".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john@example.com".to_string(),
        role: "Manager".to_string(),
        active: true,
        department_id: None,
        manager_id: None,
        salary_grade_id: None,
        hire_date: None,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    }];

    let head_name = head_id
        .as_ref()
        .and_then(|id| employees.iter().find(|e| &e.id == id))
        .map(|e| format!("{} {}", e.first_name, e.last_name))
        .unwrap_or_else(|| "No head assigned".to_string());

    assert_eq!(head_name, "John Doe");
}

#[test]
fn test_find_department_head_not_found() {
    let head_id: Option<String> = None;
    let employees: Vec<Employee> = vec![];

    let head_name = head_id
        .as_ref()
        .and_then(|id| employees.iter().find(|e| &e.id == id))
        .map(|e| format!("{} {}", e.first_name, e.last_name))
        .unwrap_or_else(|| "No head assigned".to_string());

    assert_eq!(head_name, "No head assigned");
}

// ========== Serialization Tests ==========

#[test]
fn test_department_json_serialization() {
    let dept = Department {
        id: "dept-1".to_string(),
        name: "Engineering".to_string(),
        head_id: None,
        created_at: None,
        updated_at: None,
    };

    let json = serde_json::to_string(&dept).unwrap();
    assert!(json.contains("\"id\":\"dept-1\""));
    assert!(json.contains("\"name\":\"Engineering\""));
}

#[test]
fn test_department_json_deserialization() {
    let json = r#"{"id":"dept-1","name":"Engineering","head_id":null,"created_at":null,"updated_at":null}"#;
    let dept: Department = serde_json::from_str(json).unwrap();

    assert_eq!(dept.id, "dept-1");
    assert_eq!(dept.name, "Engineering");
    assert!(dept.head_id.is_none());
}

#[test]
fn test_employee_json_serialization() {
    let emp = Employee {
        id: "emp-1".to_string(),
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        email: "jane@example.com".to_string(),
        role: "Developer".to_string(),
        active: true,
        department_id: None,
        manager_id: None,
        salary_grade_id: None,
        hire_date: None,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };

    let json = serde_json::to_string(&emp).unwrap();
    assert!(json.contains("\"first_name\":\"Jane\""));
    assert!(json.contains("\"active\":true"));
}

#[test]
fn test_salary_grade_json_serialization() {
    let grade = SalaryGrade {
        id: "grade-1".to_string(),
        code: "A1".to_string(),
        base_salary: 50000.0,
        description: Some("Entry level".to_string()),
        created_at: None,
    };

    let json = serde_json::to_string(&grade).unwrap();
    assert!(json.contains("\"code\":\"A1\""));
    assert!(json.contains("\"base_salary\":50000.0"));
}

// ========== Color32 Tests ==========

#[test]
fn test_color32_from_rgb() {
    let color = Color32::from_rgb(255, 128, 64);
    assert_eq!(color.r(), 255);
    assert_eq!(color.g(), 128);
    assert_eq!(color.b(), 64);
}

#[test]
fn test_color32_equality() {
    let c1 = Color32::from_rgb(100, 100, 100);
    let c2 = Color32::from_rgb(100, 100, 100);
    let c3 = Color32::from_rgb(200, 200, 200);

    assert_eq!(c1, c2);
    assert_ne!(c1, c3);
}

// ========== Vec2 Tests ==========

#[test]
fn test_vec2_creation() {
    let v = Vec2::new(100.0, 50.0);
    assert_eq!(v.x, 100.0);
    assert_eq!(v.y, 50.0);
}

// ========== API Client Tests ==========

#[test]
fn test_api_client_creation() {
    use crate::api::client::ApiClient;

    let client = ApiClient::new();
    // Just verify it can be created without panic
    let _ = client;
}

#[test]
fn test_api_client_clone() {
    use crate::api::client::ApiClient;

    let client = ApiClient::new();
    let cloned = client.clone();
    // Both should exist without panic
    let _ = (client, cloned);
}
