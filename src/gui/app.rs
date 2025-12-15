//! Main application struct and eframe::App implementation

use std::sync::{Arc, Mutex};
use egui::{Color32, Frame, Margin, Rounding, Stroke, Vec2};
use crate::api::client::ApiClient;
use crate::api::models::*;
use super::{Tab, Material3Colors, styled_dropdown};

/// Types of confirmation dialogs
#[derive(Clone)]
pub enum ConfirmAction {
    DeleteDepartment { id: String, name: String, employee_count: usize },
    DeleteEmployee { id: String, name: String },
    DeleteSalaryGrade { id: String, code: String, employee_count: usize },
    UpdateDepartment { 
        id: String, 
        name: String, 
        old_head_id: Option<String>,
        old_head_name: Option<String>, 
        new_head_id: Option<String>,
        new_head_name: Option<String> 
    },
    UpdateEmployee { id: String, name: String },
    UpdateSalaryGrade { id: String, code: String },
    CreateDepartment { name: String },
    CreateEmployee { name: String },
    CreateSalaryGrade { code: String },
}

/// Types of edit/create dialogs
#[derive(Clone)]
pub enum EditDialog {
    CreateDepartment,
    EditDepartment { id: String, old_head_id: Option<String> },
    CreateEmployee,
    EditEmployee { id: String },
    CreateSalaryGrade,
    EditSalaryGrade { id: String },
}

/// Main application state for Personnel Management
pub struct PersonnelApp {
    pub runtime: tokio::runtime::Runtime,
    pub api: ApiClient,
    pub colors: Material3Colors,
    pub dark_mode: bool,
    pub current_tab: Tab,
    
    // Confirmation dialog state
    pub confirm_dialog: Option<ConfirmAction>,
    
    // Edit/Create dialog state
    pub edit_dialog: Option<EditDialog>,
    
    // Department state
    pub departments: Arc<Mutex<Vec<Department>>>,
    pub dept_name: String,
    pub dept_head_id: String,
    pub selected_dept: Option<usize>,
    pub dept_loading: bool,
    
    // Employee state
    pub employees: Arc<Mutex<Vec<Employee>>>,
    pub emp_first_name: String,
    pub emp_last_name: String,
    pub emp_email: String,
    pub emp_role: String,
    pub emp_dept_id: String,
    pub emp_manager_id: String,
    pub emp_salary_grade_id: String,
    pub selected_emp: Option<usize>,
    pub emp_loading: bool,
    
    // Salary Grade state
    pub salary_grades: Arc<Mutex<Vec<SalaryGrade>>>,
    pub grade_code: String,
    pub grade_salary: String,
    pub grade_desc: String,
    pub selected_grade: Option<usize>,
    pub grade_loading: bool,
    
    pub error_message: Option<String>,
}

impl PersonnelApp {
    /// Create a new PersonnelApp instance and load initial data
    pub fn new() -> Self {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let api = ApiClient::new();
        
        let departments = Arc::new(Mutex::new(Vec::new()));
        let employees = Arc::new(Mutex::new(Vec::new()));
        let salary_grades = Arc::new(Mutex::new(Vec::new()));
        
        // Load initial data
        let api_clone = api.clone();
        let depts = departments.clone();
        let emps = employees.clone();
        let grades = salary_grades.clone();
        
        runtime.spawn(async move {
            if let Ok(data) = api_clone.get_departments().await {
                *depts.lock().unwrap() = data;
            }
            if let Ok(data) = api_clone.get_employees(false).await {
                *emps.lock().unwrap() = data;
            }
            if let Ok(data) = api_clone.get_salary_grades().await {
                *grades.lock().unwrap() = data;
            }
        });
        
        Self {
            runtime,
            api,
            colors: Material3Colors::dark(),
            dark_mode: true,
            current_tab: Tab::Departments,
            confirm_dialog: None,
            edit_dialog: None,
            departments,
            dept_name: String::new(),
            dept_head_id: String::new(),
            selected_dept: None,
            dept_loading: false,
            employees,
            emp_first_name: String::new(),
            emp_last_name: String::new(),
            emp_email: String::new(),
            emp_role: String::new(),
            emp_dept_id: String::new(),
            emp_manager_id: String::new(),
            emp_salary_grade_id: String::new(),
            selected_emp: None,
            emp_loading: false,
            salary_grades,
            grade_code: String::new(),
            grade_salary: String::new(),
            grade_desc: String::new(),
            selected_grade: None,
            grade_loading: false,
            error_message: None,
        }
    }
    
    /// Refresh departments from the API
    pub fn refresh_departments(&mut self) {
        let api = self.api.clone();
        let depts = self.departments.clone();
        self.runtime.spawn(async move {
            if let Ok(data) = api.get_departments().await {
                *depts.lock().unwrap() = data;
            }
        });
    }
    
    /// Refresh employees from the API
    pub fn refresh_employees(&mut self) {
        let api = self.api.clone();
        let emps = self.employees.clone();
        self.runtime.spawn(async move {
            if let Ok(data) = api.get_employees(false).await {
                *emps.lock().unwrap() = data;
            }
        });
    }
    
