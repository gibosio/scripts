/*
    2024/11/19 GB
    split photo files based on exif Tag DateTimeOriginal
*/

mod exif_extraction;

use std::{
    fs::DirEntry,
    path::{Path, PathBuf},
};

use exif_extraction::DirTree;

fn main() {
    let base_path = Path::new(".");
    println!("Searching dir entry in path: {}...", base_path.display());

    //getting the list of path in the path dir
    let list_of_ph_path: Vec<PathBuf> = get_list_of_ph_path(base_path);

    println!("Searching for exif data...");
    let dir_tree = DirTree::new(&list_of_ph_path, base_path);

    dir_tree.move_files();
}

fn get_list_of_ph_path(path: &Path) -> Vec<PathBuf> {
    fn get_list_of_file(path: &Path) -> Vec<DirEntry> {
        let list_of_file = std::fs::read_dir(path).expect("Error opening path: {path}");
        list_of_file.flatten().collect()
    }
    fn match_extension(ext: &str) -> bool {
        matches!(ext, "jpg" | "nef" | "JPG" | "NEF")
    }
    fn get_extension(filepath: &PathBuf) -> String {
        filepath
            .extension()
            .expect("{file:?} not recognized")
            .to_os_string()
            .into_string()
            .unwrap_or_else(|_| panic!("Invalid UTF-8 conversion: {filepath:?}"))
    }

    let list_of_dir_entry = get_list_of_file(path);
    //filter directory and symbolic link
    let list_of_file: Vec<DirEntry> = list_of_dir_entry
        .into_iter()
        .filter(|entry| {
            if let Ok(file_type) = entry.file_type() {
                file_type.is_file()
            } else {
                false
            }
        })
        .collect();

    let list_of_filepath: Vec<PathBuf> = list_of_file.into_iter().map(|file| file.path()).collect();

    return list_of_filepath
        .into_iter()
        .filter(|file| match_extension(get_extension(file).as_str()))
        .collect();
}
