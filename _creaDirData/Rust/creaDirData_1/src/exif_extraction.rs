use exif::{DateTime, Field};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self},
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct FileToMove {
    from: PathBuf,
    to: PathBuf,
}
impl FileToMove {
    fn new_from_path(dir: &Path, file: &PathBuf) -> FileToMove {
        FileToMove {
            from: file.clone(),
            to: dir.join(file),
        }
    }
    fn move_files(&self) {
        match std::fs::rename(&self.from, &self.to) {
            Ok(()) => println!(
                "\t Moved file from: {}, to: {}",
                self.from.display(),
                self.to.display()
            ),
            Err(e) => eprintln!("Error {e} \n\t\t moving file {self}"),
        }
    }
}
impl Display for FileToMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.from.display())
    }
}
struct MyDateTime(exif::DateTime);
impl MyDateTime {
    fn get_dir_name(&self) -> String {
        format!("{:4}_{:0>2}_{:0>2}", self.0.year, self.0.month, self.0.day)
    }
}

#[derive(Default)]
pub struct DirTree {
    //key value PathBuf represent the path of the directory
    directories: HashMap<PathBuf, Vec<FileToMove>>,
}
impl DirTree {
    pub fn new(paths: &[PathBuf], base_path: &Path) -> DirTree {
        let mut dir_tree = DirTree::default();
        for path in paths {
            dir_tree.fill_directories(path, base_path);
        }
        dir_tree
    }
    pub fn move_files(&self) {
        for dir in self.directories.keys() {
            match fs::create_dir(dir) {
                Ok(()) => {
                    println!("Created dir: {}", dir.display());
                }
                Err(e) => eprintln!("Error: {e}, creating dir {}", dir.display()),
            }
            match self.directories.get(dir) {
                Some(files) => {
                    for file in files {
                        file.move_files();
                    }
                }
                None => eprintln!("Failed to retrieve file list of dir: {}", dir.display()),
            }
        }
    }
    fn fill_directories(&mut self, path: &PathBuf, base_path: &Path) {
        if let Some(datetime) = get_datetime(path) {
            let dir_path = base_path.join(datetime.get_dir_name());
            self.add_file(&dir_path, path);
        } else {
            eprintln!("Failed to get datetime tag on file: {}", path.display());
        }
    }
    fn add_file(&mut self, dir: &PathBuf, file: &PathBuf) {
        let file_to_move = FileToMove::new_from_path(dir, file);
        match self.directories.get_mut(dir) {
            Some(v) => {
                v.push(file_to_move);
            }
            None => {
                self.directories.insert(dir.clone(), vec![file_to_move]);
            }
        }
    }
}

fn get_exif_field(path: &PathBuf, tag: exif::Tag) -> Option<Field> {
    match std::fs::File::open(path) {
        Ok(file) => {
            let mut buf_reader = std::io::BufReader::new(&file);
            let exif_reader = exif::Reader::new();
            match exif_reader.read_from_container(&mut buf_reader) {
                Ok(exif) => return exif.get_field(tag, exif::In::PRIMARY).cloned(),
                Err(e) => eprintln!("Error: {e} \n\t\t Invalid format: {}", path.display()),
            }
        }
        Err(e) => eprintln!("Error: {e} \n\t\t opening file: {}", path.display()),
    }
    None
}
fn get_datetime(path: &PathBuf) -> Option<MyDateTime> {
    match get_exif_field(path, exif::Tag::DateTimeOriginal) {
        Some(field) => match &field.value {
            exif::Value::Ascii(ref data) => {
                if !data.is_empty() {
                    match DateTime::from_ascii(&data[0]) {
                        Ok(datetime) => return Some(MyDateTime(datetime)),
                        Err(e) => eprintln!("Error: {e} \n\t\tfile: {}", path.display()),
                    };
                }
            }
            _ => eprintln!("Field do not have the right type"),
        },
        None => {
            eprintln!("File [{}] may not have exif information", path.display());
        }
    }
    None
}
