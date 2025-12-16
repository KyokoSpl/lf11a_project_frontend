//! Main application struct and eframe::App implementation

use super::dialogs::{ConfirmAction, EditDialog};
use super::{Material3Colors, Tab};
use crate::api::client::ApiClient;
use crate::api::models::*;
use egui::{Color32, Frame, Margin, Rounding, Stroke, Vec2};
use std::sync::{Arc, Mutex};

/// Main application state for Personnel Management
#[allow(dead_code)]
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
            .frame(
                Frame::none()
                    .fill(colors.surface)
                    .inner_margin(Margin::symmetric(24.0, 12.0)),
            )
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
                                .size(14.0),
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
                            egui::RichText::new(theme_icon)
                                .size(18.0)
                                .color(colors.on_surface),
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
            .frame(
                Frame::none()
                    .fill(colors.surface)
                    .inner_margin(Margin::symmetric(32.0, 24.0)),
            )
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
