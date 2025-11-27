use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Notebook, Label, Box, Orientation};
use crate::api::client::ApiClient;
use crate::gui::views::{department_view, employee_view, salary_grade_view};

pub fn build(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Personnel Management System")
        .default_width(1000)
        .default_height(700)
        .build();

    // Main container with padding
    let main_box = Box::new(Orientation::Vertical, 0);
    main_box.set_margin_top(0);
    main_box.set_margin_bottom(0);
    main_box.set_margin_start(0);
    main_box.set_margin_end(0);

    let api = ApiClient::new();
    let notebook = Notebook::new();
    notebook.set_tab_pos(gtk::PositionType::Top);

    // Create tab labels with icons/emojis
    let dept_label = Label::new(Some("🏢 Departments"));
    dept_label.add_css_class("heading");
    let dept_view = department_view::build(api.clone(), window.clone());
    notebook.append_page(&dept_view, Some(&dept_label));

    let emp_label = Label::new(Some("👥 Employees"));
    emp_label.add_css_class("heading");
    let emp_view = employee_view::build(api.clone(), window.clone());
    notebook.append_page(&emp_view, Some(&emp_label));

    let salary_label = Label::new(Some("💰 Salary Grades"));
    salary_label.add_css_class("heading");
    let salary_view = salary_grade_view::build(api.clone(), window.clone());
    notebook.append_page(&salary_view, Some(&salary_label));

    main_box.append(&notebook);
    window.set_child(Some(&main_box));
    window.present();
}
