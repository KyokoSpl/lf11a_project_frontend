use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Config {
    pub api_base_url: String,
    pub api_prefix: String,
    pub route_departments: String,
    pub route_employees: String,
    pub route_salary_grades: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let _ = dotenvy::dotenv();

        Ok(Config {
            api_base_url: std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://212.132.110.72:8082".to_string()),
            api_prefix: std::env::var("API_PREFIX")
                .unwrap_or_else(|_| "/api".to_string()),
            route_departments: std::env::var("ROUTE_DEPARTMENTS")
                .unwrap_or_else(|_| "/departments".to_string()),
            route_employees: std::env::var("ROUTE_EMPLOYEES")
                .unwrap_or_else(|_| "/employees".to_string()),
            route_salary_grades: std::env::var("ROUTE_SALARY_GRADES")
                .unwrap_or_else(|_| "/salary-grades".to_string()),
        })
    }

    pub fn get() -> &'static Config {
        CONFIG.get_or_init(|| {
            Config::load().unwrap_or_else(|e| {
                eprintln!("Warning: Failed to load config: {}. Using defaults.", e);
                Config::default()
            })
        })
    }

    pub fn api_url(&self) -> String {
        format!("{}{}", self.api_base_url, self.api_prefix)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_base_url: "http://localhost:8082".to_string(),
            api_prefix: "/api".to_string(),
            route_departments: "/departments".to_string(),
            route_employees: "/employees".to_string(),
            route_salary_grades: "/salary-grades".to_string(),
        }
    }
}
