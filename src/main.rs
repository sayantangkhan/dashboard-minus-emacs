use dashboard_minus_emacs::*;
use std::path::Path;

fn main() -> Result<(), markdown::message::Message> {
    let parsed = read_markdown_from_path(&Path::new(
        "/home/sayantan/Sync/Rust/Code/dashboard-minus-emacs/test-file.md",
    ))
    .unwrap();
    indented_tree_print(&parsed, 0);

    Ok(())
}
