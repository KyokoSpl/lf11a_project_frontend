use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Label, Button, ListBox, ScrolledWindow, PolicyType, Align, glib};
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

    let load_employees = Rc::new(move || {
        let api = api_clone.clone();
        let list_box = list_box_clone.clone();
        let window_inner = window_clone.clone();
        
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

                        let assign_grade_btn = Button::with_label("Assign Grade");
                        let edit_btn = Button::with_label("Edit");
                        let delete_btn = Button::with_label("Delete");

                        row_container.append(&info_box);
                        row_container.append(&assign_grade_btn);
                        row_container.append(&edit_btn);
                        row_container.append(&delete_btn);
                        
                        list_box.append(&row_container);

                        // Assign/Change Salary Grade button
                        let emp_id = emp.id.clone();
                        let api_assign = api.clone();
                        let window_assign = window_inner.clone();
                        let grades_clone = grades.clone();
                        assign_grade_btn.connect_clicked(move |_| {
                            let id = emp_id.clone();
                            let api_inner = api_assign.clone();
                            let grades_inner = grades_clone.clone();
                            let window_dialog = window_assign.clone();
                            
                            // Show dialog to select salary grade
                            let dialog = gtk::Dialog::builder()
                                .transient_for(&window_dialog)
                                .modal(true)
                                .title("Assign Salary Grade")
                                .build();
                            
                            dialog.add_button("Cancel", gtk::ResponseType::Cancel);
                            dialog.add_button("Assign", gtk::ResponseType::Accept);
                            
                            let content = dialog.content_area();
                            let vbox = GtkBox::new(Orientation::Vertical, 12);
                            vbox.set_margin_top(12);
                            vbox.set_margin_bottom(12);
                            vbox.set_margin_start(12);
                            vbox.set_margin_end(12);
                            
                            vbox.append(&Label::builder()
                                .label("Select Salary Grade:")
                                .halign(Align::Start)
                                .build());
                            
                            let list_box_grades = ListBox::new();
                            list_box_grades.set_selection_mode(gtk::SelectionMode::Single);
                            
                            for grade in &grades_inner {
                                let row_label = Label::new(Some(&format!("{} - ${:.2}", grade.code, grade.base_salary)));
                                row_label.set_halign(Align::Start);
                                list_box_grades.append(&row_label);
                            }
                            
                            let scrolled = ScrolledWindow::builder()
                                .child(&list_box_grades)
                                .min_content_height(200)
                                .build();
                            vbox.append(&scrolled);
                            content.append(&vbox);
                            
                            dialog.connect_response(move |dialog, response| {
                                if response == gtk::ResponseType::Accept {
                                    if let Some(selected_row) = list_box_grades.selected_row() {
                                        let index = selected_row.index() as usize;
                                        if let Some(selected_grade) = grades_inner.get(index) {
                                            let grade_id = selected_grade.id.clone();
                                            let api_save = api_inner.clone();
                                            let emp_id_save = id.clone();
                                            
                                            glib::MainContext::default().spawn_local(async move {
                                                let req = crate::api::models::AssignSalaryGradeRequest {
                                                    salary_grade_id: grade_id,
                                                };
                                                if let Err(e) = api_save.assign_salary_grade(&emp_id_save, &req).await {
                                                    eprintln!("Error assigning salary grade: {}", e);
                                                }
                                            });
                                        }
                                    }
                                }
                                dialog.close();
                            });
                            
                            dialog.present();
                        });

                        // Edit button
                        let emp_clone = emp.clone();
                        let api_edit = api.clone();
                        let window_edit = window_inner.clone();
                        edit_btn.connect_clicked(move |_| {
                            let emp_inner = emp_clone.clone();
                            let api_inner = api_edit.clone();
                            
                            employee_dialog::show_edit_dialog(&window_edit, &emp_inner, move |id, req| {
                                let api_save = api_inner.clone();
                                glib::MainContext::default().spawn_local(async move {
                                    if let Err(e) = api_save.update_employee(&id, &req).await {
                                        eprintln!("Error updating employee: {}", e);
                                    }
                                });
                            });
                        });

                        // Delete button
                        let emp_id = emp.id.clone();
                        let api_delete = api.clone();
                        delete_btn.connect_clicked(move |_| {
                            let id = emp_id.clone();
                            let api_del = api_delete.clone();
                            
                            glib::MainContext::default().spawn_local(async move {
                                if let Err(e) = api_del.delete_employee(&id).await {
                                    eprintln!("Error deleting employee: {}", e);
                                }
                            });
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
