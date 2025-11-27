use gtk::prelude::*;
use gtk::{Dialog, Entry, Box as GtkBox, Orientation, Label, ResponseType, CheckButton, DropDown, StringList, ScrolledWindow};
use crate::api::models::{Employee, CreateEmployeeRequest, UpdateEmployeeRequest, Department, SalaryGrade};

pub fn show_create_dialog<W, F>(parent: &W, departments: Vec<Department>, _employees: Vec<Employee>, salary_grades: Vec<SalaryGrade>, on_save: F)
where
    W: IsA<gtk::Window>,
    F: Fn(CreateEmployeeRequest) + 'static,
{
    let dialog = Dialog::builder()
        .title("Create Employee")
        .transient_for(parent)
        .modal(true)
        .default_width(500)
        .default_height(650)
        .build();

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Create", ResponseType::Accept);

    let content = dialog.content_area();
    
    // Scrollable container
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_height(550)
        .build();
    
    let vbox = GtkBox::new(Orientation::Vertical, 8);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);

    // First Name
    vbox.append(&Label::builder().label("First Name:").halign(gtk::Align::Start).build());
    let first_name_entry = Entry::new();
    first_name_entry.set_placeholder_text(Some("Enter first name"));
    vbox.append(&first_name_entry);

    // Last Name
    vbox.append(&Label::builder().label("Last Name:").halign(gtk::Align::Start).build());
    let last_name_entry = Entry::new();
    last_name_entry.set_placeholder_text(Some("Enter last name"));
    vbox.append(&last_name_entry);

    // Email
    vbox.append(&Label::builder().label("Email:").halign(gtk::Align::Start).build());
    let email_entry = Entry::new();
    email_entry.set_placeholder_text(Some("email@example.com"));
    vbox.append(&email_entry);

    // Role
    vbox.append(&Label::builder().label("Role:").halign(gtk::Align::Start).build());
    let role_entry = Entry::new();
    role_entry.set_placeholder_text(Some("e.g., Software Engineer"));
    vbox.append(&role_entry);

    // Department Dropdown
    vbox.append(&Label::builder().label("Department (optional):").halign(gtk::Align::Start).build());
    let dept_list = StringList::new(&[]);
    dept_list.append("-- None --");
    let dept_ids: Vec<String> = departments.iter().map(|d| d.id.clone()).collect();
    for dept in &departments {
        dept_list.append(&dept.name);
    }
    let dept_dropdown = DropDown::new(Some(dept_list), None::<gtk::Expression>);
    dept_dropdown.set_selected(0);
    vbox.append(&dept_dropdown);

    // Manager
    vbox.append(&Label::builder().label("Manager ID (optional):").halign(gtk::Align::Start).build());
    let manager_entry = Entry::new();
    manager_entry.set_placeholder_text(Some("Enter manager ID"));
    vbox.append(&manager_entry);

    // Salary Grade Dropdown with scrollable list
    vbox.append(&Label::builder().label("Salary Grade (optional):").halign(gtk::Align::Start).build());
    let mut salary_strings = vec!["-- None --".to_string()];
    let salary_ids: Vec<String> = salary_grades.iter().map(|s| s.id.clone()).collect();
    for grade in &salary_grades {
        let desc = grade.description.as_deref().unwrap_or("No description");
        salary_strings.push(format!("{} - ${} - {}", 
            grade.code, 
            grade.base_salary,
            desc
        ));
    }
    let salary_strs: Vec<&str> = salary_strings.iter().map(|s| s.as_str()).collect();
    let salary_list = StringList::new(&salary_strs);
    let salary_dropdown = DropDown::builder()
        .model(&salary_list)
        .build();
    salary_dropdown.set_selected(0);
    
    // Make the salary dropdown list show more items
    if let Some(popover) = salary_dropdown.last_child() {
        popover.set_height_request(400);
    }
    
    vbox.append(&salary_dropdown);

    // Hire Date
    vbox.append(&Label::builder().label("Hire Date (optional):").halign(gtk::Align::Start).build());
    let hire_date_entry = Entry::new();
    hire_date_entry.set_placeholder_text(Some("YYYY-MM-DD"));
    vbox.append(&hire_date_entry);

    scrolled.set_child(Some(&vbox));
    content.append(&scrolled);

    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let first_name = first_name_entry.text().to_string();
            let last_name = last_name_entry.text().to_string();
            let email = email_entry.text().to_string();
            let role = role_entry.text().to_string();
            
            // Get selected department ID
            let dept_id = {
                let selected = dept_dropdown.selected();
                if selected == 0 {
                    None
                } else {
                    Some(dept_ids[(selected - 1) as usize].clone())
                }
            };
            
            let manager_id = manager_entry.text().to_string();
            
            // Get selected salary grade ID
            let salary_id = {
                let selected = salary_dropdown.selected();
                if selected == 0 {
                    None
                } else {
                    Some(salary_ids[(selected - 1) as usize].clone())
                }
            };
            
            let hire_date = hire_date_entry.text().to_string();
            
            if !first_name.is_empty() && !last_name.is_empty() && !email.is_empty() {
                let req = CreateEmployeeRequest {
                    first_name,
                    last_name,
                    email,
                    role: if role.is_empty() { None } else { Some(role) },
                    department_id: dept_id,
                    manager_id: if manager_id.is_empty() { None } else { Some(manager_id) },
                    salary_grade_id: salary_id,
                    hire_date: if hire_date.is_empty() { None } else { Some(hire_date) },
                };
                on_save(req);
            }
        }
        dialog.close();
    });

    dialog.present();
}

