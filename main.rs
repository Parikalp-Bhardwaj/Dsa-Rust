use std::fs;
use std::io::{Write,Read};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let get_files = fs::read_dir("./");
    let main = String::from("./main.rs");
    match get_files{
        Ok(files) => {
            for entry in files{
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        let file_name = entry.file_name();
                        let trans_files = file_name.to_string_lossy();
                        if trans_files != main{
                         
                            let folder_str = trans_files.replace(".rs","");
                            let folder_name = format!("./{}/",folder_str);
                            fs::create_dir(&folder_name)?;
            
                            let read_file_path = format!("./{}",trans_files);
                            let new_file_path = format!("{}{}",folder_name,trans_files);
    
                            let mut file_content = Vec::new();
                            fs::File::open(&read_file_path)?
                                .read_to_end(&mut file_content)?;

                            // Write the contents to the new file location
                            let mut new_file = fs::File::create(&new_file_path)?;
                            new_file.write_all(&file_content)?;

                            fs::remove_file(read_file_path)?;

                            println!("File moved to {}", new_file_path);
                        }
                    }
                };
            }
            Ok(())
        }
        Err(err) => {
            println!("errror {}",err);
            Err(err.into())
        }
    }

}