    /// Refresh salary grades from the API
    pub fn refresh_salary_grades(&mut self) {
        let api = self.api.clone();
        let grades = self.salary_grades.clone();
        self.runtime.spawn(async move {
            if let Ok(data) = api.get_salary_grades().await {
                *grades.lock().unwrap() = data;
            }
        });
    }
}

impl Default for PersonnelApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for PersonnelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let colors = self.colors;
        
        // Apply global theme styling for popups and menus
        ctx.style_mut(|style| {
            style.visuals.window_fill = colors.surface;
            style.visuals.window_stroke = Stroke::new(1.0, colors.outline_variant);
            style.visuals.window_rounding = Rounding::same(12.0);
            style.visuals.popup_shadow = egui::epaint::Shadow {
                offset: egui::vec2(0.0, 2.0),
                blur: 8.0,
                spread: 0.0,
                color: Color32::from_black_alpha(40),
            };
            style.visuals.menu_rounding = Rounding::same(8.0);
            
            // Selection colors - use contrasting text on primary container
            style.visuals.selection.bg_fill = colors.primary;
            style.visuals.selection.stroke = Stroke::NONE;
            
            // Widget text colors for better contrast
            style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, colors.on_surface);
            style.visuals.widgets.active.fg_stroke = Stroke::new(1.0, colors.on_primary);
        });
        
        // Top navigation bar
        egui::TopBottomPanel::top("top_panel")
            .frame(Frame::none()
                .fill(colors.surface)
                .inner_margin(Margin::symmetric(24.0, 12.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    
                    // Tab buttons with pill style and icons
                    for (tab, icon, label) in [
                        (Tab::Departments, "🏢", "Departments"),
                        (Tab::Employees, "👥", "Employees"),
                        (Tab::SalaryGrades, "💰", "Salary Grades"),
                    ] {
                        let is_selected = self.current_tab == tab;
                        let (bg, text_color) = if is_selected {
                            (colors.primary, colors.on_primary)
                        } else {
                            (Color32::TRANSPARENT, colors.on_surface_variant)
                        };
                        
                        let button = egui::Button::new(
                            egui::RichText::new(format!("{} {}", icon, label))
                                .color(text_color)
                                .size(14.0)
                        )
                        .fill(bg)
                        .stroke(Stroke::NONE)
                        .rounding(Rounding::same(20.0))
                        .min_size(Vec2::new(0.0, 40.0));
                        
                        if ui.add(button).clicked() {
                            self.current_tab = tab;
                        }
                    }
                    
                    // Theme toggle button on the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (theme_icon, theme_tooltip) = if self.dark_mode {
                            ("☀", "Switch to Light Mode")
                        } else {
                            ("🌙", "Switch to Dark Mode")
                        };
                        
                        let theme_btn = egui::Button::new(
                            egui::RichText::new(theme_icon).size(18.0).color(colors.on_surface)
                        )
                        .fill(colors.surface_variant)
                        .stroke(Stroke::new(1.0, colors.outline_variant))
                        .rounding(Rounding::same(20.0))
                        .min_size(Vec2::new(40.0, 40.0));
                        
                        if ui.add(theme_btn).on_hover_text(theme_tooltip).clicked() {
                            self.dark_mode = !self.dark_mode;
                            self.colors = if self.dark_mode {
                                Material3Colors::dark()
                            } else {
                                Material3Colors::light()
                            };
                        }
                    });
                });
            });
        
        // Main content area
        egui::CentralPanel::default()
            .frame(Frame::none()
                .fill(colors.surface)
                .inner_margin(Margin::symmetric(32.0, 24.0)))
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 12.0;
                        
                        match self.current_tab {
                            Tab::Departments => self.show_departments(ui),
                            Tab::Employees => self.show_employees(ui),
                            Tab::SalaryGrades => self.show_salary_grades(ui),
                        }
                        
                        if let Some(ref error) = self.error_message {
                            ui.add_space(16.0);
                            Frame::none()
                                .fill(Color32::from_rgb(50, 30, 30))
                                .rounding(Rounding::same(8.0))
                                .inner_margin(Margin::same(12.0))
                                .show(ui, |ui| {
                                    ui.colored_label(colors.error, format!("⚠ {}", error));
                                });
                        }
                    });
            });
        
        // Render confirmation dialog if active
        self.show_confirm_dialog(ctx);
        
        // Render edit/create dialog if active
        self.show_edit_dialog(ctx);
        
        ctx.request_repaint();
    }
}

