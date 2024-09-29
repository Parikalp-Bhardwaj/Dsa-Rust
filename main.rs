use std::fs;
use std::io::{Read, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let get_files = fs::read_dir("./")?;

    for entry in get_files {
        if let Ok(entry) = entry {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                let file_name = entry.file_name();
                let trans_files = file_name.to_string_lossy();

                if trans_files == ".gitignore" || trans_files == "main.rs" {
                    continue;
                }

                if trans_files.ends_with(".rs") {
                    let folder_str = trans_files.replace(".rs", "");
                    let folder_name = format!("./{}/", folder_str);

                    println!("Attempting to create directory: {}", folder_name);

                    if Path::new(&folder_name).exists() {
                        println!("Directory {} already exists.", folder_name);
                    } else {
                        match fs::create_dir(&folder_name) {
                            Ok(_) => println!("Directory {} created successfully.", folder_name),
                            Err(e) => {
                                eprintln!("Failed to create directory {}: {}", folder_name, e);
                                continue; 
                            }
                        }
                    }

                    let read_file_path = format!("./{}", trans_files);
                    let new_file_path = format!("{}{}", folder_name, trans_files);


                    println!("Reading from: {}", read_file_path);
                    if Path::new(&read_file_path).exists() {

                        let mut file_content = Vec::new();
                        fs::File::open(&read_file_path)?.read_to_end(&mut file_content)?;

                        let mut new_file = fs::File::create(&new_file_path)?;
                        new_file.write_all(&file_content)?;

                      
                        fs::remove_file(&read_file_path)?;

                        println!("File moved to {}", new_file_path);
                    } else {
                        eprintln!("File not found: {}", read_file_path);
                    }
                }
            }
        }
    }
    Ok(())
}
