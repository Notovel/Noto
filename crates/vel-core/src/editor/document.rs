use std::path::{Path, PathBuf};

#[cfg(feature = "web")]
use noto_protocol::file::File;

use crate::editor::action::{Action, ActionHandler};
use crate::editor::cursor::Cursor;
use crate::editor::history::History;

pub struct Document {
    path: PathBuf,
    data: Vec<u8>,
    cursor: Cursor,
    history: History,
}

impl Document {
    pub fn new(path: &Path) -> Result<Document, std::io::Error> {
        let content: Vec<u8>;

        #[cfg(feature = "web")]
        {
            content = File::get_data(path)?;
        }
        #[cfg(not(feature = "web"))]
        {
            content = std::fs::read(path)?;
        }

        Ok(Document {
            path: path.to_path_buf(),
            data: content,
            cursor: Cursor::new(),
            history: History::new()
        })
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        #[cfg(feature = "web")]
        {
            match File::save_data(self.path, &self.data) {
                Ok(_) => (),
                Err(e) => println!("Error writing file: {}", e),
            }
        }
        #[cfg(not(feature = "web"))]
        {
            match std::fs::write(&self.path, &self.data) {
                Ok(_) => (),
                Err(e) => println!("Error writing file: {}", e),
            }
        }
    }
}

impl ActionHandler for Document {
    fn handle_action(&mut self, action: Action) {
        let worked: bool = match &action {
            Action::Input(ia) => {
                dbg!(ia);
                true
            }
            Action::Delete(_) => {
                unimplemented!();
            }
            Action::Paste(_) => {
                unimplemented!();
            }
            Action::Selection(_) => {
                unimplemented!();
            }
            Action::Move(_) => {
                unimplemented!();
            }
            Action::MoveCursor(_) => {
                unimplemented!();
            },
            Action::MultiCursor(_) => {
                unimplemented!();
            }
            _ => unreachable!("Invalid action: {:?} in document: {}", action, self.path.display())
        };
        if !worked {
            self.history.append(action).expect("Failed to append action to history"); todo!("Correct Error Handling");
        }
    }
}