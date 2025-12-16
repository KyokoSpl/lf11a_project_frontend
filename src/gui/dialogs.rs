//! Dialog types for confirmation and edit/create dialogs

/// Types of confirmation dialogs
#[derive(Clone)]
pub enum ConfirmAction {
    DeleteDepartment {
        id: String,
        name: String,
        employee_count: usize,
    },
    DeleteEmployee {
        id: String,
        name: String,
    },
    DeleteSalaryGrade {
        id: String,
        code: String,
        employee_count: usize,
    },
    UpdateDepartment {
        id: String,
        name: String,
        old_head_id: Option<String>,
        old_head_name: Option<String>,
        new_head_id: Option<String>,
        new_head_name: Option<String>,
    },
    UpdateEmployee {
        id: String,
        name: String,
    },
    UpdateSalaryGrade {
        id: String,
        code: String,
    },
    CreateDepartment {
        name: String,
    },
    CreateEmployee {
        name: String,
    },
    CreateSalaryGrade {
        code: String,
    },
}

/// Types of edit/create dialogs
#[derive(Clone)]
pub enum EditDialog {
    CreateDepartment,
    EditDepartment {
        id: String,
        old_head_id: Option<String>,
    },
    CreateEmployee,
    EditEmployee {
        id: String,
    },
    CreateSalaryGrade,
    EditSalaryGrade {
        id: String,
    },
}
