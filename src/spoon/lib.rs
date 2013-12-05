#[desc = "File-based issue tracking"];
#[license = "GPL"];

pub struct Issue {
    id: uint,
    name: ~str,
    assigned_to: Option<~str>,
    label: ~str,
    status: ~str,
    description: ~str,
    created_at: uint
}

pub fn create_issue(name: ~str, label: ~str, description: ~str) -> Issue {
    Issue {
        id: 0,
        name: name,
        assigned_to: None,
        label: label,
        status: ~"Open",
        description: description,
        created_at: 0
    }
}

