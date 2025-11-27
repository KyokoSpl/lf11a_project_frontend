use crate::gui::models::*;
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

    pub async fn get_departments(&self) -> Result<Vec<Department>, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_departments);
        let resp = self.client.get(&url).send().await?;
        let departments = resp.json::<Vec<Department>>().await?;
        Ok(departments)
    }

    pub async fn create_department(&self, req: &CreateDepartmentRequest) -> Result<(), Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_departments);
        self.client.post(&url).json(req).send().await?;
        Ok(())
    }

    pub async fn get_employees(&self) -> Result<Vec<Employee>, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_employees);
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

    pub async fn get_salary_grades(&self) -> Result<Vec<SalaryGrade>, Box<dyn Error>> {
        let config = Config::get();
        let url = format!("{}{}", config.api_url(), config.route_salary_grades);
        let resp = self.client.get(&url).send().await?;
        let grades = resp.json::<Vec<SalaryGrade>>().await?;
        Ok(grades)
    }
}