impl PersonnelApp {
    /// Show confirmation dialog modal
    fn show_confirm_dialog(&mut self, ctx: &egui::Context) {
        let colors = self.colors;
        
        if let Some(action) = self.confirm_dialog.clone() {
            let (title, message, is_destructive) = match &action {
                ConfirmAction::DeleteDepartment { name, employee_count, .. } => {
                    let msg = if *employee_count > 0 {
                        format!(
                            "Are you sure you want to delete the department \"{}\"?\n\n\
                            ⚠️ Warning: {} employee(s) are assigned to this department.\n\
                            They will be left without a department assignment.",
                            name, employee_count
                        )
                    } else {
                        format!("Are you sure you want to delete the department \"{}\"?", name)
                    };
                    ("🗑 Delete Department", msg, true)
                },
                ConfirmAction::DeleteEmployee { name, .. } => {
                    (
                        "🗑 Delete Employee",
                        format!(
                            "Are you sure you want to delete the employee \"{}\"?\n\n\
                            ⚠️ This action cannot be undone.\n\
                            All associated records will be affected.",
                            name
                        ),
                        true
                    )
                },
                ConfirmAction::DeleteSalaryGrade { code, employee_count, .. } => {
                    let msg = if *employee_count > 0 {
                        format!(
                            "Are you sure you want to delete salary grade \"{}\"?\n\n\
                            ⚠️ Warning: {} employee(s) are using this salary grade.\n\
                            They will be left without a salary grade assignment.",
                            code, employee_count
                        )
                    } else {
                        format!("Are you sure you want to delete salary grade \"{}\"?", code)
                    };
                    ("🗑 Delete Salary Grade", msg, true)
                },
                ConfirmAction::UpdateDepartment { name, old_head_name, new_head_name, .. } => {
                    let head_change = match (old_head_name, new_head_name) {
                        (Some(old), Some(new)) if old != new => format!(
                            "\n\n👔 Head Change:\n\
                            • Previous head \"{}\" will be demoted from department head role\n\
                            • New head \"{}\" will be promoted to department head role",
                            old, new
                        ),
                        (None, Some(new)) => format!(
                            "\n\n👔 Head Assignment:\n\
                            • \"{}\" will be promoted to department head role",
                            new
                        ),
                        (Some(old), None) => format!(
                            "\n\n👔 Head Removal:\n\
                            • \"{}\" will be demoted from department head role",
                            old
                        ),
                        _ => String::new(),
                    };
                    (
                        "✏ Update Department",
                        format!("Update department \"{}\"?{}", name, head_change),
                        false
                    )
                },
                ConfirmAction::UpdateEmployee { name, .. } => {
                    (
                        "✏ Update Employee",
                        format!("Save changes to employee \"{}\"?", name),
                        false
                    )
                },
                ConfirmAction::UpdateSalaryGrade { code, .. } => {
                    (
                        "✏ Update Salary Grade",
                        format!(
                            "Update salary grade \"{}\"?\n\n\
                            ℹ️ All employees with this grade will see the updated values.",
                            code
                        ),
                        false
                    )
                },
                ConfirmAction::CreateDepartment { name } => {
                    (
                        "➕ Create Department",
                        format!("Create new department \"{}\"?", name),
                        false
                    )
                },
                ConfirmAction::CreateEmployee { name } => {
                    (
                        "➕ Create Employee",
                        format!("Create new employee \"{}\"?", name),
                        false
                    )
                },
                ConfirmAction::CreateSalaryGrade { code } => {
                    (
                        "➕ Create Salary Grade",
                        format!("Create new salary grade \"{}\"?", code),
                        false
                    )
                },
            };
            
            egui::Window::new(title)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(Frame::none()
                    .fill(colors.surface_variant)
                    .rounding(Rounding::same(16.0))
                    .stroke(Stroke::new(1.0, colors.outline_variant))
                    .inner_margin(Margin::same(24.0)))
                .show(ctx, |ui| {
                    ui.set_min_width(400.0);
                    
                    ui.label(egui::RichText::new(&message)
                        .size(14.0)
                        .color(colors.on_surface));
                    
                    ui.add_space(24.0);
                    
                    ui.horizontal(|ui| {
                        // Cancel button
                        let cancel_btn = egui::Button::new(
                            egui::RichText::new("Cancel").size(13.0).color(colors.on_surface)
                        )
                        .fill(colors.surface_variant)
                        .stroke(Stroke::new(1.0, colors.outline_variant))
                        .rounding(Rounding::same(8.0))
                        .min_size(Vec2::new(100.0, 40.0));
                        
                        if ui.add(cancel_btn).clicked() {
                            self.confirm_dialog = None;
                        }
                        
                        ui.add_space(12.0);
                        
                        // Confirm button
                        let (confirm_bg, confirm_text) = if is_destructive {
                            (Color32::from_rgb(220, 53, 69), Color32::WHITE)
                        } else {
                            (colors.primary, colors.on_primary)
                        };
                        
                        let confirm_label = if is_destructive { "Delete" } else { "Confirm" };
                        let confirm_btn = egui::Button::new(
                            egui::RichText::new(confirm_label).size(13.0).color(confirm_text)
                        )
                        .fill(confirm_bg)
                        .stroke(Stroke::NONE)
                        .rounding(Rounding::same(8.0))
                        .min_size(Vec2::new(100.0, 40.0));
                        
                        if ui.add(confirm_btn).clicked() {
                            self.execute_confirmed_action(action);
                            self.confirm_dialog = None;
                        }
                    });
                });
        }
    }
    
