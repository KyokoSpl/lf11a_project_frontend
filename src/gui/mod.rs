//! GUI module for the Personnel Management application
//!
//! This module contains all the UI-related code organized into submodules:
//! - `colors`: Material 3 color palette
//! - `components`: Reusable UI components (buttons, cards, dropdowns)
//! - `dialogs`: Dialog type definitions (ConfirmAction, EditDialog)
//! - `dialog_handlers`: Dialog handling logic (confirmations, action execution)
//! - `forms`: Form UI components for create/edit dialogs
//! - `views`: Tab views (departments, employees, salary_grades)
//! - `app`: Main application struct and eframe::App implementation

pub mod app;
pub mod colors;
pub mod components;
pub mod dialog_handlers;
pub mod dialogs;
pub mod forms;
pub mod views;

// Re-export commonly used types
pub use app::PersonnelApp;
pub use colors::Material3Colors;
pub use components::{material_button, styled_dropdown};
pub use dialogs::{ConfirmAction, EditDialog};

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
        let cloned = tab; // Copy trait is implemented
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
