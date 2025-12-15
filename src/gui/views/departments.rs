//! Departments view for the Departments tab

use crate::api::models::*;
use crate::gui::app::{ConfirmAction, EditDialog, PersonnelApp};
use crate::gui::{material_button, Material3Colors};
use egui::{Button, Color32, Frame, Margin, RichText, Rounding, Stroke, Ui, Vec2};

impl PersonnelApp {
    pub fn show_departments(&mut self, ui: &mut Ui) {
        let colors = self.colors;
        let emps = self.employees.lock().unwrap().clone();

        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Departments")
                    .size(28.0)
                    .color(colors.on_surface)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if material_button(ui, &colors, "↻ Refresh", false).clicked() {
                    self.refresh_departments();
                }
                ui.add_space(8.0);
                if material_button(ui, &colors, "+ Create", true).clicked() {
                    self.dept_name.clear();
                    self.dept_head_id.clear();
                    self.edit_dialog = Some(EditDialog::CreateDepartment);
                }
            });
        });
        ui.add_space(20.0);

        ui.label(
            RichText::new("All Departments")
                .size(16.0)
                .color(colors.on_surface_variant),
        );
        ui.add_space(12.0);

        let depts = self.departments.lock().unwrap().clone();

        if depts.is_empty() {
            Frame::none()
                .fill(colors.surface_variant)
                .rounding(Rounding::same(12.0))
                .inner_margin(Margin::same(32.0))
                .show(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("No departments yet").color(colors.on_surface_variant),
                        );
                    });
                });
        }

        for dept in depts.iter() {
            let emp_count = emps
                .iter()
                .filter(|e| e.department_id.as_ref() == Some(&dept.id))
                .count();
            self.render_dept_card(ui, &colors, dept, &emps, emp_count);
        }
    }

    fn render_dept_card(
        &mut self,
        ui: &mut Ui,
        colors: &Material3Colors,
        dept: &Department,
        emps: &[Employee],
        emp_count: usize,
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
                            ui.label(RichText::new("🏢").size(24.0));
                        });

                    ui.add_space(12.0);

                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(&dept.name)
                                .size(16.0)
                                .strong()
                                .color(colors.on_surface),
                        );
                        ui.add_space(4.0);
                        let head_name = dept
                            .head_id
                            .as_ref()
                            .and_then(|id| emps.iter().find(|e| &e.id == id))
                            .map(|e| format!("{} {}", e.first_name, e.last_name))
                            .unwrap_or_else(|| "No head assigned".to_string());
                        ui.label(
                            RichText::new(format!("👔 Head: {}", head_name))
                                .size(13.0)
                                .color(colors.on_surface_variant),
                        );
                        ui.label(
                            RichText::new(format!("👥 {} employee(s)", emp_count))
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
                            self.confirm_dialog = Some(ConfirmAction::DeleteDepartment {
                                id: dept.id.clone(),
                                name: dept.name.clone(),
                                employee_count: emp_count,
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
                            self.dept_name = dept.name.clone();
                            self.dept_head_id = dept.head_id.clone().unwrap_or_default();
                            self.edit_dialog = Some(EditDialog::EditDepartment {
                                id: dept.id.clone(),
                                old_head_id: dept.head_id.clone(),
                            });
                        }
                    });
                });
            });
    }
}