    /// Execute the confirmed action
    fn execute_confirmed_action(&mut self, action: ConfirmAction) {
        match action {
            ConfirmAction::DeleteDepartment { id, .. } => {
                let api = self.api.clone();
                let depts_ref = self.departments.clone();
                self.runtime.spawn(async move {
                    if let Err(e) = api.delete_department(&id).await {
                        eprintln!("Error deleting department: {}", e);
                    }
                    if let Ok(data) = api.get_departments().await {
                        *depts_ref.lock().unwrap() = data;
                    }
                });
            },
            ConfirmAction::DeleteEmployee { id, .. } => {
                let api = self.api.clone();
                let emps_ref = self.employees.clone();
                self.runtime.spawn(async move {
                    if let Err(e) = api.delete_employee(&id).await {
                        eprintln!("Error deleting employee: {}", e);
                    }
                    if let Ok(data) = api.get_employees(false).await {
                        *emps_ref.lock().unwrap() = data;
                    }
                });
            },
            ConfirmAction::DeleteSalaryGrade { id, .. } => {
                let api = self.api.clone();
                let grades_ref = self.salary_grades.clone();
                self.runtime.spawn(async move {
                    if let Err(e) = api.delete_salary_grade(&id).await {
                        eprintln!("Error deleting salary grade: {}", e);
                    }
                    if let Ok(data) = api.get_salary_grades().await {
                        *grades_ref.lock().unwrap() = data;
                    }
                });
            },
            ConfirmAction::UpdateDepartment { id, old_head_id, new_head_id, .. } => {
                let api = self.api.clone();
                let req = UpdateDepartmentRequest {
                    name: Some(self.dept_name.clone()),
                    head_id: if self.dept_head_id.is_empty() { None } else { Some(self.dept_head_id.clone()) },
                };
                let depts_ref = self.departments.clone();
                let emps_ref = self.employees.clone();
                
                // Clone the head IDs for the async block
                let old_head = old_head_id.clone();
                let new_head = new_head_id.clone();
                
                self.runtime.spawn(async move {
                    // 1. Demote old head if they exist and are different from new head
                    if let Some(ref old_id) = old_head {
                        let should_demote = match &new_head {
                            Some(new_id) => old_id != new_id,
                            None => true,
                        };
                        
                        if should_demote {
                            let demote_req = UpdateEmployeeRequest {
                                role: Some("Employee".to_string()),
                                ..Default::default()
                            };
                            if let Err(e) = api.update_employee(old_id, &demote_req).await {
                                eprintln!("Error demoting old department head: {}", e);
                            }
                        }
                    }
                    
                    // 2. Promote new head if they exist and are different from old head
                    if let Some(ref new_id) = new_head {
                        let should_promote = match &old_head {
                            Some(old_id) => new_id != old_id,
                            None => true,
                        };
                        
                        if should_promote {
                            let promote_req = UpdateEmployeeRequest {
                                role: Some("DepartmentHead".to_string()),
                                ..Default::default()
                            };
                            if let Err(e) = api.update_employee(new_id, &promote_req).await {
                                eprintln!("Error promoting new department head: {}", e);
                            }
                        }
                    }
                    
                    // 3. Update the department
                    if let Err(e) = api.update_department(&id, &req).await {
                        eprintln!("Error updating department: {}", e);
                    }
                    
                    // 4. Refresh data
                    if let Ok(data) = api.get_departments().await {
                        *depts_ref.lock().unwrap() = data;
                    }
                    if let Ok(data) = api.get_employees(false).await {
                        *emps_ref.lock().unwrap() = data;
                    }
                });
                
                self.selected_dept = None;
                self.dept_name.clear();
                self.dept_head_id.clear();
            },
            ConfirmAction::UpdateEmployee { id, .. } => {
                let api = self.api.clone();
                let req = UpdateEmployeeRequest {
                    first_name: Some(self.emp_first_name.clone()),
                    last_name: Some(self.emp_last_name.clone()),
                    email: Some(self.emp_email.clone()),
                    role: Some(self.emp_role.clone()),
                    active: None,
                    department_id: if self.emp_dept_id.is_empty() { None } else { Some(self.emp_dept_id.clone()) },
                    manager_id: if self.emp_manager_id.is_empty() { None } else { Some(self.emp_manager_id.clone()) },
                    salary_grade_id: if self.emp_salary_grade_id.is_empty() { None } else { Some(self.emp_salary_grade_id.clone()) },
                    hire_date: None,
                };
                let emps_ref = self.employees.clone();
                
                self.runtime.spawn(async move {
                    if let Err(e) = api.update_employee(&id, &req).await {
                        eprintln!("Error updating employee: {}", e);
                    }
                    if let Ok(data) = api.get_employees(false).await {
                        *emps_ref.lock().unwrap() = data;
                    }
                });
                
                self.selected_emp = None;
                self.clear_emp_form();
            },
            ConfirmAction::UpdateSalaryGrade { id, .. } => {
                let base_salary = self.grade_salary.parse::<f64>().unwrap_or(0.0);
                let api = self.api.clone();
                let req = UpdateSalaryGradeRequest {
                    code: Some(self.grade_code.clone()),
                    base_salary: Some(base_salary),
                    description: if self.grade_desc.is_empty() { None } else { Some(self.grade_desc.clone()) },
                };
                let grades_ref = self.salary_grades.clone();
                
                self.runtime.spawn(async move {
                    if let Err(e) = api.update_salary_grade(&id, &req).await {
                        eprintln!("Error updating salary grade: {}", e);
                    }
                    if let Ok(data) = api.get_salary_grades().await {
                        *grades_ref.lock().unwrap() = data;
                    }
                });
                
                self.selected_grade = None;
                self.clear_grade_form();
            },
            ConfirmAction::CreateDepartment { .. } => {
                let api = self.api.clone();
                let req = CreateDepartmentRequest {
                    name: self.dept_name.clone(),
                    head_id: if self.dept_head_id.is_empty() { None } else { Some(self.dept_head_id.clone()) },
                };
                let depts_ref = self.departments.clone();
                
                self.runtime.spawn(async move {
                    if let Err(e) = api.create_department(&req).await {
                        eprintln!("Error creating department: {}", e);
                    }
                    if let Ok(data) = api.get_departments().await {
                        *depts_ref.lock().unwrap() = data;
                    }
                });
                
                self.dept_name.clear();
                self.dept_head_id.clear();
            },
            ConfirmAction::CreateEmployee { .. } => {
                let api = self.api.clone();
                let req = CreateEmployeeRequest {
                    first_name: self.emp_first_name.clone(),
                    last_name: self.emp_last_name.clone(),
                    email: self.emp_email.clone(),
                    department_id: if self.emp_dept_id.is_empty() { None } else { Some(self.emp_dept_id.clone()) },
                    manager_id: if self.emp_manager_id.is_empty() { None } else { Some(self.emp_manager_id.clone()) },
                    role: Some(self.emp_role.clone()),
                    salary_grade_id: if self.emp_salary_grade_id.is_empty() { None } else { Some(self.emp_salary_grade_id.clone()) },
                    hire_date: None,
                };
                let emps_ref = self.employees.clone();
                
                self.runtime.spawn(async move {
                    if let Err(e) = api.create_employee(&req).await {
                        eprintln!("Error creating employee: {}", e);
                    }
                    if let Ok(data) = api.get_employees(false).await {
                        *emps_ref.lock().unwrap() = data;
                    }
                });
                
                self.clear_emp_form();
            },
            ConfirmAction::CreateSalaryGrade { .. } => {
                let base_salary = self.grade_salary.parse::<f64>().unwrap_or(0.0);
                let api = self.api.clone();
                let req = CreateSalaryGradeRequest {
                    code: self.grade_code.clone(),
                    base_salary,
                    description: if self.grade_desc.is_empty() { None } else { Some(self.grade_desc.clone()) },
                };
                let grades_ref = self.salary_grades.clone();
                
                self.runtime.spawn(async move {
                    if let Err(e) = api.create_salary_grade(&req).await {
                        eprintln!("Error creating salary grade: {}", e);
                    }
                    if let Ok(data) = api.get_salary_grades().await {
                        *grades_ref.lock().unwrap() = data;
                    }
                });
                
                self.clear_grade_form();
            },
        }
    }
    
