use std::io;
use std::process;
use std::env;

fn main() {
    let stdin = io::stdin();

    let file_path = std::env::args().nth(1).expect("usage: exec <file_path>");

    let mut text_editor = match line_editor::TextEditor::new(file_path) {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("ERR initializing editor: {e}");
            return;
        }
    };

    text_editor.execute(&io::stdin());
}
