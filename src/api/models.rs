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
