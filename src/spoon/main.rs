use file::write_to_file;
use lib::create_issue;

mod lib;
mod file;

fn main() {
    let issue = create_issue(~"test issue", ~"feature", ~"yolo");
    write_to_file(issue);
}
