//! Salary Grades view for the Salary Grades tab

use crate::api::models::*;
use crate::gui::app::{ConfirmAction, EditDialog, PersonnelApp};
use crate::gui::{material_button, Material3Colors};
use egui::{Button, Color32, Frame, Margin, RichText, Rounding, Stroke, Ui, Vec2};

impl PersonnelApp {
    pub fn show_salary_grades(&mut self, ui: &mut Ui) {
        let colors = self.colors;

        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Salary Grades")
                    .size(28.0)
                    .color(colors.on_surface)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if material_button(ui, &colors, "↻ Refresh", false).clicked() {
                    self.refresh_salary_grades();
                }
                ui.add_space(8.0);
                if material_button(ui, &colors, "+ Create", true).clicked() {
                    self.clear_grade_form();
                    self.edit_dialog = Some(EditDialog::CreateSalaryGrade);
                }
            });
        });
        ui.add_space(20.0);

        ui.label(
            RichText::new("All Salary Grades")
                .size(16.0)
                .color(colors.on_surface_variant),
        );
        ui.add_space(12.0);

        let grades = self.salary_grades.lock().unwrap().clone();
        let employees = self.employees.lock().unwrap().clone();

        if grades.is_empty() {
            Frame::none()
                .fill(colors.surface_variant)
                .rounding(Rounding::same(12.0))
                .inner_margin(Margin::same(32.0))
                .show(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("No salary grades yet").color(colors.on_surface_variant),
                        );
                    });
                });
        }

        for grade in grades.iter() {
            let employee_count = employees
                .iter()
                .filter(|e| e.salary_grade_id.as_deref() == Some(&grade.id))
                .count();
            self.render_grade_card(ui, &colors, grade, employee_count);
        }
    }

    fn render_grade_card(
        &mut self,
        ui: &mut Ui,
        colors: &Material3Colors,
        grade: &SalaryGrade,
        employee_count: usize,
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
                            ui.label(RichText::new("💰").size(24.0));
                        });

                    ui.add_space(12.0);

                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(&grade.code)
                                    .size(16.0)
                                    .strong()
                                    .color(colors.on_surface),
                            );
                            ui.add_space(12.0);
                            ui.label(
                                RichText::new(format!("${:.2}", grade.base_salary))
                                    .size(16.0)
                                    .color(colors.primary),
                            );
                        });
                        ui.add_space(4.0);
                        let desc = grade.description.as_deref().unwrap_or("No description");
                        ui.label(
                            RichText::new(desc)
                                .size(13.0)
                                .color(colors.on_surface_variant),
                        );
                        ui.add_space(2.0);
                        let emp_text = if employee_count == 1 {
                            "employee"
                        } else {
                            "employees"
                        };
                        ui.label(
                            RichText::new(format!("{} {}", employee_count, emp_text))
                                .size(12.0)
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
                            self.confirm_dialog = Some(ConfirmAction::DeleteSalaryGrade {
                                id: grade.id.clone(),
                                code: grade.code.clone(),
                                employee_count,
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
                            self.grade_code = grade.code.clone();
                            self.grade_salary = grade.base_salary.to_string();
                            self.grade_desc = grade.description.clone().unwrap_or_default();
                            self.edit_dialog = Some(EditDialog::EditSalaryGrade {
                                id: grade.id.clone(),
                            });
                        }
                    });
                });
            });
    }

    pub fn clear_grade_form(&mut self) {
        self.grade_code.clear();
        self.grade_salary.clear();
        self.grade_desc.clear();
    }
}
