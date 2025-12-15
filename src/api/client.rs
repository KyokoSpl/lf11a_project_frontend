use crate::api::models::*;
use crate::config::Config;
use reqwest::Client;
use std::error::Error;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
}

#[allow(dead_code)]
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

    pub async fn create_department(
        &self,
        req: &CreateDepartmentRequest,
    ) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_departments);
        self.client.post(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn update_department(
        &self,
        id: &str,
        req: &UpdateDepartmentRequest,
    ) -> Result<(), Box<dyn Error>> {
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

    pub async fn get_employees_by_department(
        &self,
        dept_id: &str,
    ) -> Result<Vec<Employee>, Box<dyn Error>> {
        let config = Config::get();
        let url = format!(
            "{}{}/{}/employees",
            config.api_url(),
            config.route_departments,
            dept_id
        );
        let resp = self.client.get(&url).send().await?;
        let employees = resp.json::<Vec<Employee>>().await?;
        Ok(employees)
    }

    // Employee endpoints
    pub async fn get_employees(
        &self,
        include_inactive: bool,
    ) -> Result<Vec<Employee>, Box<dyn Error>> {
        let config = Config::get();
        let url = if include_inactive {
            format!(
                "{}{}?include_inactive=true",
                config.api_url(),
                config.route_employees
            )
        } else {
            format!("{}{}", config.api_url(), config.route_employees)
        };
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

    pub async fn update_employee(
        &self,
        id: &str,
        req: &UpdateEmployeeRequest,
    ) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_employees, id);
        let resp = self.client.put(&url).json(req).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, body).into());
        }
        Ok(())
    }

    pub async fn delete_employee(&self, id: &str) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}/{}", config.api_url(), config.route_employees, id);
        self.client.delete(&url).send().await?;
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

    pub async fn create_salary_grade(
        &self,
        req: &CreateSalaryGradeRequest,
    ) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_salary_grades);
        self.client.post(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn update_salary_grade(
        &self,
        id: &str,
        req: &UpdateSalaryGradeRequest,
    ) -> Result<(), Box<dyn Error>> {
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
}
