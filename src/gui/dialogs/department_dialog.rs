use gtk::prelude::*;
use gtk::{Dialog, Entry, Box as GtkBox, Orientation, Label, ResponseType, DropDown, StringList};
use crate::api::models::{Department, Employee, CreateDepartmentRequest, UpdateDepartmentRequest};

pub fn show_create_dialog<W, F>(parent: &W, employees: Vec<Employee>, on_save: F)
where
    W: IsA<gtk::Window>,
    F: Fn(CreateDepartmentRequest, Option<String>) + 'static,
{
    let dialog = Dialog::builder()
        .title("Create Department")
        .transient_for(parent)
        .modal(true)
        .build();

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Create", ResponseType::Accept);

    let content = dialog.content_area();
    let vbox = GtkBox::new(Orientation::Vertical, 12);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);

    // Name field
    let name_label = Label::new(Some("Department Name:"));
    name_label.set_halign(gtk::Align::Start);
    vbox.append(&name_label);

    let name_entry = Entry::new();
    name_entry.set_placeholder_text(Some("Enter department name"));
    vbox.append(&name_entry);

    // Head Employee dropdown
    let head_label = Label::new(Some("Head Employee (optional):"));
    head_label.set_halign(gtk::Align::Start);
    head_label.set_margin_top(8);
    vbox.append(&head_label);

    // Create string list with employee names
    let mut options = vec!["None".to_string()];
    let mut employee_ids = vec![None];
    
    for emp in employees.iter() {
        let display_name = format!("{} {} ({})", emp.first_name, emp.last_name, emp.role);
        options.push(display_name);
        employee_ids.push(Some(emp.id.clone()));
    }
    
    let string_list = StringList::new(&options.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    let dropdown = DropDown::new(Some(string_list), None::<gtk::Expression>);
    dropdown.set_selected(0); // Default to "None"
    vbox.append(&dropdown);

    content.append(&vbox);

    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let name = name_entry.text().to_string();
            let selected = dropdown.selected() as usize;
            let head_id = employee_ids.get(selected).and_then(|id| id.clone());
            
            if !name.is_empty() {
                let req = CreateDepartmentRequest {
                    name: name.clone(),
                    head_id: head_id.clone(),
                };
                on_save(req, head_id);
            }
        }
        dialog.close();
    });

    dialog.present();
}

pub fn show_edit_dialog<W, F>(parent: &W, department: &Department, employees: Vec<Employee>, on_save: F)
where
    W: IsA<gtk::Window>,
    F: Fn(String, UpdateDepartmentRequest, Option<String>, Option<String>) + 'static,
{
    let dialog = Dialog::builder()
        .title("Edit Department")
        .transient_for(parent)
        .modal(true)
        .build();

    dialog.add_button("Cancel", ResponseType::Cancel);
    dialog.add_button("Save", ResponseType::Accept);

    let content = dialog.content_area();
    let vbox = GtkBox::new(Orientation::Vertical, 12);
    vbox.set_margin_top(12);
    vbox.set_margin_bottom(12);
    vbox.set_margin_start(12);
    vbox.set_margin_end(12);

    // Name field
    let name_label = Label::new(Some("Department Name:"));
    name_label.set_halign(gtk::Align::Start);
    vbox.append(&name_label);

    let name_entry = Entry::new();
    name_entry.set_text(&department.name);
    vbox.append(&name_entry);

    // Head Employee dropdown
    let head_label = Label::new(Some("Head Employee (optional):"));
    head_label.set_halign(gtk::Align::Start);
    head_label.set_margin_top(8);
    vbox.append(&head_label);

    // Create string list with employee names
    let mut options = vec!["None".to_string()];
    let mut employee_ids = vec![None];
    let mut selected_index = 0;
    
    for (idx, emp) in employees.iter().enumerate() {
        let display_name = format!("{} {} ({})", emp.first_name, emp.last_name, emp.role);
        options.push(display_name);
        employee_ids.push(Some(emp.id.clone()));
        
        // Check if this is the current head
        if let Some(ref head_id) = department.head_id {
            if &emp.id == head_id {
                selected_index = idx + 1;
            }
        }
    }
    
    let string_list = StringList::new(&options.iter().map(|s| s.as_str()).collect::<Vec<_>>());
    let dropdown = DropDown::new(Some(string_list), None::<gtk::Expression>);
    dropdown.set_selected(selected_index as u32);
    vbox.append(&dropdown);

    content.append(&vbox);

    let dept_id = department.id.clone();
    let old_head_id = department.head_id.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let name = name_entry.text().to_string();
            let selected = dropdown.selected() as usize;
            let new_head_id = employee_ids.get(selected).and_then(|id| id.clone());
            
            let req = UpdateDepartmentRequest {
                name: if name.is_empty() { None } else { Some(name) },
                head_id: new_head_id.clone(),
            };
            on_save(dept_id.clone(), req, old_head_id.clone(), new_head_id);
        }
        dialog.close();
    });

    dialog.present();
}
