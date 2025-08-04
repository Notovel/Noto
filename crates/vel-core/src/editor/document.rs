use std::path::{Path, PathBuf};

#[cfg(feature = "web")]
use noto_protocol::file::File;

use crate::editor::action::{Action, ActionHandler};
use crate::editor::cursor::Cursor;

pub struct Document {
    path: PathBuf,
    data: Vec<u8>,
    cursor: Cursor,
}

impl Document {
    pub fn new(path: &Path) -> Result<Document, std::io::Error> {
        let content: Vec<u8>;

        //#[cfg(feature = "web")]
        //{
        //    content = File::get_data(path)?;
        //}
        #[cfg(not(feature = "web"))]
        {
            content = std::fs::read(path)?;
        }

        Ok(Document {
            path: path.to_path_buf(),
            data: content,
            cursor: Cursor::new()
        })
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        //#[cfg(feature = "web")]
        //{
        //    match File::save_data(self.path, &self.data) {
        //        Ok(_) => (),
        //        Err(e) => println!("Error writing file: {}", e),
        //    }
        //}
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
    fn handle(&self, action: &Action) -> Result<(), bool> {
        match action {
            Action::Input(ia) => {
                dbg!(ia);
            }
            Action::Delete(_) => {

            }
            Action::Paste(_) => {

            }
            Action::Selection(_) => {

            }
            Action::Move(_) => {

            }
            Action::MoveCursor(_) => {

            },
            &Action::MultiCursor(_) => todo!()
        }
        Ok(())
    }
}