    /// Show edit/create dialog modal
    fn show_edit_dialog(&mut self, ctx: &egui::Context) {
        let colors = self.colors;
        
        if let Some(dialog) = self.edit_dialog.clone() {
            let title = match &dialog {
                EditDialog::CreateDepartment => "➕ Create Department",
                EditDialog::EditDepartment { .. } => "✏️ Edit Department",
                EditDialog::CreateEmployee => "➕ Create Employee",
                EditDialog::EditEmployee { .. } => "✏️ Edit Employee",
                EditDialog::CreateSalaryGrade => "➕ Create Salary Grade",
                EditDialog::EditSalaryGrade { .. } => "✏️ Edit Salary Grade",
            };
            
            egui::Window::new(title)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(Frame::none()
                    .fill(colors.surface_variant)
                    .rounding(Rounding::same(16.0))
                    .stroke(Stroke::new(1.0, colors.outline_variant))
                    .inner_margin(Margin::same(28.0)))
                .show(ctx, |ui| {
                    ui.set_min_width(480.0);
                    
                    match &dialog {
                        EditDialog::CreateDepartment | EditDialog::EditDepartment { .. } => {
                            self.show_department_form(ui, &dialog);
                        },
                        EditDialog::CreateEmployee | EditDialog::EditEmployee { .. } => {
                            self.show_employee_form(ui, &dialog);
                        },
                        EditDialog::CreateSalaryGrade | EditDialog::EditSalaryGrade { .. } => {
                            self.show_salary_grade_form(ui, &dialog);
                        },
                    }
                });
        }
    }
    
    /// Show department form in dialog
    fn show_department_form(&mut self, ui: &mut egui::Ui, dialog: &EditDialog) {
        let colors = self.colors;
        let emps = self.employees.lock().unwrap().clone();
        
        ui.vertical(|ui| {
            // Department Name
            ui.label(egui::RichText::new("Department Name").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.dept_name)
                        .desired_width(405.0)
                        .hint_text(egui::RichText::new("Enter department name...").color(colors.on_surface_variant))
                        .text_color(colors.on_surface)
                        .frame(false));
                });
            
            ui.add_space(16.0);
            
