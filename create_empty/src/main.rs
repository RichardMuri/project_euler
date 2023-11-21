use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

extern crate glob;
use glob::glob;

mod template;

fn get_project_path() -> Option<PathBuf>
{
    // Search PWD for a directory named project_euler
    let project_root = "project_euler";
    let current_path = match current_dir(){
        Ok(result) => result, 
        Err(e) => {println!("Failed to get path to executable: {e}");
                          return None},
    };


    for a in current_path.ancestors(){
        if a.is_dir() && a.exists(){
            let name =  match a.file_name(){
                Some(n) => n,
                None => {
                    println!("Warning, failed to parse filename: {0}", a.to_string_lossy());
                    continue;
                },
            };

            if name == project_root
            {
                return Some(a.to_path_buf());
            }
        }
    }
    None
}

fn main() {
    let project_path =  match get_project_path(){
        Some(p) => {println!("Project base directory: {}", p.display()); p},
        None => {
            println!("Failed to locate project base directory");
            return;
        },
    };

    let problem_path = project_path.join("problems");
    if !problem_path.exists(){
        println!("Error: failed to locate problem directory at path: {}", problem_path.display())
    }

    let glob_expression = problem_path.join("*");
    let mut max_file_num: u64 = 0;
    for f in glob(glob_expression.to_str().unwrap()).expect("Failed to read glob pattern")
    {
        // println!("File: {:?}", f.as_ref().unwrap());
        // println!("{:?}", f.unwrap().file_name().unwrap().to_str().unwrap().parse::<i32>());
        let Ok(file_num) = f.unwrap().file_name().unwrap().to_str().unwrap().parse::<u64>() else{continue;};
        if file_num >= max_file_num{
            max_file_num = file_num + 1;
        }
    }

    println!("Making a directory for problem {max_file_num}.");
    
    let new_dir_name = problem_path.join(max_file_num.to_string());
    match fs::create_dir(&new_dir_name) {
        Ok(_) => (),
        Err(e) => {println!("Error: failed to create directory for problem {max_file_num}: {e}"); return}
    };

    let src_path = new_dir_name.join("src");
    match fs::create_dir(&src_path) {
        Ok(_) => (),
        Err(e) => {println!("Error: failed to create src directory for problem {max_file_num}: {e}"); return}
    };
    
    let binfile_name = src_path.join("bin.rs");
    match fs::write(binfile_name, template::bin_template(max_file_num)){
        Ok(_) => (),
        Err(e) => {println!("Error: failed to write binfile for problem {max_file_num}: {e}"); return}
    };

    let lib_name = src_path.join("lib.rs");
    match fs::write(lib_name, template::lib_template(max_file_num)){
        Ok(_) => (),
        Err(e) => {println!("Error: failed to write lib for problem {max_file_num}: {e}"); return}
    };

    let cargo_name = new_dir_name.join("Cargo.toml");
    match fs::write(cargo_name, template::cargo_template(max_file_num)){
        Ok(_) => (),
        Err(e) => {println!("Error: failed to write Cargo.toml for problem {max_file_num}: {e}"); return}
    };
}
