use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Label, Button, ListBox, ScrolledWindow, PolicyType, Align, glib, MessageDialog, ButtonsType, MessageType};
use std::rc::Rc;
use crate::api::client::ApiClient;
use crate::gui::dialogs::employee_dialog;

pub fn build<W: IsA<gtk::Window>>(api: ApiClient, window: W) -> GtkBox {
    let container = GtkBox::new(Orientation::Vertical, 16);
    container.set_margin_top(20);
    container.set_margin_bottom(20);
    container.set_margin_start(24);
    container.set_margin_end(24);

    let header = GtkBox::new(Orientation::Horizontal, 16);
    header.add_css_class("toolbar");
    header.set_margin_bottom(12);
    
    let title = Label::builder()
        .label("Employees")
        .css_classes(vec!["title-1"])
        .halign(Align::Start)
        .hexpand(true)
        .build();
    
    let add_btn = Button::with_label("➕ Add Employee");
    add_btn.add_css_class("suggested-action");
    
    let refresh_btn = Button::with_label("🔄 Refresh");
    
    header.append(&title);
    header.append(&add_btn);
    header.append(&refresh_btn);
    container.append(&header);

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::None);
    list_box.add_css_class("boxed-list");

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&list_box)
        .build();
    scrolled.set_margin_top(12);

    container.append(&scrolled);

    let api_clone = api.clone();
    let list_box_clone = list_box.clone();
    let window_clone = window.clone();
    let refresh_btn_clone = refresh_btn.clone();

    let load_employees = Rc::new(move || {
        let api = api_clone.clone();
        let list_box = list_box_clone.clone();
        let window_inner = window_clone.clone();
        let refresh_btn_inner = refresh_btn_clone.clone();
        
        glib::MainContext::default().spawn_local(async move {
            // Fetch employees, departments, and salary grades to show relationships
            let employees_result = api.get_employees(false).await;
            let departments_result = api.get_departments().await;
            let grades_result = api.get_salary_grades().await;
            
            match (employees_result, departments_result, grades_result) {
                (Ok(employees), Ok(departments), Ok(grades)) => {
                    while let Some(child) = list_box.first_child() {
                        list_box.remove(&child);
                    }

                    for emp in employees {
                        let row_container = GtkBox::new(Orientation::Horizontal, 16);

                        let info_box = GtkBox::new(Orientation::Vertical, 8);
                        let name_label = Label::new(Some(&format!("{} {}", emp.first_name, emp.last_name)));
                        name_label.set_halign(Align::Start);
                        name_label.add_css_class("title-2");

                        let email_label = Label::new(Some(&format!("📧 {}", &emp.email)));
                        email_label.set_halign(Align::Start);
                        email_label.add_css_class("caption");

                        let role_label = Label::new(Some(&format!("💼 {}", &emp.role)));
                        role_label.set_halign(Align::Start);
                        role_label.add_css_class("caption");
                        
                        // Show department
                        let dept_name = emp.department_id.as_ref()
                            .and_then(|id| departments.iter().find(|d| &d.id == id))
                            .map(|d| d.name.clone())
                            .unwrap_or_else(|| "No department".to_string());
                        let dept_label = Label::new(Some(&format!("Department: {}", dept_name)));
                        dept_label.set_halign(Align::Start);
                        dept_label.add_css_class("caption");
                        
                        // Show salary grade
                        let grade_info = emp.salary_grade_id.as_ref()
                            .and_then(|id| grades.iter().find(|g| &g.id == id))
                            .map(|g| format!("Salary Grade: {} (${:.2})", g.code, g.base_salary))
                            .unwrap_or_else(|| "No salary grade assigned".to_string());
                        let grade_label = Label::new(Some(&grade_info));
                        grade_label.set_halign(Align::Start);
                        grade_label.add_css_class("caption");
                        
                        info_box.append(&name_label);
                        info_box.append(&email_label);
                        info_box.append(&role_label);
                        info_box.append(&dept_label);
                        info_box.append(&grade_label);
                        info_box.set_hexpand(true);

                        // Button container with vertical orientation
                        let button_box = GtkBox::new(Orientation::Vertical, 8);
                        let edit_btn = Button::with_label("Edit");
                        let delete_btn = Button::with_label("Delete");
                        button_box.append(&edit_btn);
                        button_box.append(&delete_btn);

                        row_container.append(&info_box);
                        row_container.append(&button_box);
                        
                        list_box.append(&row_container);

                        // Edit button
                        let emp_clone = emp.clone();
                        let api_edit = api.clone();
                        let window_edit = window_inner.clone();
                        let departments_clone = departments.clone();
                        let grades_edit = grades.clone();
                        let refresh_for_edit = refresh_btn_inner.clone();
                        edit_btn.connect_clicked(move |_| {
                            let emp_inner = emp_clone.clone();
                            let api_inner = api_edit.clone();
                            let depts = departments_clone.clone();
                            let salary_grades = grades_edit.clone();
                            let refresh_btn_edit = refresh_for_edit.clone();
                            
                            employee_dialog::show_edit_dialog(&window_edit, &emp_inner, depts, salary_grades, move |id, req| {
                                let api_save = api_inner.clone();
                                let refresh_btn_save = refresh_btn_edit.clone();
                                glib::MainContext::default().spawn_local(async move {
                                    if let Err(e) = api_save.update_employee(&id, &req).await {
                                        eprintln!("Error updating employee: {}", e);
                                    } else {
                                        // Trigger refresh by clicking the refresh button
                                        refresh_btn_save.emit_clicked();
                                    }
                                });
                            });
                        });

                        // Delete button
                        let emp_id = emp.id.clone();
                        let emp_name = format!("{} {}", emp.first_name, emp.last_name);
                        let api_delete = api.clone();
                        let window_delete = window_inner.clone();
                        let refresh_for_delete = refresh_btn_inner.clone();
                        delete_btn.connect_clicked(move |_| {
                            let id = emp_id.clone();
                            let name = emp_name.clone();
                            let api_del = api_delete.clone();
                            let window_del = window_delete.clone();
                            let refresh_btn_del = refresh_for_delete.clone();
                            
                            // Show confirmation dialog
                            let dialog = MessageDialog::builder()
                                .transient_for(&window_del)
                                .modal(true)
                                .buttons(ButtonsType::YesNo)
                                .message_type(MessageType::Warning)
                                .text(&format!("Delete Employee '{}'?", name))
                                .secondary_text("This action cannot be undone. The employee record will be permanently removed from the system.")
                                .build();
                            
                            dialog.connect_response(move |dialog, response| {
                                if response == gtk::ResponseType::Yes {
                                    let id_inner = id.clone();
                                    let api_inner = api_del.clone();
                                    let refresh_inner = refresh_btn_del.clone();
                                    
                                    glib::MainContext::default().spawn_local(async move {
                                        if let Err(e) = api_inner.delete_employee(&id_inner).await {
                                            eprintln!("Error deleting employee: {}", e);
                                        } else {
                                            refresh_inner.emit_clicked();
                                        }
                                    });
                                }
                                dialog.close();
                            });
                            
                            dialog.present();
                        });
                    }
                }
                _ => {
                    eprintln!("Error fetching employees, departments, or salary grades");
                }
            }
        });
    });

    // Add button
    let api_add = api.clone();
    let window_add = window.clone();
    add_btn.connect_clicked(move |_| {
        let api_inner = api_add.clone();
        let window_inner = window_add.clone();
        
        // Fetch data for dropdowns
        glib::MainContext::default().spawn_local(async move {
            let depts = api_inner.get_departments().await.unwrap_or_default();
            let emps = api_inner.get_employees(false).await.unwrap_or_default();
            let grades = api_inner.get_salary_grades().await.unwrap_or_default();
            
            employee_dialog::show_create_dialog(&window_inner, depts, emps, grades, move |req| {
                let api_create = api_inner.clone();
                glib::MainContext::default().spawn_local(async move {
                    if let Err(e) = api_create.create_employee(&req).await {
                        eprintln!("Error creating employee: {}", e);
                    }
                });
            });
        });
    });

    // Refresh button
    let load_refresh = load_employees.clone();
    refresh_btn.connect_clicked(move |_| {
        load_refresh();
    });

    load_employees();

    container
}