            // Department Head dropdown
            ui.label(egui::RichText::new("Department Head (optional)").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            
            let current_head_name = if self.dept_head_id.is_empty() {
                "No head assigned".to_string()
            } else {
                emps.iter()
                    .find(|e| e.id == self.dept_head_id)
                    .map(|e| format!("{} {} ({})", e.first_name, e.last_name, e.role))
                    .unwrap_or_else(|| "Select employee...".to_string())
            };
            
            styled_dropdown(ui, &colors, |ui| {
                egui::ComboBox::from_id_salt("dept_head_dialog_dropdown")
                    .selected_text(egui::RichText::new(&current_head_name).color(colors.on_surface))
                    .width(430.0)
                    .height(300.0)
                    .show_ui(ui, |ui| {
                        ui.set_min_width(405.0);
                        let is_none_selected = self.dept_head_id.is_empty();
                        let none_text = egui::RichText::new("❌ No head assigned")
                            .color(if is_none_selected { colors.on_primary } else { colors.on_surface });
                        if ui.selectable_label(is_none_selected, none_text).clicked() {
                            self.dept_head_id.clear();
                        }
                        ui.separator();
                        for emp in &emps {
                            let is_selected = self.dept_head_id == emp.id;
                            let label = format!("👤 {} {} - {}", emp.first_name, emp.last_name, emp.role);
                            let label_text = egui::RichText::new(label)
                                .color(if is_selected { colors.on_primary } else { colors.on_surface });
                            if ui.selectable_label(is_selected, label_text).clicked() {
                                self.dept_head_id = emp.id.clone();
                            }
                        }
                    })
            });
            
            ui.add_space(24.0);
            
            // Buttons
            ui.horizontal(|ui| {
                let cancel_btn = egui::Button::new(
                    egui::RichText::new("Cancel").size(13.0).color(colors.on_surface)
                )
                .fill(colors.surface_variant)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(100.0, 40.0));
                
                if ui.add(cancel_btn).clicked() {
                    self.edit_dialog = None;
                    self.dept_name.clear();
                    self.dept_head_id.clear();
                }
                
                ui.add_space(12.0);
                
                let (action_label, is_edit) = match dialog {
                    EditDialog::EditDepartment { .. } => ("Update", true),
                    _ => ("Create", false),
                };
                
                let action_btn = egui::Button::new(
                    egui::RichText::new(action_label).size(13.0).color(colors.on_primary)
                )
                .fill(colors.primary)
                .stroke(Stroke::NONE)
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(100.0, 40.0));
                
                if ui.add(action_btn).clicked() && !self.dept_name.is_empty() {
                    if is_edit {
                        if let EditDialog::EditDepartment { id, old_head_id } = dialog {
                            let old_head_name = old_head_id.as_ref()
                                .and_then(|hid| emps.iter().find(|e| &e.id == hid))
                                .map(|e| format!("{} {}", e.first_name, e.last_name));
                            let new_head_id = if self.dept_head_id.is_empty() { None } else { Some(self.dept_head_id.clone()) };
                            let new_head_name = if self.dept_head_id.is_empty() { None } else {
                                emps.iter().find(|e| e.id == self.dept_head_id)
                                    .map(|e| format!("{} {}", e.first_name, e.last_name))
                            };
                            
                            self.confirm_dialog = Some(ConfirmAction::UpdateDepartment {
                                id: id.clone(),
                                name: self.dept_name.clone(),
                                old_head_id: old_head_id.clone(),
                                old_head_name,
                                new_head_id,
                                new_head_name,
                            });
                            self.edit_dialog = None;
                        }
                    } else {
                        self.confirm_dialog = Some(ConfirmAction::CreateDepartment {
                            name: self.dept_name.clone(),
                        });
                        self.edit_dialog = None;
                    }
                }
            });
        });
    }
    
    /// Show employee form in dialog
    fn show_employee_form(&mut self, ui: &mut egui::Ui, dialog: &EditDialog) {
        let colors = self.colors;
        let depts = self.departments.lock().unwrap().clone();
        let emps = self.employees.lock().unwrap().clone();
        let grades = self.salary_grades.lock().unwrap().clone();
        
        ui.vertical(|ui| {
            // Row 1: First Name, Last Name
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("First Name").size(12.0).color(colors.on_surface_variant));
                    ui.add_space(4.0);
                    Frame::none()
                        .fill(colors.surface)
                        .stroke(Stroke::new(1.0, colors.outline_variant))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(12.0, 10.0))
                        .show(ui, |ui| {
                            ui.add(egui::TextEdit::singleline(&mut self.emp_first_name)
                                .desired_width(195.0)
                                .hint_text(egui::RichText::new("First name...").color(colors.on_surface_variant))
                                .text_color(colors.on_surface)
                                .frame(false));
                        });
                });
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Last Name").size(12.0).color(colors.on_surface_variant));
                    ui.add_space(4.0);
                    Frame::none()
                        .fill(colors.surface)
                        .stroke(Stroke::new(1.0, colors.outline_variant))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(12.0, 10.0))
                        .show(ui, |ui| {
                            ui.add(egui::TextEdit::singleline(&mut self.emp_last_name)
                                .desired_width(195.0)
                                .hint_text(egui::RichText::new("Last name...").color(colors.on_surface_variant))
                                .text_color(colors.on_surface)
                                .frame(false));
                        });
                });
            });
            
            ui.add_space(12.0);
            
            // Email
            ui.label(egui::RichText::new("Email").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.emp_email)
                        .desired_width(405.0)
                        .hint_text(egui::RichText::new("email@example.com").color(colors.on_surface_variant))
                        .text_color(colors.on_surface)
                        .frame(false));
                });
            
            ui.add_space(12.0);
            
            // Role dropdown
            ui.label(egui::RichText::new("Role").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            let role_display = if self.emp_role.is_empty() { "Select role...".to_string() } else { self.emp_role.clone() };
            styled_dropdown(ui, &colors, |ui| {
                egui::ComboBox::from_id_salt("emp_role_dialog_dropdown")
                    .selected_text(egui::RichText::new(&role_display).color(colors.on_surface))
                    .width(430.0)
                    .height(300.0)
                    .show_ui(ui, |ui| {
                        ui.set_min_width(405.0);
                        for role in ["Employee", "DepartmentHead", "DeputyHead", "Admin"] {
                            let is_selected = self.emp_role == role;
                            let label_text = egui::RichText::new(role)
                                .color(if is_selected { colors.on_primary } else { colors.on_surface });
                            if ui.selectable_label(is_selected, label_text).clicked() {
                                self.emp_role = role.to_string();
                            }
                        }
                    })
            });
            
            ui.add_space(12.0);
            
            // Department dropdown
            ui.label(egui::RichText::new("Department").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            let dept_name = if self.emp_dept_id.is_empty() {
                "No department".to_string()
            } else {
                depts.iter().find(|d| d.id == self.emp_dept_id)
                    .map(|d| d.name.clone())
                    .unwrap_or_else(|| "Select...".to_string())
            };
            styled_dropdown(ui, &colors, |ui| {
                egui::ComboBox::from_id_salt("emp_dept_dialog_dropdown")
                    .selected_text(egui::RichText::new(&dept_name).color(colors.on_surface))
                    .width(430.0)
                    .height(300.0)
                    .show_ui(ui, |ui| {
                        ui.set_min_width(405.0);
                        let is_none_selected = self.emp_dept_id.is_empty();
                        let none_text = egui::RichText::new("❌ No department")
                            .color(if is_none_selected { colors.on_primary } else { colors.on_surface });
                        if ui.selectable_label(is_none_selected, none_text).clicked() {
                            self.emp_dept_id.clear();
                        }
                        ui.separator();
                        for dept in &depts {
                            let is_selected = self.emp_dept_id == dept.id;
                            let label_text = egui::RichText::new(&dept.name)
                                .color(if is_selected { colors.on_primary } else { colors.on_surface });
                            if ui.selectable_label(is_selected, label_text).clicked() {
                                self.emp_dept_id = dept.id.clone();
                            }
                        }
                    })
            });
            
            ui.add_space(12.0);
            
            // Manager dropdown
            ui.label(egui::RichText::new("Manager").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            let manager_name = if self.emp_manager_id.is_empty() {
                "No manager".to_string()
            } else {
                emps.iter().find(|e| e.id == self.emp_manager_id)
                    .map(|e| format!("{} {}", e.first_name, e.last_name))
                    .unwrap_or_else(|| "Select...".to_string())
            };
            styled_dropdown(ui, &colors, |ui| {
                egui::ComboBox::from_id_salt("emp_manager_dialog_dropdown")
                    .selected_text(egui::RichText::new(&manager_name).color(colors.on_surface))
                    .width(430.0)
                    .height(300.0)
                    .show_ui(ui, |ui| {
                        ui.set_min_width(405.0);
                        let is_none_selected = self.emp_manager_id.is_empty();
                        let none_text = egui::RichText::new("❌ No manager")
                            .color(if is_none_selected { colors.on_primary } else { colors.on_surface });
                        if ui.selectable_label(is_none_selected, none_text).clicked() {
                            self.emp_manager_id.clear();
                        }
                        ui.separator();
                        for emp in &emps {
                            let is_selected = self.emp_manager_id == emp.id;
                            let label = format!("{} {} - {}", emp.first_name, emp.last_name, emp.role);
                            let label_text = egui::RichText::new(label)
                                .color(if is_selected { colors.on_primary } else { colors.on_surface });
                            if ui.selectable_label(is_selected, label_text).clicked() {
                                self.emp_manager_id = emp.id.clone();
                            }
                        }
                    })
            });
            
            ui.add_space(12.0);
            
            // Salary Grade dropdown
            ui.label(egui::RichText::new("Salary Grade").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            let grade_name = if self.emp_salary_grade_id.is_empty() {
                "No salary grade".to_string()
            } else {
                grades.iter().find(|g| g.id == self.emp_salary_grade_id)
                    .map(|g| format!("{} - ${:.2}", g.code, g.base_salary))
                    .unwrap_or_else(|| "Select...".to_string())
            };
            styled_dropdown(ui, &colors, |ui| {
                egui::ComboBox::from_id_salt("emp_grade_dialog_dropdown")
                    .selected_text(egui::RichText::new(&grade_name).color(colors.on_surface))
                    .width(430.0)
                    .height(300.0)
                    .show_ui(ui, |ui| {
                        ui.set_min_width(405.0);
                        let is_none_selected = self.emp_salary_grade_id.is_empty();
                        let none_text = egui::RichText::new("❌ No salary grade")
                            .color(if is_none_selected { colors.on_primary } else { colors.on_surface });
                        if ui.selectable_label(is_none_selected, none_text).clicked() {
                            self.emp_salary_grade_id.clear();
                        }
                        ui.separator();
                        for grade in &grades {
                            let is_selected = self.emp_salary_grade_id == grade.id;
                            let label = format!("{} - ${:.2}", grade.code, grade.base_salary);
                            let label_text = egui::RichText::new(label)
                                .color(if is_selected { colors.on_primary } else { colors.on_surface });
                            if ui.selectable_label(is_selected, label_text).clicked() {
                                self.emp_salary_grade_id = grade.id.clone();
                            }
                        }
                    })
            });
            
            ui.add_space(24.0);
            
            // Buttons
            ui.horizontal(|ui| {
                let cancel_btn = egui::Button::new(
                    egui::RichText::new("Cancel").size(13.0).color(colors.on_surface)
                )
                .fill(colors.surface_variant)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(100.0, 40.0));
                
                if ui.add(cancel_btn).clicked() {
                    self.edit_dialog = None;
                    self.clear_emp_form();
                }
                
                ui.add_space(12.0);
                
                let (action_label, is_edit) = match dialog {
                    EditDialog::EditEmployee { .. } => ("Update", true),
                    _ => ("Create", false),
                };
                
                let action_btn = egui::Button::new(
                    egui::RichText::new(action_label).size(13.0).color(colors.on_primary)
                )
                .fill(colors.primary)
                .stroke(Stroke::NONE)
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(100.0, 40.0));
                
                if ui.add(action_btn).clicked() && !self.emp_first_name.is_empty() && !self.emp_last_name.is_empty() {
                    let name = format!("{} {}", self.emp_first_name, self.emp_last_name);
                    if is_edit {
                        if let EditDialog::EditEmployee { id } = dialog {
                            self.confirm_dialog = Some(ConfirmAction::UpdateEmployee {
                                id: id.clone(),
                                name,
                            });
                            self.edit_dialog = None;
                        }
                    } else {
                        self.confirm_dialog = Some(ConfirmAction::CreateEmployee { name });
                        self.edit_dialog = None;
                    }
                }
            });
        });
    }
    
    /// Show salary grade form in dialog
    fn show_salary_grade_form(&mut self, ui: &mut egui::Ui, dialog: &EditDialog) {
        let colors = self.colors;
        
        ui.vertical(|ui| {
            // Grade Code
            ui.label(egui::RichText::new("Grade Code").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.grade_code)
                        .desired_width(405.0)
                        .hint_text(egui::RichText::new("e.g. A1, B2, C3...").color(colors.on_surface_variant))
                        .text_color(colors.on_surface)
                        .frame(false));
                });
            
            ui.add_space(12.0);
            
            // Base Salary
            ui.label(egui::RichText::new("Base Salary").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.grade_salary)
                        .desired_width(405.0)
                        .hint_text(egui::RichText::new("Enter amount (e.g. 50000)").color(colors.on_surface_variant))
                        .text_color(colors.on_surface)
                        .frame(false));
                });
            
            ui.add_space(12.0);
            
            // Description
            ui.label(egui::RichText::new("Description (optional)").size(12.0).color(colors.on_surface_variant));
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.grade_desc)
                        .desired_width(405.0)
                        .hint_text(egui::RichText::new("Description...").color(colors.on_surface_variant))
                        .text_color(colors.on_surface)
                        .frame(false));
                });
            
            ui.add_space(24.0);
            
            // Buttons
            ui.horizontal(|ui| {
                let cancel_btn = egui::Button::new(
                    egui::RichText::new("Cancel").size(13.0).color(colors.on_surface)
                )
                .fill(colors.surface_variant)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(100.0, 40.0));
                
                if ui.add(cancel_btn).clicked() {
                    self.edit_dialog = None;
                    self.clear_grade_form();
                }
                
                ui.add_space(12.0);
                
                let (action_label, is_edit) = match dialog {
                    EditDialog::EditSalaryGrade { .. } => ("Update", true),
                    _ => ("Create", false),
                };
                
                let action_btn = egui::Button::new(
                    egui::RichText::new(action_label).size(13.0).color(colors.on_primary)
                )
                .fill(colors.primary)
                .stroke(Stroke::NONE)
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(100.0, 40.0));
                
                if ui.add(action_btn).clicked() && !self.grade_code.is_empty() {
                    if is_edit {
                        if let EditDialog::EditSalaryGrade { id } = dialog {
                            self.confirm_dialog = Some(ConfirmAction::UpdateSalaryGrade {
                                id: id.clone(),
                                code: self.grade_code.clone(),
                            });
                            self.edit_dialog = None;
                        }
                    } else {
                        self.confirm_dialog = Some(ConfirmAction::CreateSalaryGrade {
                            code: self.grade_code.clone(),
                        });
                        self.edit_dialog = None;
                    }
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_new() {
        let app = PersonnelApp::new();
        assert_eq!(app.current_tab, Tab::Departments);
        assert!(app.dept_name.is_empty());
        assert!(app.employees.lock().unwrap().is_empty());
    }
    
    #[test]
    fn test_app_default() {
        let app = PersonnelApp::default();
        assert_eq!(app.current_tab, Tab::Departments);
    }
}
