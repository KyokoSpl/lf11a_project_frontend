//! Dialog handlers for confirmation and edit dialogs

use super::dialogs::{ConfirmAction, EditDialog};
use super::PersonnelApp;
use crate::api::models::*;
use egui::{Color32, Context, Frame, Margin, Rounding, Stroke, Vec2};

impl PersonnelApp {
    /// Show confirmation dialog modal
    pub fn show_confirm_dialog(&mut self, ctx: &Context) {
        let colors = self.colors;

        if let Some(action) = self.confirm_dialog.clone() {
            let (title, message, is_destructive) = match &action {
                ConfirmAction::DeleteDepartment {
                    name,
                    employee_count,
                    ..
                } => {
                    let msg = if *employee_count > 0 {
                        format!(
                            "Are you sure you want to delete the department \"{}\"?\n\n\
                            ⚠️ Warning: {} employee(s) are assigned to this department.\n\
                            They will be left without a department assignment.",
                            name, employee_count
                        )
                    } else {
                        format!(
                            "Are you sure you want to delete the department \"{}\"?",
                            name
                        )
                    };
                    ("🗑 Delete Department", msg, true)
                }
                ConfirmAction::DeleteEmployee { name, .. } => (
                    "🗑 Delete Employee",
                    format!(
                        "Are you sure you want to delete the employee \"{}\"?\n\n\
                            ⚠️ This action cannot be undone.\n\
                            All associated records will be affected.",
                        name
                    ),
                    true,
                ),
                ConfirmAction::DeleteSalaryGrade {
                    code,
                    employee_count,
                    ..
                } => {
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
                }
                ConfirmAction::UpdateDepartment {
                    name,
                    old_head_name,
                    new_head_name,
                    ..
                } => {
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
                        false,
                    )
                }
                ConfirmAction::UpdateEmployee { name, .. } => (
                    "✏ Update Employee",
                    format!("Save changes to employee \"{}\"?", name),
                    false,
                ),
                ConfirmAction::UpdateSalaryGrade { code, .. } => (
                    "✏ Update Salary Grade",
                    format!(
                        "Update salary grade \"{}\"?\n\n\
                            ℹ️ All employees with this grade will see the updated values.",
                        code
                    ),
                    false,
                ),
                ConfirmAction::CreateDepartment { name } => (
                    "➕ Create Department",
                    format!("Create new department \"{}\"?", name),
                    false,
                ),
                ConfirmAction::CreateEmployee { name } => (
                    "➕ Create Employee",
                    format!("Create new employee \"{}\"?", name),
                    false,
                ),
                ConfirmAction::CreateSalaryGrade { code } => (
                    "➕ Create Salary Grade",
                    format!("Create new salary grade \"{}\"?", code),
                    false,
                ),
            };

            egui::Window::new(title)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .frame(
                    Frame::none()
                        .fill(colors.surface_variant)
                        .rounding(Rounding::same(16.0))
                        .stroke(Stroke::new(1.0, colors.outline_variant))
                        .inner_margin(Margin::same(24.0)),
                )
                .show(ctx, |ui| {
                    ui.set_min_width(400.0);

                    ui.label(
                        egui::RichText::new(&message)
                            .size(14.0)
                            .color(colors.on_surface),
                    );

                    ui.add_space(24.0);

                    ui.horizontal(|ui| {
                        // Cancel button
                        let cancel_btn = egui::Button::new(
                            egui::RichText::new("Cancel")
                                .size(13.0)
                                .color(colors.on_surface),
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
                            egui::RichText::new(confirm_label)
                                .size(13.0)
                                .color(confirm_text),
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
    pub fn execute_confirmed_action(&mut self, action: ConfirmAction) {
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
            }
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
            }
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
            }
            ConfirmAction::UpdateDepartment {
                id,
                old_head_id,
                new_head_id,
                ..
            } => {
                let api = self.api.clone();
                let req = UpdateDepartmentRequest {
                    name: Some(self.dept_name.clone()),
                    head_id: if self.dept_head_id.is_empty() {
                        None
                    } else {
                        Some(self.dept_head_id.clone())
                    },
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
            }
            ConfirmAction::UpdateEmployee { id, .. } => {
                let api = self.api.clone();
                let req = UpdateEmployeeRequest {
                    first_name: Some(self.emp_first_name.clone()),
                    last_name: Some(self.emp_last_name.clone()),
                    email: Some(self.emp_email.clone()),
                    role: Some(self.emp_role.clone()),
                    active: None,
                    department_id: if self.emp_dept_id.is_empty() {
                        None
                    } else {
                        Some(self.emp_dept_id.clone())
                    },
                    manager_id: if self.emp_manager_id.is_empty() {
                        None
                    } else {
                        Some(self.emp_manager_id.clone())
                    },
                    salary_grade_id: if self.emp_salary_grade_id.is_empty() {
                        None
                    } else {
                        Some(self.emp_salary_grade_id.clone())
                    },
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
            }
            ConfirmAction::UpdateSalaryGrade { id, .. } => {
                let base_salary = self.grade_salary.parse::<f64>().unwrap_or(0.0);
                let api = self.api.clone();
                let req = UpdateSalaryGradeRequest {
                    code: Some(self.grade_code.clone()),
                    base_salary: Some(base_salary),
                    description: if self.grade_desc.is_empty() {
                        None
                    } else {
                        Some(self.grade_desc.clone())
                    },
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
            }
            ConfirmAction::CreateDepartment { .. } => {
                let api = self.api.clone();
                let req = CreateDepartmentRequest {
                    name: self.dept_name.clone(),
                    head_id: if self.dept_head_id.is_empty() {
                        None
                    } else {
                        Some(self.dept_head_id.clone())
                    },
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
            }
            ConfirmAction::CreateEmployee { .. } => {
                let api = self.api.clone();
                let req = CreateEmployeeRequest {
                    first_name: self.emp_first_name.clone(),
                    last_name: self.emp_last_name.clone(),
                    email: self.emp_email.clone(),
                    department_id: if self.emp_dept_id.is_empty() {
                        None
                    } else {
                        Some(self.emp_dept_id.clone())
                    },
                    manager_id: if self.emp_manager_id.is_empty() {
                        None
                    } else {
                        Some(self.emp_manager_id.clone())
                    },
                    role: Some(self.emp_role.clone()),
                    salary_grade_id: if self.emp_salary_grade_id.is_empty() {
                        None
                    } else {
                        Some(self.emp_salary_grade_id.clone())
                    },
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
            }
            ConfirmAction::CreateSalaryGrade { .. } => {
                let base_salary = self.grade_salary.parse::<f64>().unwrap_or(0.0);
                let api = self.api.clone();
                let req = CreateSalaryGradeRequest {
                    code: self.grade_code.clone(),
                    base_salary,
                    description: if self.grade_desc.is_empty() {
                        None
                    } else {
                        Some(self.grade_desc.clone())
                    },
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
            }
        }
    }

    /// Show edit/create dialog modal
    pub fn show_edit_dialog(&mut self, ctx: &Context) {
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
                .frame(
                    Frame::none()
                        .fill(colors.surface_variant)
                        .rounding(Rounding::same(16.0))
                        .stroke(Stroke::new(1.0, colors.outline_variant))
                        .inner_margin(Margin::same(28.0)),
                )
                .show(ctx, |ui| {
                    ui.set_min_width(480.0);

                    match &dialog {
                        EditDialog::CreateDepartment | EditDialog::EditDepartment { .. } => {
                            self.show_department_form(ui, &dialog);
                        }
                        EditDialog::CreateEmployee | EditDialog::EditEmployee { .. } => {
                            self.show_employee_form(ui, &dialog);
                        }
                        EditDialog::CreateSalaryGrade | EditDialog::EditSalaryGrade { .. } => {
                            self.show_salary_grade_form(ui, &dialog);
                        }
                    }
                });
        }
    }
}
