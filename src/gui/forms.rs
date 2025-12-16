//! Form handlers for create/edit dialogs

use super::dialogs::{ConfirmAction, EditDialog};
use super::{styled_dropdown, PersonnelApp};
use egui::{Frame, Margin, Rounding, Stroke, Ui, Vec2};

impl PersonnelApp {
    /// Show department form in dialog
    pub fn show_department_form(&mut self, ui: &mut Ui, dialog: &EditDialog) {
        let colors = self.colors;
        let emps = self.employees.lock().unwrap().clone();

        ui.vertical(|ui| {
            // Department Name
            ui.label(
                egui::RichText::new("Department Name")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.dept_name)
                            .desired_width(405.0)
                            .hint_text(
                                egui::RichText::new("Enter department name...")
                                    .color(colors.on_surface_variant),
                            )
                            .text_color(colors.on_surface)
                            .frame(false),
                    );
                });

            ui.add_space(16.0);

            // Department Head dropdown
            ui.label(
                egui::RichText::new("Department Head (optional)")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
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
                        let none_text =
                            egui::RichText::new("❌ No head assigned").color(if is_none_selected {
                                colors.on_primary
                            } else {
                                colors.on_surface
                            });
                        if ui.selectable_label(is_none_selected, none_text).clicked() {
                            self.dept_head_id.clear();
                        }
                        ui.separator();
                        for emp in &emps {
                            let is_selected = self.dept_head_id == emp.id;
                            let label =
                                format!("👤 {} {} - {}", emp.first_name, emp.last_name, emp.role);
                            let label_text = egui::RichText::new(label).color(if is_selected {
                                colors.on_primary
                            } else {
                                colors.on_surface
                            });
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
                    egui::RichText::new("Cancel")
                        .size(13.0)
                        .color(colors.on_surface),
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
                    egui::RichText::new(action_label)
                        .size(13.0)
                        .color(colors.on_primary),
                )
                .fill(colors.primary)
                .stroke(Stroke::NONE)
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(100.0, 40.0));

