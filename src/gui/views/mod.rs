//! Views module containing the tab views
//!
//! Each tab in the application has its own submodule:
//! - `departments`: Department management view
//! - `employees`: Employee management view  
//! - `salary_grades`: Salary grade management view
//!
//! Each view is implemented as methods on PersonnelApp via impl blocks.

pub mod departments;
pub mod employees;
pub mod salary_grades;
