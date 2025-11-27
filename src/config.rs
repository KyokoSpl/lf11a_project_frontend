use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Config {
    pub api_base_url: String,
    pub api_prefix: String,
    pub route_departments: String,
    pub route_employees: String,
    pub route_salary_grades: String,
    pub log_level: LogLevel,
    pub debug_mode: bool,
    pub app_name: String,
    pub app_version: String,
    pub request_timeout_secs: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            _ => LogLevel::Info,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Load .env file if it exists
        let _ = dotenvy::dotenv();

        Ok(Config {
            api_base_url: std::env::var("API_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8082".to_string()),
            api_prefix: std::env::var("API_PREFIX")
                .unwrap_or_else(|_| "/api".to_string()),
            route_departments: std::env::var("ROUTE_DEPARTMENTS")
                .unwrap_or_else(|_| "/departments".to_string()),
            route_employees: std::env::var("ROUTE_EMPLOYEES")
                .unwrap_or_else(|_| "/employees".to_string()),
            route_salary_grades: std::env::var("ROUTE_SALARY_GRADES")
                .unwrap_or_else(|_| "/salary-grades".to_string()),
            log_level: LogLevel::from_str(
                &std::env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string())
            ),
            debug_mode: std::env::var("DEBUG_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .to_lowercase() == "true",
            app_name: std::env::var("APP_NAME")
                .unwrap_or_else(|_| "LF11A Project Frontend".to_string()),
            app_version: std::env::var("APP_VERSION")
                .unwrap_or_else(|_| env!("CARGO_PKG_VERSION").to_string()),
            request_timeout_secs: std::env::var("REQUEST_TIMEOUT_SECS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            retry_attempts: std::env::var("RETRY_ATTEMPTS")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .unwrap_or(3),
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

    pub fn should_log(&self, level: &LogLevel) -> bool {
        use LogLevel::*;
        let level_priority = |l: &LogLevel| match l {
            Trace => 0,
            Debug => 1,
            Info => 2,
            Warn => 3,
            Error => 4,
        };
        level_priority(level) >= level_priority(&self.log_level)
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
            log_level: LogLevel::Info,
            debug_mode: false,
            app_name: "LF11A Project Frontend".to_string(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            request_timeout_secs: 30,
            retry_attempts: 3,
        }
    }
}

// Convenience macros for logging
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        if $crate::config::Config::get().should_log(&$crate::config::LogLevel::Trace) {
            println!("[TRACE] {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        if $crate::config::Config::get().should_log(&$crate::config::LogLevel::Debug) {
            println!("[DEBUG] {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        if $crate::config::Config::get().should_log(&$crate::config::LogLevel::Info) {
            println!("[INFO] {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        if $crate::config::Config::get().should_log(&$crate::config::LogLevel::Warn) {
            println!("[WARN] {}", format!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        if $crate::config::Config::get().should_log(&$crate::config::LogLevel::Error) {
            eprintln!("[ERROR] {}", format!($($arg)*));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from_str("TRACE"), LogLevel::Trace);
        assert_eq!(LogLevel::from_str("DEBUG"), LogLevel::Debug);
        assert_eq!(LogLevel::from_str("INFO"), LogLevel::Info);
        assert_eq!(LogLevel::from_str("WARN"), LogLevel::Warn);
        assert_eq!(LogLevel::from_str("ERROR"), LogLevel::Error);
        assert_eq!(LogLevel::from_str("invalid"), LogLevel::Info);
    }

    #[test]
    fn test_log_level_case_insensitive() {
        assert_eq!(LogLevel::from_str("trace"), LogLevel::Trace);
        assert_eq!(LogLevel::from_str("DeBuG"), LogLevel::Debug);
        assert_eq!(LogLevel::from_str("WaRn"), LogLevel::Warn);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.api_base_url, "http://localhost:8082");
        assert_eq!(config.api_prefix, "/api");
        assert_eq!(config.log_level, LogLevel::Info);
        assert!(!config.debug_mode);
        assert_eq!(config.request_timeout_secs, 30);
        assert_eq!(config.retry_attempts, 3);
    }

    #[test]
    fn test_config_api_url() {
        let config = Config::default();
        let api_url = config.api_url();
        assert_eq!(api_url, "http://localhost:8082/api");
    }

    #[test]
    fn test_config_should_log() {
        let config = Config {
            log_level: LogLevel::Info,
            ..Config::default()
        };

        assert!(!config.should_log(&LogLevel::Trace));
        assert!(!config.should_log(&LogLevel::Debug));
        assert!(config.should_log(&LogLevel::Info));
        assert!(config.should_log(&LogLevel::Warn));
        assert!(config.should_log(&LogLevel::Error));
    }

    #[test]
    fn test_config_should_log_debug_level() {
        let config = Config {
            log_level: LogLevel::Debug,
            ..Config::default()
        };

        assert!(!config.should_log(&LogLevel::Trace));
        assert!(config.should_log(&LogLevel::Debug));
        assert!(config.should_log(&LogLevel::Info));
    }

    #[test]
    fn test_config_routes() {
        let config = Config::default();
        assert!(config.route_departments.starts_with('/'));
        assert!(config.route_employees.starts_with('/'));
        assert!(config.route_salary_grades.starts_with('/'));
    }

    #[test]
    fn test_config_app_metadata() {
        let config = Config::default();
        assert!(!config.app_name.is_empty());
        assert!(!config.app_version.is_empty());
    }
}