                if ui.add(action_btn).clicked() && !self.dept_name.is_empty() {
                    if is_edit {
                        if let EditDialog::EditDepartment { id, old_head_id } = dialog {
                            let old_head_name = old_head_id
                                .as_ref()
                                .and_then(|hid| emps.iter().find(|e| &e.id == hid))
                                .map(|e| format!("{} {}", e.first_name, e.last_name));
                            let new_head_id = if self.dept_head_id.is_empty() {
                                None
                            } else {
                                Some(self.dept_head_id.clone())
                            };
                            let new_head_name = if self.dept_head_id.is_empty() {
                                None
                            } else {
                                emps.iter()
                                    .find(|e| e.id == self.dept_head_id)
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
    pub fn show_employee_form(&mut self, ui: &mut Ui, dialog: &EditDialog) {
        let colors = self.colors;
        let depts = self.departments.lock().unwrap().clone();
        let emps = self.employees.lock().unwrap().clone();
        let grades = self.salary_grades.lock().unwrap().clone();

        ui.vertical(|ui| {
            // Row 1: First Name, Last Name
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("First Name")
                            .size(12.0)
                            .color(colors.on_surface_variant),
                    );
                    ui.add_space(4.0);
                    Frame::none()
                        .fill(colors.surface)
                        .stroke(Stroke::new(1.0, colors.outline_variant))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(12.0, 10.0))
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.emp_first_name)
                                    .desired_width(195.0)
                                    .hint_text(
                                        egui::RichText::new("First name...")
                                            .color(colors.on_surface_variant),
                                    )
                                    .text_color(colors.on_surface)
                                    .frame(false),
                            );
                        });
                });
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Last Name")
                            .size(12.0)
                            .color(colors.on_surface_variant),
                    );
                    ui.add_space(4.0);
                    Frame::none()
                        .fill(colors.surface)
                        .stroke(Stroke::new(1.0, colors.outline_variant))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(12.0, 10.0))
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.emp_last_name)
                                    .desired_width(195.0)
                                    .hint_text(
                                        egui::RichText::new("Last name...")
                                            .color(colors.on_surface_variant),
                                    )
                                    .text_color(colors.on_surface)
                                    .frame(false),
                            );
                        });
                });
            });

            ui.add_space(12.0);

            // Email
            ui.label(
                egui::RichText::new("Email")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.emp_email)
                            .desired_width(405.0)
                            .hint_text(
                                egui::RichText::new("email@example.com")
                                    .color(colors.on_surface_variant),
                            )
                            .text_color(colors.on_surface)
                            .frame(false),
                    );
                });

            ui.add_space(12.0);

            // Role dropdown
            ui.label(
                egui::RichText::new("Role")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
            ui.add_space(4.0);
            let role_display = if self.emp_role.is_empty() {
                "Select role...".to_string()
            } else {
                self.emp_role.clone()
            };
            styled_dropdown(ui, &colors, |ui| {
                egui::ComboBox::from_id_salt("emp_role_dialog_dropdown")
                    .selected_text(egui::RichText::new(&role_display).color(colors.on_surface))
                    .width(430.0)
                    .height(300.0)
                    .show_ui(ui, |ui| {
                        ui.set_min_width(405.0);
                        for role in ["Employee", "DepartmentHead", "DeputyHead", "Admin"] {
                            let is_selected = self.emp_role == role;
                            let label_text = egui::RichText::new(role).color(if is_selected {
                                colors.on_primary
                            } else {
                                colors.on_surface
                            });
                            if ui.selectable_label(is_selected, label_text).clicked() {
                                self.emp_role = role.to_string();
                            }
                        }
                    })
            });

            ui.add_space(12.0);

            // Department dropdown
            ui.label(
                egui::RichText::new("Department")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
            ui.add_space(4.0);
            let dept_name = if self.emp_dept_id.is_empty() {
                "No department".to_string()
            } else {
                depts
                    .iter()
                    .find(|d| d.id == self.emp_dept_id)
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
                        let none_text =
                            egui::RichText::new("❌ No department").color(if is_none_selected {
                                colors.on_primary
                            } else {
                                colors.on_surface
                            });
                        if ui.selectable_label(is_none_selected, none_text).clicked() {
                            self.emp_dept_id.clear();
                        }
                        ui.separator();
                        for dept in &depts {
                            let is_selected = self.emp_dept_id == dept.id;
                            let label_text =
                                egui::RichText::new(&dept.name).color(if is_selected {
                                    colors.on_primary
                                } else {
                                    colors.on_surface
                                });
                            if ui.selectable_label(is_selected, label_text).clicked() {
                                self.emp_dept_id = dept.id.clone();
                            }
                        }
                    })
            });

            ui.add_space(12.0);

            // Manager dropdown
            ui.label(
                egui::RichText::new("Manager")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
            ui.add_space(4.0);
            let manager_name = if self.emp_manager_id.is_empty() {
                "No manager".to_string()
            } else {
                emps.iter()
                    .find(|e| e.id == self.emp_manager_id)
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
                        let none_text =
                            egui::RichText::new("❌ No manager").color(if is_none_selected {
                                colors.on_primary
                            } else {
                                colors.on_surface
                            });
                        if ui.selectable_label(is_none_selected, none_text).clicked() {
                            self.emp_manager_id.clear();
                        }
                        ui.separator();
                        for emp in &emps {
                            let is_selected = self.emp_manager_id == emp.id;
                            let label =
                                format!("{} {} - {}", emp.first_name, emp.last_name, emp.role);
                            let label_text = egui::RichText::new(label).color(if is_selected {
                                colors.on_primary
                            } else {
                                colors.on_surface
                            });
                            if ui.selectable_label(is_selected, label_text).clicked() {
                                self.emp_manager_id = emp.id.clone();
                            }
                        }
                    })
            });

            ui.add_space(12.0);

            // Salary Grade dropdown
            ui.label(
                egui::RichText::new("Salary Grade")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
            ui.add_space(4.0);
            let grade_name = if self.emp_salary_grade_id.is_empty() {
                "No salary grade".to_string()
            } else {
                grades
                    .iter()
                    .find(|g| g.id == self.emp_salary_grade_id)
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
                        let none_text =
                            egui::RichText::new("❌ No salary grade").color(if is_none_selected {
                                colors.on_primary
                            } else {
                                colors.on_surface
                            });
                        if ui.selectable_label(is_none_selected, none_text).clicked() {
                            self.emp_salary_grade_id.clear();
                        }
                        ui.separator();
                        for grade in &grades {
                            let is_selected = self.emp_salary_grade_id == grade.id;
                            let label = format!("{} - ${:.2}", grade.code, grade.base_salary);
                            let label_text = egui::RichText::new(label).color(if is_selected {
                                colors.on_primary
                            } else {
                                colors.on_surface
                            });
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
                    egui::RichText::new("Cancel")
                        .size(13.0)
                        .color(colors.on_surface),
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
                    egui::RichText::new(action_label)
                        .size(13.0)
                        .color(colors.on_primary),
                )
                .fill(colors.primary)
                .stroke(Stroke::NONE)
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(100.0, 40.0));

                if ui.add(action_btn).clicked()
                    && !self.emp_first_name.is_empty()
                    && !self.emp_last_name.is_empty()
                {
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
    pub fn show_salary_grade_form(&mut self, ui: &mut Ui, dialog: &EditDialog) {
        let colors = self.colors;

        ui.vertical(|ui| {
            // Grade Code
            ui.label(
                egui::RichText::new("Grade Code")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.grade_code)
                            .desired_width(405.0)
                            .hint_text(
                                egui::RichText::new("e.g. A1, B2, C3...")
                                    .color(colors.on_surface_variant),
                            )
                            .text_color(colors.on_surface)
                            .frame(false),
                    );
                });

            ui.add_space(12.0);

            // Base Salary
            ui.label(
                egui::RichText::new("Base Salary")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.grade_salary)
                            .desired_width(405.0)
                            .hint_text(
                                egui::RichText::new("Enter amount (e.g. 50000)")
                                    .color(colors.on_surface_variant),
                            )
                            .text_color(colors.on_surface)
                            .frame(false),
                    );
                });

            ui.add_space(12.0);

            // Description
            ui.label(
                egui::RichText::new("Description (optional)")
                    .size(12.0)
                    .color(colors.on_surface_variant),
            );
            ui.add_space(4.0);
            Frame::none()
                .fill(colors.surface)
                .stroke(Stroke::new(1.0, colors.outline_variant))
                .rounding(Rounding::same(8.0))
                .inner_margin(Margin::symmetric(12.0, 10.0))
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.grade_desc)
                            .desired_width(405.0)
                            .hint_text(
                                egui::RichText::new("Description...")
                                    .color(colors.on_surface_variant),
                            )
                            .text_color(colors.on_surface)
                            .frame(false),
                    );
                });

            ui.add_space(24.0);

            // Buttons
            ui.horizontal(|ui| {
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
                    self.edit_dialog = None;
                    self.clear_grade_form();
                }

                ui.add_space(12.0);

                let (action_label, is_edit) = match dialog {
                    EditDialog::EditSalaryGrade { .. } => ("Update", true),
                    _ => ("Create", false),
                };

                let action_btn = egui::Button::new(
                    egui::RichText::new(action_label)
                        .size(13.0)
                        .color(colors.on_primary),
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
