#[desc = "File-based issue tracking"];
#[license = "GPL"];

extern mod extra;

//use extra::uuid::Uuid;

pub struct Issue {
    id: extra::uuid::Uuid,
    name: ~str,
    assigned_to: ~Option<~str>,
    label: ~str,
    status: ~str,
    description: ~str,
    created_at: uint
}

pub fn create_issue(name: ~str, label: ~str, description: ~str) -> ~Issue {
    let uuid = extra::uuid::Uuid::new_v4();
    ~Issue {
        id: uuid,
        name: name,
        assigned_to: ~None,
        label: label,
        status: ~"Open",
        description: description,
        created_at: 0
    }
}
