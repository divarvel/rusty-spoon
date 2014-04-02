#![desc = "File-based issue tracking"];
#[license = "GPL"]


extern crate uuid;

//use uuid::Uuid;

pub struct Issue {
    id: uuid::Uuid,
    name: ~str,
    assigned_to: ~Option<~str>,
    label: ~str,
    status: ~str,
    description: ~str,
    created_at: uint
}

pub fn create_issue(name: ~str, label: ~str, description: ~str) -> ~Issue {
    let uuid = uuid::Uuid::new_v4();
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

pub fn print_issue_line(issue: &Issue) -> () {
    println(format!("{} {} {} ({})", issue.status, issue.label, issue.name, issue.assigned_to.clone().unwrap_or(~"nobody")));
}

pub fn print_issue_list(issues: ~[Issue]) -> () {
    for issue in issues.iter() {
        print_issue_line(issue);
    }
}
