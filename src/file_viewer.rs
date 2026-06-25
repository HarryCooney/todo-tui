use ratatui::widgets::{ListItem, ListState};
use std::fs::{self, File, DirEntry};
use color_eyre::Result;
use std::path::Path;

const FILE_PATH:&str = "./lists";

#[derive(Default, Debug)]
pub struct File_Viewer {
    //This might change later to Vec<DirEntry> This would provide more flexibility
    pub files: Vec<String>,
    pub state: ListState
}

//Mouning file system
//Create the folder in local/share if it doesn't exist
//Otherwise, return OK
impl File_Viewer {
    fn mount_storage() -> Result<()> {
        let path = Path::new(FILE_PATH);
        if !path.exists() {
            fs::create_dir(FILE_PATH)?;
        }
        Ok(())
    }
    pub fn read_storage(&mut self) -> Result<()> {
        File_Viewer::mount_storage()?;
        let path = Path::new(FILE_PATH);
        let dir_ents = path.read_dir()?;
        //Vector need to be cleared before items are read into it. Otherwise, duplicates appear
        self.files.clear();
        for ent in dir_ents {
            if let Ok(ent) = ent {
                self.add_file(&ent)?;
            }
        }
        Ok(())
    }

    fn add_file(&mut self, ent: &DirEntry) -> Result<()> {
        if ent.file_type()?.is_file() {
            self.files.push(ent.file_name().into_string().unwrap());
        }
        Ok(())
    }
}

