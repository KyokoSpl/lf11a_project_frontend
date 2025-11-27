use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: String,
    pub name: String,
    pub head_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateDepartmentRequest {
    pub name: String,
    pub head_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateDepartmentRequest {
    pub name: Option<String>,
    pub head_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub active: bool,
    pub department_id: Option<String>,
    pub manager_id: Option<String>,
    pub salary_grade_id: Option<String>,
    pub hire_date: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateEmployeeRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department_id: Option<String>,
    pub manager_id: Option<String>,
    pub role: Option<String>,
    pub salary_grade_id: Option<String>,
    pub hire_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateEmployeeRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub active: Option<bool>,
    pub department_id: Option<String>,
    pub manager_id: Option<String>,
    pub salary_grade_id: Option<String>,
    pub hire_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssignManagerRequest {
    pub manager_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssignSalaryGradeRequest {
    pub salary_grade_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SalaryGrade {
    pub id: String,
    pub code: String,
    pub base_salary: f64,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateSalaryGradeRequest {
    pub code: String,
    pub base_salary: f64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSalaryGradeRequest {
    pub code: Option<String>,
    pub base_salary: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_department_serialization() {
        let dept = Department {
            id: "test-123".to_string(),
            name: "Engineering".to_string(),
            head_id: Some("head-456".to_string()),
            created_at: None,
            updated_at: None,
        };

        let json = serde_json::to_string(&dept).unwrap();
        assert!(json.contains("Engineering"));
        assert!(json.contains("test-123"));
    }

    #[test]
    fn test_department_deserialization() {
        let json = r#"{"id":"123","name":"HR","head_id":null,"created_at":null,"updated_at":null}"#;
        let dept: Department = serde_json::from_str(json).unwrap();
        assert_eq!(dept.name, "HR");
        assert_eq!(dept.id, "123");
        assert!(dept.head_id.is_none());
    }

    #[test]
    fn test_create_department_request_default() {
        let req = CreateDepartmentRequest::default();
        assert!(req.name.is_empty());
        assert!(req.head_id.is_none());
    }

    #[test]
    fn test_employee_with_all_fields() {
        let emp = Employee {
            id: "emp-1".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@example.com".to_string(),
            role: "DepartmentHead".to_string(),
            active: true,
            department_id: Some("dept-1".to_string()),
            manager_id: None,
            salary_grade_id: Some("grade-1".to_string()),
            hire_date: Some("2024-01-01".to_string()),
            created_at: None,
            updated_at: None,
            deleted_at: None,
        };

        assert_eq!(emp.role, "DepartmentHead");
        assert!(emp.active);
        assert!(emp.manager_id.is_none());
    }

    #[test]
    fn test_update_employee_request_partial() {
        let req = UpdateEmployeeRequest {
            role: Some("Admin".to_string()),
            ..Default::default()
        };

        assert_eq!(req.role, Some("Admin".to_string()));
        assert!(req.first_name.is_none());
        assert!(req.email.is_none());
    }

    #[test]
    fn test_salary_grade_serialization() {
        let grade = SalaryGrade {
            id: "grade-1".to_string(),
            code: "E4".to_string(),
            base_salary: 80000.0,
            description: Some("Senior Engineer".to_string()),
            created_at: None,
        };

        let json = serde_json::to_string(&grade).unwrap();
        assert!(json.contains("E4"));
        assert!(json.contains("80000"));
    }

    #[test]
    fn test_create_salary_grade_request() {
        let req = CreateSalaryGradeRequest {
            code: "E1".to_string(),
            base_salary: 50000.0,
            description: Some("Junior".to_string()),
        };

        assert_eq!(req.code, "E1");
        assert_eq!(req.base_salary, 50000.0);
    }

    #[test]
    fn test_update_salary_grade_partial() {
        let req = UpdateSalaryGradeRequest {
            base_salary: Some(90000.0),
            ..Default::default()
        };

        assert_eq!(req.base_salary, Some(90000.0));
        assert!(req.code.is_none());
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
    fn test_user_serialization() {
        let user = User {
            id: 1,
            name: "Admin".to_string(),
            email: "admin@example.com".to_string(),
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("Admin"));
        assert!(json.contains("admin@example.com"));
    }
}
