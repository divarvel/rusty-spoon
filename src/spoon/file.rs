use lib::Issue;
use std::io::File;

mod lib;

pub fn write_to_file(i: &Issue) {
    let file_name = ~"issues/" + i.id.to_str() + ".md";
    let file = File::create(&Path::new(file_name));

    match file {
        Some(mut f) => {
            let date = i.created_at.to_str();
            let title_line = ~"# " + i.name + "\n\n";

            f.write(bytes!("---\n"));

            f.write(bytes!("status: "));
            f.write(i.status.as_bytes());
            f.write(bytes!("\n"));

            f.write(bytes!("label: "));
            f.write(i.label.as_bytes());
            f.write(bytes!("\n"));

            match i.assigned_to.clone() {
                ~Some(a) => {
                    f.write(bytes!("assignee: "));
                    f.write(a.as_bytes());
                    f.write(bytes!("\n"));
                }
                ~None => ()
            }


            f.write(bytes!("created: "));
            f.write(date.as_bytes());
            f.write(bytes!("\n"));


            f.write(bytes!("---\n\n"));

            f.write(title_line.as_bytes());
            f.write(i.description.as_bytes());
        }
        None => {
            println("file error");
        }
    }
}
