//! GUI module for the Personnel Management application
//! 
//! This module contains all the UI-related code organized into submodules:
//! - `colors`: Material 3 color palette
//! - `components`: Reusable UI components (buttons, cards)
//! - `views`: Tab views (departments, employees, salary_grades)
//! - `app`: Main application struct and eframe::App implementation

pub mod colors;
pub mod components;
pub mod views;
pub mod app;

// Re-export commonly used types
pub use colors::Material3Colors;
pub use components::{material_button, material_card, styled_text_input, styled_dropdown};
pub use app::PersonnelApp;

/// The tab navigation enum
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tab {
    Departments,
    Employees,
    SalaryGrades,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_equality() {
        assert_eq!(Tab::Departments, Tab::Departments);
        assert_ne!(Tab::Departments, Tab::Employees);
        assert_ne!(Tab::Employees, Tab::SalaryGrades);
    }

    #[test]
    fn test_tab_clone() {
        let tab = Tab::Employees;
        let cloned = tab.clone();
        assert_eq!(tab, cloned);
    }

    #[test]
    fn test_tab_copy() {
        let tab = Tab::SalaryGrades;
        let copied = tab;
        assert_eq!(tab, copied);
    }

    #[test]
    fn test_tab_debug() {
        assert_eq!(format!("{:?}", Tab::Departments), "Departments");
        assert_eq!(format!("{:?}", Tab::Employees), "Employees");
        assert_eq!(format!("{:?}", Tab::SalaryGrades), "SalaryGrades");
    }
}
