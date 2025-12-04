use gtk::prelude::*;
use gtk::{Box as GtkBox, Orientation, Label, Button, ListBox, ScrolledWindow, PolicyType, Align, glib, MessageDialog, ButtonsType, MessageType};
use std::rc::Rc;
use std::cell::RefCell;
use crate::api::client::ApiClient;
use crate::api::models::UpdateEmployeeRequest;
use crate::gui::dialogs::department_dialog;

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
        .label("Departments")
        .css_classes(vec!["title-1"])
        .halign(Align::Start)
        .hexpand(true)
        .build();
    
    let add_btn = Button::with_label("➕ Add Department");
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

    // Create a shared refresh function using Rc
    let refresh_fn: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::new(RefCell::new(None));
    
    let api_load = api.clone();
    let list_load = list_box.clone();
    let window_load = window.clone();
    let refresh_fn_clone = refresh_fn.clone();
    
    let do_load = Rc::new(move || {
        let api = api_load.clone();
        let list_box = list_load.clone();
        let window_inner = window_load.clone();
        let refresh_fn_inner = refresh_fn_clone.clone();
        
        glib::MainContext::default().spawn_local(async move {
            // Fetch both departments and employees to show relationships
            println!("DEBUG: Fetching fresh data from API...");
            let departments_result = api.get_departments().await;
            let employees_result = api.get_employees(false).await;
            
            match (departments_result, employees_result) {
                (Ok(departments), Ok(employees)) => {
                    println!("DEBUG: Received {} departments and {} employees", departments.len(), employees.len());
                    // Log employee roles
                    for emp in &employees {
                        println!("DEBUG: Employee {} {} - Role: {}", emp.first_name, emp.last_name, emp.role);
                    }
                    while let Some(child) = list_box.first_child() {
                        list_box.remove(&child);
                    }

                    for dept in departments {
                        // Count employees in this department
                        let employee_count = employees.iter()
                            .filter(|e| e.department_id.as_ref() == Some(&dept.id))
                            .count();
                        
                        let row_outer = GtkBox::new(Orientation::Vertical, 8);
                        
                        let row = GtkBox::new(Orientation::Horizontal, 16);

                        let info_box = GtkBox::new(Orientation::Vertical, 6);
                        let name_label = Label::new(Some(&dept.name));
                        name_label.set_halign(Align::Start);
                        name_label.add_css_class("title-2");
                        
                        // Find head employee name
                        let head_name = if let Some(ref head_id) = dept.head_id {
                            employees.iter()
                                .find(|e| &e.id == head_id)
                                .map(|e| format!("👤 Head: {} {}", e.first_name, e.last_name))
                                .unwrap_or_else(|| "👤 Head: Unknown".to_string())
                        } else {
                            "👤 Head: None".to_string()
                        };
                        
                        let head_label = Label::new(Some(&head_name));
                        head_label.set_halign(Align::Start);
                        head_label.add_css_class("caption");
                        
                        let employee_count_label = Label::new(Some(&format!("👥 {} employee(s)", employee_count)));
                        employee_count_label.set_halign(Align::Start);
                        employee_count_label.add_css_class("caption");
                        
                        info_box.append(&name_label);
                        info_box.append(&head_label);
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

                        // View employees button
                        let dept_id = dept.id.clone();
                        let employees_clone = employees.clone();
                        let window_view = window_inner.clone();
                        view_btn.connect_clicked(move |_| {
                            let emps_in_dept: Vec<_> = employees_clone.iter()
                                .filter(|e| e.department_id.as_ref() == Some(&dept_id))
                                .collect();
                            
                            let dialog = gtk::MessageDialog::builder()
                                .transient_for(&window_view)
                                .modal(true)
                                .title("Employees in this Department")
                                .build();
                            
                            if emps_in_dept.is_empty() {
                                dialog.set_property("text", "No employees in this department");
                            } else {
                                let names: Vec<String> = emps_in_dept.iter()
                                    .map(|e| format!("{} {} ({})", e.first_name, e.last_name, e.role))
                                    .collect();
                                dialog.set_property("text", format!("Employees:\n\n{}", names.join("\n")));
                            }
                            
                            dialog.add_button("Close", gtk::ResponseType::Close);
                            dialog.connect_response(|dialog, _| dialog.close());
                            dialog.present();
                        });

                        // Edit button handler
                        let dept_clone = dept.clone();
                        let api_edit = api.clone();
                        let window_edit = window_inner.clone();
                        let refresh_edit = refresh_fn_inner.clone();
                        edit_btn.connect_clicked(move |_| {
                            let dept_edit = dept_clone.clone();
                            let api_inner = api_edit.clone();
                            let refresh_inner = refresh_edit.clone();
                            let window_confirm = window_edit.clone();
                            
                            // Fetch fresh employee list for the dialog
                            glib::MainContext::default().spawn_local(async move {
                                match api_inner.get_employees(false).await {
                                    Ok(fresh_employees) => {
                                        let api_save_outer = api_inner.clone();
                                        let refresh_save_outer = refresh_inner.clone();
                                        let window_handler_outer = window_confirm.clone();
                                        
                                        department_dialog::show_edit_dialog(&window_confirm, &dept_edit, fresh_employees.clone(), move |id, req, old_head_id, new_head_id| {
                                            let api_save = api_save_outer.clone();
                                            let refresh_save = refresh_save_outer.clone();
                                            let employees_handler = fresh_employees.clone();
                                            let window_handler = window_handler_outer.clone();
                                            
                                            glib::MainContext::default().spawn_local(async move {
                                                // Update department first
                                                if let Err(e) = api_save.update_department(&id, &req).await {
                                                    crate::log_error!("Error updating department: {}", e);
                                                    return;
                                                }
                                                
                                                println!("Department updated successfully");
                                                
                                                // Handle role changes if head changed
                                                if old_head_id != new_head_id {
                                                    println!("Head changed from {:?} to {:?}", old_head_id, new_head_id);
                                                    
                                                    // Promote new head to "Department Head" if selected
                                                    if let Some(ref new_id) = new_head_id {
                                                        println!("Promoting employee {}", new_id);
                                                        let update_req = UpdateEmployeeRequest {
                                                            role: Some("DepartmentHead".to_string()),
                                                            ..Default::default()
                                                        };
                                                        match api_save.update_employee(new_id, &update_req).await {
                                                            Ok(_) => {
                                                                println!("Successfully promoted employee {}", new_id);
                                                                println!("Waiting for backend to process...");
                                                                // Give backend time to complete the update
                                                                glib::timeout_future_seconds(1).await;
                                                                println!("Calling refresh after promotion...");
                                                            },
                                                            Err(e) => crate::log_error!("Error promoting new head: {}", e),
                                                        }
                                                    }
                                                    
                                                    // Ask about demoting old head
                                                    if let Some(ref old_id) = old_head_id {
                                                        if new_head_id.as_ref() != Some(old_id) {
                                                            // Find old head employee name
                                                            let old_head_name = employees_handler.iter()
                                                                .find(|e| &e.id == old_id)
                                                                .map(|e| format!("{} {}", e.first_name, e.last_name))
                                                                .unwrap_or_else(|| "Previous head".to_string());
                                                            
                                                            let api_demote = api_save.clone();
                                                            let old_id_demote = old_id.clone();
                                                            let refresh_demote = refresh_save.clone();
                                                            
                                                            glib::MainContext::default().spawn_local(async move {
                                                                let dialog = MessageDialog::builder()
                                                                    .transient_for(&window_handler)
                                                                    .modal(true)
                                                                    .message_type(MessageType::Question)
                                                                    .buttons(ButtonsType::YesNo)
                                                                    .text(&format!("Department head changed"))
                                                                    .secondary_text(&format!("Do you want to demote {} from Department Head to Employee?", old_head_name))
                                                                    .build();
                                                                
                                                                dialog.connect_response(move |dialog, response| {
                                                                    let refresh_final = refresh_demote.clone();
                                                                    if response == gtk::ResponseType::Yes {
                                                                        let api_final = api_demote.clone();
                                                                        let id_final = old_id_demote.clone();
                                                                        glib::MainContext::default().spawn_local(async move {
                                                                            let update_req = UpdateEmployeeRequest {
                                                                                role: Some("Employee".to_string()),
                                                                                ..Default::default()
                                                                            };
                                                                            if let Err(e) = api_final.update_employee(&id_final, &update_req).await {
                                                                                crate::log_error!("Error demoting old head: {}", e);
                                                                            }
                                                                            // Refresh after demotion
                                                                            if let Some(ref refresh) = *refresh_final.borrow() {
                                                                                refresh();
                                                                            }
                                                                        });
                                                                    }
                                                                    dialog.close();
                                                                });
                                                                
                                                                dialog.present();
                                                            });
                                                        }
                                                    }
                                                }
                                                
                                                // Always refresh after saving to show updated roles
                                                println!("About to call refresh after department update...");
                                                if let Some(ref refresh) = *refresh_save.borrow() {
                                                    println!("Executing refresh now!");
                                                    refresh();
                                                } else {
                                                    println!("WARNING: Refresh function is None!");
                                                }
                                            });
                                        });
                                    }
                                    Err(e) => {
                                        crate::log_error!("Error fetching employees for edit dialog: {}", e);
                                    }
                                }
                            });
                        });

                        // Delete button handler
                        let dept_id = dept.id.clone();
                        let dept_name = dept.name.clone();
                        let api_delete = api.clone();
                        let refresh_delete = refresh_fn_inner.clone();
                        let window_delete = window_inner.clone();
                        delete_btn.connect_clicked(move |_| {
                            let id = dept_id.clone();
                            let name = dept_name.clone();
                            let api_del = api_delete.clone();
                            let refresh_del = refresh_delete.clone();
                            let window_del = window_delete.clone();
                            
                            // Show confirmation dialog
                            let dialog = MessageDialog::builder()
                                .transient_for(&window_del)
                                .modal(true)
                                .buttons(ButtonsType::YesNo)
                                .message_type(MessageType::Warning)
                                .text(&format!("Delete Department '{}'?", name))
                                .secondary_text("This action cannot be undone. All employees in this department will be unassigned from it. The department head will lose their role.")
                                .build();
                            
                            dialog.connect_response(move |dialog, response| {
                                if response == gtk::ResponseType::Yes {
                                    let id_inner = id.clone();
                                    let api_inner = api_del.clone();
                                    let refresh_inner = refresh_del.clone();
                                    
                                    glib::MainContext::default().spawn_local(async move {
                                        if let Err(e) = api_inner.delete_department(&id_inner).await {
                                            crate::log_error!("Error deleting department: {}", e);
                                        } else {
                                            // Reload list with full details
                                            if let Some(ref refresh) = *refresh_inner.borrow() {
                                                refresh();
                                            }
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
                    crate::log_error!("Error fetching departments or employees");
                }
            }
        });
    });

    // Store the refresh function
    *refresh_fn.borrow_mut() = Some(do_load.clone());

    // Add button handler
    let api_add = api.clone();
    let window_add = window.clone();
    let load_add = do_load.clone();
    add_btn.connect_clicked(move |_| {
        let api_inner = api_add.clone();
        let window_inner = window_add.clone();
        let load_fn = load_add.clone();
        
        // Fetch employees first to show in dropdown
        glib::MainContext::default().spawn_local(async move {
            match api_inner.get_employees(false).await {
                Ok(employees) => {
                    let api_create = api_inner.clone();
                    let load_fn_inner = load_fn.clone();
                    
                    department_dialog::show_create_dialog(&window_inner, employees, move |req, new_head_id| {
                        let api_save = api_create.clone();
                        let load_save = load_fn_inner.clone();
                        glib::MainContext::default().spawn_local(async move {
                            if let Err(e) = api_save.create_department(&req).await {
                                crate::log_error!("Error creating department: {}", e);
                            } else {
                                // Promote new head to "Department Head" if selected
                                if let Some(ref new_id) = new_head_id {
                                    let update_req = UpdateEmployeeRequest {
                                        role: Some("Department Head".to_string()),
                                        ..Default::default()
                                    };
                                    if let Err(e) = api_save.update_employee(new_id, &update_req).await {
                                        crate::log_error!("Error promoting new head: {}", e);
                                    }
                                }
                                
                                // Reload list with full details
                                load_save();
                            }
                        });
                    });
                }
                Err(e) => {
                    crate::log_error!("Error fetching employees: {}", e);
                }
            }
        });
    });

    // Refresh button handler
    let load_refresh = do_load.clone();
    refresh_btn.connect_clicked(move |_| {
        load_refresh();
    });

    // Initial load
    do_load();

    container
}
