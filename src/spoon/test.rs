#![license = "MIT"]

use spoon = lib;
use spoon::create_issue;
mod lib;

#[test]
fn test_create_issue() {
    let issue = create_issue(~"Test issue", ~"feature", ~"just a test issue");

    // Explicitly set data
    assert!(issue.name == ~"Test issue");
    assert!(issue.label == ~"feature");
    assert!(issue.description == ~"just a test issue");

    // Implicit static data
    assert!(issue.assigned_to == ~None);
    assert!(issue.status == ~"Open");
}

