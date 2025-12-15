//! Employees view for the Employees tab

use crate::api::models::*;
use crate::gui::app::{ConfirmAction, EditDialog, PersonnelApp};
use crate::gui::{material_button, Material3Colors};
use egui::{Button, Color32, Frame, Margin, RichText, Rounding, Stroke, Ui, Vec2};

impl PersonnelApp {
    pub fn show_employees(&mut self, ui: &mut Ui) {
        let colors = self.colors;

        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Employees")
                    .size(28.0)
                    .color(colors.on_surface)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if material_button(ui, &colors, "↻ Refresh", false).clicked() {
                    self.refresh_employees();
                }
                ui.add_space(8.0);
                if material_button(ui, &colors, "+ Create", true).clicked() {
                    self.clear_emp_form();
                    self.edit_dialog = Some(EditDialog::CreateEmployee);
                }
            });
        });
        ui.add_space(20.0);

        ui.label(
            RichText::new("All Employees")
                .size(16.0)
                .color(colors.on_surface_variant),
        );
        ui.add_space(12.0);

        let emps = self.employees.lock().unwrap().clone();
        let depts = self.departments.lock().unwrap().clone();
        let grades = self.salary_grades.lock().unwrap().clone();

        if emps.is_empty() {
            Frame::none()
                .fill(colors.surface_variant)
                .rounding(Rounding::same(12.0))
                .inner_margin(Margin::same(32.0))
                .show(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("No employees yet").color(colors.on_surface_variant),
                        );
                    });
                });
        }

        for emp in emps.iter() {
            self.render_employee_card(ui, &colors, emp, &depts, &emps, &grades);
        }
    }

    fn render_employee_card(
        &mut self,
        ui: &mut Ui,
        colors: &Material3Colors,
        emp: &Employee,
        depts: &[Department],
        _all_emps: &[Employee],
        grades: &[SalaryGrade],
    ) {
        Frame::none()
            .fill(colors.surface_variant)
            .stroke(Stroke::NONE)
            .rounding(Rounding::same(12.0))
            .inner_margin(Margin::same(16.0))
            .outer_margin(Margin::symmetric(0.0, 4.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    Frame::none()
                        .fill(colors.primary_container)
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::same(12.0))
                        .show(ui, |ui| {
                            ui.label(RichText::new("👤").size(24.0));
                        });

                    ui.add_space(12.0);

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{} {}", emp.first_name, emp.last_name))
                                .size(16.0)
                                .strong()
                                .color(colors.on_surface),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(&emp.email)
                                .size(13.0)
                                .color(colors.on_surface_variant),
                        );
                        ui.label(
                            RichText::new(format!("Role: {}", emp.role))
                                .size(13.0)
                                .color(colors.on_surface_variant),
                        );
                        let dept_name = emp
                            .department_id
                            .as_ref()
                            .and_then(|id| depts.iter().find(|d| &d.id == id))
                            .map(|d| d.name.clone())
                            .unwrap_or_else(|| "No department".to_string());
                        ui.label(
                            RichText::new(format!("Dept: {}", dept_name))
                                .size(13.0)
                                .color(colors.on_surface_variant),
                        );
                        let grade_info = emp
                            .salary_grade_id
                            .as_ref()
                            .and_then(|id| grades.iter().find(|g| &g.id == id))
                            .map(|g| format!("{} - ${:.2}", g.code, g.base_salary))
                            .unwrap_or_else(|| "No grade".to_string());
                        ui.label(
                            RichText::new(format!("Grade: {}", grade_info))
                                .size(13.0)
                                .color(colors.on_surface_variant),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let delete_btn =
                            Button::new(RichText::new("🗑 Delete").size(12.0).color(Color32::WHITE))
                                .fill(Color32::from_rgb(220, 53, 69))
                                .stroke(Stroke::NONE)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(75.0, 36.0));
                        if ui.add(delete_btn).clicked() {
                            self.confirm_dialog = Some(ConfirmAction::DeleteEmployee {
                                id: emp.id.clone(),
                                name: format!("{} {}", emp.first_name, emp.last_name),
                            });
                        }
                        ui.add_space(8.0);
                        let edit_btn = Button::new(
                            RichText::new("✏ Edit").size(12.0).color(colors.on_primary),
                        )
                        .fill(colors.primary)
                        .stroke(Stroke::NONE)
                        .rounding(Rounding::same(8.0))
                        .min_size(Vec2::new(65.0, 36.0));
                        if ui.add(edit_btn).clicked() {
                            self.emp_first_name = emp.first_name.clone();
                            self.emp_last_name = emp.last_name.clone();
                            self.emp_email = emp.email.clone();
                            self.emp_role = emp.role.clone();
                            self.emp_dept_id = emp.department_id.clone().unwrap_or_default();
                            self.emp_manager_id = emp.manager_id.clone().unwrap_or_default();
                            self.emp_salary_grade_id =
                                emp.salary_grade_id.clone().unwrap_or_default();
                            self.edit_dialog =
                                Some(EditDialog::EditEmployee { id: emp.id.clone() });
                        }
                    });
                });
            });
    }

    pub fn clear_emp_form(&mut self) {
        self.emp_first_name.clear();
        self.emp_last_name.clear();
        self.emp_email.clear();
        self.emp_role.clear();
        self.emp_dept_id.clear();
        self.emp_manager_id.clear();
        self.emp_salary_grade_id.clear();
    }
}