pub fn show_edit_dialog<W, F>(parent: &W, employee: &Employee, departments: Vec<Department>, salary_grades: Vec<SalaryGrade>, on_save: F)
where
    W: IsA<gtk::Window>,
    F: Fn(String, UpdateEmployeeRequest) + 'static,
{
    let dialog = Dialog::builder()
        .title("Edit Employee")
        .transient_for(parent)
        .modal(true)
        .default_width(500)
        .default_height(600)
        .build();

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Save", ResponseType::Accept);

    let content = dialog.content_area();
    
    // Scrollable container
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .min_content_height(500)
        .build();
    
    let vbox = GtkBox::new(Orientation::Vertical, 8);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);

    // First Name
    vbox.append(&Label::builder().label("First Name:").halign(gtk::Align::Start).build());
    let first_name_entry = Entry::new();
    first_name_entry.set_text(&employee.first_name);
    vbox.append(&first_name_entry);

    // Last Name
    vbox.append(&Label::builder().label("Last Name:").halign(gtk::Align::Start).build());
    let last_name_entry = Entry::new();
    last_name_entry.set_text(&employee.last_name);
    vbox.append(&last_name_entry);

    // Email
    vbox.append(&Label::builder().label("Email:").halign(gtk::Align::Start).build());
    let email_entry = Entry::new();
    email_entry.set_text(&employee.email);
    vbox.append(&email_entry);

    // Role
    vbox.append(&Label::builder().label("Role:").halign(gtk::Align::Start).build());
    let role_entry = Entry::new();
    role_entry.set_text(&employee.role);
    vbox.append(&role_entry);

    // Department Dropdown
    vbox.append(&Label::builder().label("Department (optional):").halign(gtk::Align::Start).build());
    let mut dept_strings = vec!["-- None --".to_string()];
    let dept_ids: Vec<String> = departments.iter().map(|d| d.id.clone()).collect();
    for dept in &departments {
        dept_strings.push(dept.name.clone());
    }
    let dept_strs: Vec<&str> = dept_strings.iter().map(|s| s.as_str()).collect();
    let dept_list = StringList::new(&dept_strs);
    let dept_dropdown = DropDown::new(Some(dept_list), None::<gtk::Expression>);
    
    // Select current department
    if let Some(ref dept_id) = employee.department_id {
        if let Some(pos) = dept_ids.iter().position(|id| id == dept_id) {
            dept_dropdown.set_selected((pos + 1) as u32);
        }
    } else {
        dept_dropdown.set_selected(0);
    }
    vbox.append(&dept_dropdown);

    // Salary Grade Dropdown
    vbox.append(&Label::builder().label("Salary Grade (optional):").halign(gtk::Align::Start).build());
    let mut salary_strings = vec!["-- None --".to_string()];
    let salary_ids: Vec<String> = salary_grades.iter().map(|s| s.id.clone()).collect();
    for grade in &salary_grades {
        let desc = grade.description.as_deref().unwrap_or("No description");
        salary_strings.push(format!("{} - ${} - {}", 
            grade.code, 
            grade.base_salary,
            desc
        ));
    }
    let salary_strs: Vec<&str> = salary_strings.iter().map(|s| s.as_str()).collect();
    let salary_list = StringList::new(&salary_strs);
    let salary_dropdown = DropDown::builder()
        .model(&salary_list)
        .build();
    
    // Select current salary grade
    if let Some(ref salary_id) = employee.salary_grade_id {
        if let Some(pos) = salary_ids.iter().position(|id| id == salary_id) {
            salary_dropdown.set_selected((pos + 1) as u32);
        }
    } else {
        salary_dropdown.set_selected(0);
    }
    
    // Make the salary dropdown list show more items
    if let Some(popover) = salary_dropdown.last_child() {
        popover.set_height_request(400);
    }
    vbox.append(&salary_dropdown);

    // Active
    let active_check = CheckButton::with_label("Active");
    active_check.set_active(employee.active);
    vbox.append(&active_check);

    scrolled.set_child(Some(&vbox));
    content.append(&scrolled);

    let emp_id = employee.id.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let first_name = first_name_entry.text().to_string();
            let last_name = last_name_entry.text().to_string();
            let email = email_entry.text().to_string();
            let role = role_entry.text().to_string();
            let active = active_check.is_active();
            
            // Get department from dropdown
            let department_id = {
                let selected = dept_dropdown.selected();
                if selected == 0 {
                    None
                } else {
                    Some(dept_ids.get((selected - 1) as usize).unwrap().clone())
                }
            };

            // Get salary grade from dropdown
            let salary_grade_id = {
                let selected = salary_dropdown.selected();
                if selected == 0 {
                    None
                } else {
                    Some(salary_ids.get((selected - 1) as usize).unwrap().clone())
                }
            };
            
            let req = UpdateEmployeeRequest {
                first_name: if first_name.is_empty() { None } else { Some(first_name) },
                last_name: if last_name.is_empty() { None } else { Some(last_name) },
                email: if email.is_empty() { None } else { Some(email) },
                role: if role.is_empty() { None } else { Some(role) },
                active: Some(active),
                department_id,
                manager_id: None,
                salary_grade_id,
                hire_date: None,
            };
            on_save(emp_id.clone(), req);
        }
        dialog.close();
    });

    dialog.present();
}
