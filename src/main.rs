use std::io::Read;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut files: Vec<String> = vec![];
    let mut lines_of_code: u64 = 0;

    for entry in WalkDir::new(".\\").into_iter().filter_map(|e| e.ok()) {
        // if the directory is node_modules, skip it
        if entry.file_name().to_str().unwrap().contains("node_modules") {
            continue;
        }
        // or bin
        if entry.file_name().to_str().unwrap().contains("bin") {
            continue;
        }
        // or .git
        if entry.file_name().to_str().unwrap().contains(".git") {
            continue;
        }
        
        for arg in &args {
            if entry.path().to_str().unwrap().ends_with(arg) {
                files.push(entry.path().to_str().unwrap().to_string());
            }
        }
    }

    for file in files {
        let mut file = std::fs::File::open(file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        lines_of_code += contents.lines().count() as u64;
    }

    println!("Lines of code: {}", lines_of_code);
}
