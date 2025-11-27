use crate::api::models::*;
use crate::config::Config;
use reqwest::Client;
use std::error::Error;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    // Department endpoints
    pub async fn get_departments(&self) -> Result<Vec<Department>, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_departments);
        let resp = self.client.get(&url).send().await?;
        let departments = resp.json::<Vec<Department>>().await?;
        Ok(departments)
    }

    pub async fn get_department(&self, id: &str) -> Result<Department, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_departments, id);
        let resp = self.client.get(&url).send().await?;
        let department = resp.json::<Department>().await?;
        Ok(department)
    }

    pub async fn create_department(&self, req: &CreateDepartmentRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_departments);
        self.client.post(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn update_department(&self, id: &str, req: &UpdateDepartmentRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_departments, id);
        self.client.put(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn delete_department(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_departments, id);
        self.client.delete(&url).send().await?;
        Ok(())
    }

    // Employee endpoints
    pub async fn get_employees(&self, include_inactive: bool) -> Result<Vec<Employee>, Box<dyn Error>> {
        let config = Config::get();
        let url = if include_inactive {
            format!("{}{}?include_inactive=true", config.api_url(), config.route_employees)
        } else {
            format!("{}{}", config.api_url(), config.route_employees)
        };
        let resp = self.client.get(&url).send().await?;
        let employees = resp.json::<Vec<Employee>>().await?;
        Ok(employees)
    }

    pub async fn get_employee(&self, id: &str) -> Result<Employee, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_employees, id);
        let resp = self.client.get(&url).send().await?;
        let employee = resp.json::<Employee>().await?;
        Ok(employee)
    }

    pub async fn get_employees_by_department(&self, dept_id: &str) -> Result<Vec<Employee>, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}/employees", config.api_url(), config.route_departments, dept_id);
        let resp = self.client.get(&url).send().await?;
        let employees = resp.json::<Vec<Employee>>().await?;
        Ok(employees)
    }

    pub async fn create_employee(&self, req: &CreateEmployeeRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_employees);
        self.client.post(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn update_employee(&self, id: &str, req: &UpdateEmployeeRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_employees, id);
        self.client.put(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn delete_employee(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_employees, id);
        self.client.delete(&url).send().await?;
        Ok(())
    }

    pub async fn assign_manager(&self, id: &str, req: &AssignManagerRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}/manager", config.api_url(), config.route_employees, id);
        self.client.put(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn assign_salary_grade(&self, id: &str, req: &AssignSalaryGradeRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}/salary-grade", config.api_url(), config.route_employees, id);
        self.client.put(&url).json(req).send().await?;
        Ok(())
    }

    // Salary Grade endpoints
    pub async fn get_salary_grades(&self) -> Result<Vec<SalaryGrade>, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_salary_grades);
        let resp = self.client.get(&url).send().await?;
        let grades = resp.json::<Vec<SalaryGrade>>().await?;
        Ok(grades)
    }

    pub async fn get_salary_grade(&self, id: &str) -> Result<SalaryGrade, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_salary_grades, id);
        let resp = self.client.get(&url).send().await?;
        let grade = resp.json::<SalaryGrade>().await?;
        Ok(grade)
    }

    pub async fn create_salary_grade(&self, req: &CreateSalaryGradeRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_salary_grades);
        self.client.post(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn update_salary_grade(&self, id: &str, req: &UpdateSalaryGradeRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_salary_grades, id);
        self.client.put(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn delete_salary_grade(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_salary_grades, id);
        self.client.delete(&url).send().await?;
        Ok(())
    }

    // User endpoints
    pub async fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}/users", config.api_url());
        let resp = self.client.get(&url).send().await?;
        let users = resp.json::<Vec<User>>().await?;
        Ok(users)
    }

    pub async fn get_user(&self, id: i32) -> Result<User, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}/users/{}", config.api_url(), id);
        let resp = self.client.get(&url).send().await?;
        let user = resp.json::<User>().await?;
        Ok(user)
    }

    pub async fn create_user(&self, req: &CreateUserRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}/users", config.api_url());
        self.client.post(&url).json(req).send().await?;
        Ok(())
    }
}
