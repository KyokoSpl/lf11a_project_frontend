use gtk::prelude::*;
use gtk::{Window, Dialog, Entry, Button, Box as GtkBox, Orientation, Label, ResponseType, CheckButton, DropDown, StringList};
use crate::api::models::{Employee, CreateEmployeeRequest, UpdateEmployeeRequest, Department, SalaryGrade};

pub fn show_create_dialog<W, F>(parent: &W, departments: Vec<Department>, employees: Vec<Employee>, salary_grades: Vec<SalaryGrade>, on_save: F)
where
    W: IsA<gtk::Window>,
    F: Fn(CreateEmployeeRequest) + 'static,
{
    let dialog = Dialog::builder()
        .title("Create Employee")
        .transient_for(parent)
        .modal(true)
        .default_width(400)
        .build();

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Create", ResponseType::Accept);

    let content = dialog.content_area();
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

    // Department
    vbox.append(&Label::builder().label("Department (optional):").halign(gtk::Align::Start).build());
    let dept_entry = Entry::new();
    dept_entry.set_placeholder_text(Some("Enter department ID"));
    vbox.append(&dept_entry);

    // Manager
    vbox.append(&Label::builder().label("Manager ID (optional):").halign(gtk::Align::Start).build());
    let manager_entry = Entry::new();
    manager_entry.set_placeholder_text(Some("Enter manager ID"));
    vbox.append(&manager_entry);

    // Salary Grade
    vbox.append(&Label::builder().label("Salary Grade ID (optional):").halign(gtk::Align::Start).build());
    let salary_entry = Entry::new();
    salary_entry.set_placeholder_text(Some("Enter salary grade ID"));
    vbox.append(&salary_entry);

    // Hire Date
    vbox.append(&Label::builder().label("Hire Date (optional):").halign(gtk::Align::Start).build());
    let hire_date_entry = Entry::new();
    hire_date_entry.set_placeholder_text(Some("YYYY-MM-DD"));
    vbox.append(&hire_date_entry);

    content.append(&vbox);

    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let first_name = first_name_entry.text().to_string();
            let last_name = last_name_entry.text().to_string();
            let email = email_entry.text().to_string();
            let role = role_entry.text().to_string();
            let dept_id = dept_entry.text().to_string();
            let manager_id = manager_entry.text().to_string();
            let salary_id = salary_entry.text().to_string();
            let hire_date = hire_date_entry.text().to_string();
            
            if !first_name.is_empty() && !last_name.is_empty() && !email.is_empty() {
                let req = CreateEmployeeRequest {
                    first_name,
                    last_name,
                    email,
                    role: if role.is_empty() { None } else { Some(role) },
                    department_id: if dept_id.is_empty() { None } else { Some(dept_id) },
                    manager_id: if manager_id.is_empty() { None } else { Some(manager_id) },
                    salary_grade_id: if salary_id.is_empty() { None } else { Some(salary_id) },
                    hire_date: if hire_date.is_empty() { None } else { Some(hire_date) },
                };
                on_save(req);
            }
        }
        dialog.close();
    });

    dialog.present();
}

pub fn show_edit_dialog<W, F>(parent: &W, employee: &Employee, on_save: F)
where
    W: IsA<gtk::Window>,
    F: Fn(String, UpdateEmployeeRequest) + 'static,
{
    let dialog = Dialog::builder()
        .title("Edit Employee")
        .transient_for(parent)
        .modal(true)
        .default_width(400)
        .build();

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Save", ResponseType::Accept);

    let content = dialog.content_area();
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

    // Active
    let active_check = CheckButton::with_label("Active");
    active_check.set_active(employee.active);
    vbox.append(&active_check);

    content.append(&vbox);

    let emp_id = employee.id.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let first_name = first_name_entry.text().to_string();
            let last_name = last_name_entry.text().to_string();
            let email = email_entry.text().to_string();
            let role = role_entry.text().to_string();
            let active = active_check.is_active();
            
            let req = UpdateEmployeeRequest {
                first_name: if first_name.is_empty() { None } else { Some(first_name) },
                last_name: if last_name.is_empty() { None } else { Some(last_name) },
                email: if email.is_empty() { None } else { Some(email) },
                role: if role.is_empty() { None } else { Some(role) },
                active: Some(active),
                department_id: None,
                manager_id: None,
                salary_grade_id: None,
                hire_date: None,
            };
            on_save(emp_id.clone(), req);
        }
        dialog.close();
    });

    dialog.present();
}
