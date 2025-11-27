use gtk::prelude::*;
use gtk::{Window, Dialog, Entry, Button, Box as GtkBox, Orientation, Label, ResponseType};
use crate::api::models::{SalaryGrade, CreateSalaryGradeRequest, UpdateSalaryGradeRequest};

pub fn show_create_dialog<W, F>(parent: &W, on_save: F)
where
    W: IsA<gtk::Window>,
    F: Fn(CreateSalaryGradeRequest) + 'static,
{
    let dialog = Dialog::builder()
        .title("Create Salary Grade")
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

    // Code field
    let code_label = Label::new(Some("Grade Code:"));
    code_label.set_halign(gtk::Align::Start);
    vbox.append(&code_label);

    let code_entry = Entry::new();
    code_entry.set_placeholder_text(Some("e.g., L1, L2, SR1"));
    vbox.append(&code_entry);

    // Base Salary field
    let salary_label = Label::new(Some("Base Salary:"));
    salary_label.set_halign(gtk::Align::Start);
    salary_label.set_margin_top(8);
    vbox.append(&salary_label);

    let salary_entry = Entry::new();
    salary_entry.set_placeholder_text(Some("Enter base salary"));
    vbox.append(&salary_entry);

    // Description field (optional)
    let desc_label = Label::new(Some("Description (optional):"));
    desc_label.set_halign(gtk::Align::Start);
    desc_label.set_margin_top(8);
    vbox.append(&desc_label);

    let desc_entry = Entry::new();
    desc_entry.set_placeholder_text(Some("Enter description"));
    vbox.append(&desc_entry);

    content.append(&vbox);

    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let code = code_entry.text().to_string();
            let salary_str = salary_entry.text().to_string();
            let description = desc_entry.text().to_string();
            
            if !code.is_empty() && !salary_str.is_empty() {
                if let Ok(base_salary) = salary_str.parse::<f64>() {
                    let req = CreateSalaryGradeRequest {
                        code,
                        base_salary,
                        description: if description.is_empty() { None } else { Some(description) },
                    };
                    on_save(req);
                }
            }
        }
        dialog.close();
    });

    dialog.present();
}

pub fn show_edit_dialog<W, F>(parent: &W, salary_grade: &SalaryGrade, on_save: F)
where
    W: IsA<gtk::Window>,
    F: Fn(String, UpdateSalaryGradeRequest) + 'static,
{
    let dialog = Dialog::builder()
        .title("Edit Salary Grade")
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

    // Code field
    let code_label = Label::new(Some("Grade Code:"));
    code_label.set_halign(gtk::Align::Start);
    vbox.append(&code_label);

    let code_entry = Entry::new();
    code_entry.set_text(&salary_grade.code);
    vbox.append(&code_entry);

    // Base Salary field
    let salary_label = Label::new(Some("Base Salary:"));
    salary_label.set_halign(gtk::Align::Start);
    salary_label.set_margin_top(8);
    vbox.append(&salary_label);

    let salary_entry = Entry::new();
    salary_entry.set_text(&salary_grade.base_salary.to_string());
    vbox.append(&salary_entry);

    // Description field
    let desc_label = Label::new(Some("Description (optional):"));
    desc_label.set_halign(gtk::Align::Start);
    desc_label.set_margin_top(8);
    vbox.append(&desc_label);

    let desc_entry = Entry::new();
    if let Some(ref desc) = salary_grade.description {
        desc_entry.set_text(desc);
    }
    vbox.append(&desc_entry);

    content.append(&vbox);

    let grade_id = salary_grade.id.clone();
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let code = code_entry.text().to_string();
            let salary_str = salary_entry.text().to_string();
            let description = desc_entry.text().to_string();
            
            let base_salary = if salary_str.is_empty() { 
                None 
            } else { 
                salary_str.parse::<f64>().ok() 
            };

            let req = UpdateSalaryGradeRequest {
                code: if code.is_empty() { None } else { Some(code) },
                base_salary,
                description: if description.is_empty() { None } else { Some(description) },
            };
            on_save(grade_id.clone(), req);
        }
        dialog.close();
    });

    dialog.present();
}
