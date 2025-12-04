use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Label, Button, ListBox, ScrolledWindow, PolicyType, Align, glib, MessageDialog, ButtonsType, MessageType};
use std::rc::Rc;
use crate::api::client::ApiClient;
use crate::gui::dialogs::salary_grade_dialog;

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
        .label("Salary Grades")
        .css_classes(vec!["title-1"])
        .halign(Align::Start)
        .hexpand(true)
        .build();
    
    let add_btn = Button::with_label("➕ Add Grade");
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

    let load_grades = Rc::new(move || {
        let api = api_clone.clone();
        let list_box = list_box_clone.clone();
        let window_inner = window_clone.clone();
        let refresh_btn_inner = refresh_btn_clone.clone();
        
        glib::MainContext::default().spawn_local(async move {
            // Fetch both grades and employees to show relationships
            let grades_result = api.get_salary_grades().await;
            let employees_result = api.get_employees(false).await;
            
            match (grades_result, employees_result) {
                (Ok(grades), Ok(employees)) => {
                    while let Some(child) = list_box.first_child() {
                        list_box.remove(&child);
                    }

                    for grade in grades {
                        // Count employees with this salary grade
                        let employee_count = employees.iter()
                            .filter(|e| e.salary_grade_id.as_ref() == Some(&grade.id))
                            .count();
                        
                        let row_outer = GtkBox::new(Orientation::Vertical, 8);

                        let row = GtkBox::new(Orientation::Horizontal, 16);

                        let info_box = GtkBox::new(Orientation::Vertical, 6);
                        let code_label = Label::new(Some(&grade.code));
                        code_label.set_halign(Align::Start);
                        code_label.add_css_class("title-2");

                        let salary_label = Label::new(Some(&format!("💵 ${:.2}", grade.base_salary)));
                        salary_label.set_halign(Align::Start);
                        salary_label.add_css_class("accent");
                        salary_label.add_css_class("accent");
                        
                        let employee_count_label = Label::new(Some(&format!("{} employee(s) with this grade", employee_count)));
                        employee_count_label.set_halign(Align::Start);
                        employee_count_label.add_css_class("caption");

                        info_box.append(&code_label);
                        info_box.append(&salary_label);
                        info_box.append(&employee_count_label);
                        info_box.set_hexpand(true);

                        let view_btn = Button::with_label("View Employees");
                        let edit_btn = Button::with_label("Edit");
                        let delete_btn = Button::with_label("Delete");

                        row.append(&info_box);
                        row.append(&view_btn);
                        row.append(&edit_btn);
                        row.append(&delete_btn);
                        
                        row_outer.append(&row);
                        list_box.append(&row_outer);

                        // View employees button - show which employees have this grade
                        let grade_id = grade.id.clone();
                        let employees_clone = employees.clone();
                        let window_view = window_inner.clone();
                        view_btn.connect_clicked(move |_| {
                            let emps_with_grade: Vec<_> = employees_clone.iter()
                                .filter(|e| e.salary_grade_id.as_ref() == Some(&grade_id))
                                .collect();
                            
                            let dialog = gtk::MessageDialog::builder()
                                .transient_for(&window_view)
                                .modal(true)
                                .title("Employees with this Salary Grade")
                                .build();
                            
                            if emps_with_grade.is_empty() {
                                dialog.set_property("text", "No employees have this salary grade");
                            } else {
                                let names: Vec<String> = emps_with_grade.iter()
                                    .map(|e| format!("{} {} ({})", e.first_name, e.last_name, e.role))
                                    .collect();
                                dialog.set_property("text", format!("Employees:\n\n{}", names.join("\n")));
                            }
                            
                            dialog.add_button("Close", gtk::ResponseType::Close);
                            dialog.connect_response(|dialog, _| dialog.close());
                            dialog.present();
                        });

                        // Edit button
                        let grade_clone = grade.clone();
                        let api_edit = api.clone();
                        let window_edit = window_inner.clone();
                        edit_btn.connect_clicked(move |_| {
                            let grade_inner = grade_clone.clone();
                            let api_inner = api_edit.clone();
                            
                            salary_grade_dialog::show_edit_dialog(&window_edit, &grade_inner, move |id, req| {
                                let api_save = api_inner.clone();
                                glib::MainContext::default().spawn_local(async move {
                                    if let Err(e) = api_save.update_salary_grade(&id, &req).await {
                                        eprintln!("Error updating salary grade: {}", e);
                                    }
                                });
                            });
                        });

                        // Delete button
                        let grade_id = grade.id.clone();
                        let grade_code = grade.code.clone();
                        let api_delete = api.clone();
                        let window_delete = window_inner.clone();
                        let refresh_for_delete = refresh_btn_inner.clone();
                        delete_btn.connect_clicked(move |_| {
                            let id = grade_id.clone();
                            let code = grade_code.clone();
                            let api_del = api_delete.clone();
                            let window_del = window_delete.clone();
                            let refresh_btn_del = refresh_for_delete.clone();
                            
                            // Show confirmation dialog
                            let dialog = MessageDialog::builder()
                                .transient_for(&window_del)
                                .modal(true)
                                .buttons(ButtonsType::YesNo)
                                .message_type(MessageType::Warning)
                                .text(&format!("Delete Salary Grade '{}'?", code))
                                .secondary_text("This action cannot be undone. Employees assigned to this salary grade will no longer have a grade assigned.")
                                .build();
                            
                            dialog.connect_response(move |dialog, response| {
                                if response == gtk::ResponseType::Yes {
                                    let id_inner = id.clone();
                                    let api_inner = api_del.clone();
                                    let refresh_inner = refresh_btn_del.clone();
                                    
                                    glib::MainContext::default().spawn_local(async move {
                                        if let Err(e) = api_inner.delete_salary_grade(&id_inner).await {
                                            eprintln!("Error deleting salary grade: {}", e);
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
                    eprintln!("Error fetching salary grades or employees");
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
        
        salary_grade_dialog::show_create_dialog(&window_inner, move |req| {
            let api_create = api_inner.clone();
            glib::MainContext::default().spawn_local(async move {
                if let Err(e) = api_create.create_salary_grade(&req).await {
                    eprintln!("Error creating salary grade: {}", e);
                }
            });
        });
    });

    // Refresh button
    let load_refresh = load_grades.clone();
    refresh_btn.connect_clicked(move |_| {
        load_refresh();
    });

    load_grades();

    container
}
