// Integration tests for API models
use lf11a_project_frontend::api::models::{
    Department, Employee, SalaryGrade,
    CreateDepartmentRequest, UpdateDepartmentRequest,
    CreateEmployeeRequest, UpdateEmployeeRequest,
    CreateSalaryGradeRequest, UpdateSalaryGradeRequest,
};

#[test]
fn test_department_creation() {
    let dept = Department {
        id: "test-id".to_string(),
        name: "Engineering".to_string(),
        head_id: Some("head-123".to_string()),
        created_at: None,
        updated_at: None,
    };
    
    assert_eq!(dept.name, "Engineering");
    assert!(dept.head_id.is_some());
}

#[test]
fn test_department_without_head() {
    let dept = Department {
        id: "test-id".to_string(),
        name: "Marketing".to_string(),
        head_id: None,
        created_at: None,
        updated_at: None,
    };
    
    assert!(dept.head_id.is_none());
}

#[test]
fn test_employee_creation() {
    let employee = Employee {
        id: "emp-123".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        role: "Employee".to_string(),
        active: true,
        department_id: Some("dept-456".to_string()),
        manager_id: None,
        salary_grade_id: Some("grade-789".to_string()),
        hire_date: Some("2024-01-01".to_string()),
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };
    
    assert_eq!(employee.first_name, "John");
    assert_eq!(employee.last_name, "Doe");
    assert_eq!(employee.email, "john.doe@example.com");
    assert_eq!(employee.role, "Employee");
    assert!(employee.active);
}

#[test]
fn test_employee_roles() {
    let roles = vec!["Admin", "DepartmentHead", "DeputyHead", "Employee"];
    
    for role in roles {
        let employee = Employee {
            id: "emp-test".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            email: "test@example.com".to_string(),
            role: role.to_string(),
            active: true,
            department_id: None,
            manager_id: None,
            salary_grade_id: None,
            hire_date: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        };
        
        assert_eq!(employee.role, role);
    }
}

#[test]
fn test_salary_grade_creation() {
    let grade = SalaryGrade {
        id: "grade-123".to_string(),
        code: "E4".to_string(),
        base_salary: 80000.0,
        description: Some("Senior Engineer".to_string()),
        created_at: None,
    };
    
    assert_eq!(grade.code, "E4");
    assert_eq!(grade.base_salary, 80000.0);
}

#[test]
fn test_create_department_request() {
    let request = CreateDepartmentRequest {
        name: "Research".to_string(),
        head_id: None,
    };
    
    assert_eq!(request.name, "Research");
    assert!(request.head_id.is_none());
}

#[test]
fn test_update_department_request_partial() {
    let request = UpdateDepartmentRequest {
        name: Some("Updated Name".to_string()),
        head_id: None,
    };
    
    assert_eq!(request.name, Some("Updated Name".to_string()));
    assert!(request.head_id.is_none());
}

#[test]
fn test_create_employee_request() {
    let request = CreateEmployeeRequest {
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        email: "jane.smith@example.com".to_string(),
        department_id: Some("dept-123".to_string()),
        manager_id: None,
        role: Some("Employee".to_string()),
        salary_grade_id: Some("grade-456".to_string()),
        hire_date: Some("2024-06-01".to_string()),
    };
    
    assert_eq!(request.first_name, "Jane");
    assert_eq!(request.email, "jane.smith@example.com");
}

#[test]
fn test_update_employee_request_role_change() {
    let request = UpdateEmployeeRequest {
        role: Some("DepartmentHead".to_string()),
        ..Default::default()
    };
    
    assert_eq!(request.role, Some("DepartmentHead".to_string()));
    assert!(request.first_name.is_none());
}

#[test]
fn test_create_salary_grade_request() {
    let request = CreateSalaryGradeRequest {
        code: "E1".to_string(),
        base_salary: 50000.0,
        description: Some("Junior".to_string()),
    };

    assert_eq!(request.code, "E1");
    assert_eq!(request.base_salary, 50000.0);
}

#[test]
fn test_update_salary_grade_partial() {
    let request = UpdateSalaryGradeRequest {
        base_salary: Some(90000.0),
        ..Default::default()
    };

    assert_eq!(request.base_salary, Some(90000.0));
    assert!(request.code.is_none());
}

#[test]
fn test_employee_clone() {
    let emp1 = Employee {
        id: "emp-1".to_string(),
        first_name: "Jane".to_string(),
        last_name: "Smith".to_string(),
        email: "jane@example.com".to_string(),
        role: "Employee".to_string(),
        active: true,
        department_id: None,
        manager_id: None,
        salary_grade_id: None,
        hire_date: None,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };

    let emp2 = emp1.clone();
    assert_eq!(emp1.id, emp2.id);
    assert_eq!(emp1.first_name, emp2.first_name);
}

#[test]
fn test_employee_email_format() {
    let employee = Employee {
        id: "emp-test".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        email: "test@example.com".to_string(),
        role: "Employee".to_string(),
        active: true,
        department_id: None,
        manager_id: None,
        salary_grade_id: None,
        hire_date: None,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };
    
    // Basic email validation
    assert!(employee.email.contains('@'));
    assert!(employee.email.contains('.'));
}
