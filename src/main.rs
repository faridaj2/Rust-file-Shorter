#![windows_subsystem = "windows"]
slint::include_modules!();

use std::{fs::{self}, path::Path};

use rfd::FileDialog;
use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_browse_src({
        let ui = ui_handle.clone();
        move || {
            let folder = FileDialog::new().pick_folder();

            if let Some(path) = folder{
                let p_string = path.display().to_string();
                ui.unwrap().set_src_folder(p_string.into());
            }
        }
    });
    
    ui.on_browse_dst({
        let ui = ui_handle.clone();
        move || {
            let folder = FileDialog::new().pick_folder();
            if let Some(path) = folder{
                let p_string = path.display().to_string();
                ui.unwrap().set_dst_folder(p_string.into());
            }
        }
    });
    ui.on_process_files({
        let ui = ui_handle.clone();
        move || {
            let uix = ui.unwrap();
            let src = uix.get_src_folder().to_string();
            let dst = uix.get_dst_folder().to_string();

            let mut success = 0;
            let mut fail = 0;

            let mut files_in_folder: Vec<String> = vec![];
            let format = uix.get_format_type();

            let mut log = String::from("-------------- Proses -----------\n");

            if let Ok(file) = fs::read_dir(&src){
                for entry in file.flatten(){
                    files_in_folder.push(entry.file_name().to_string_lossy().to_string());
                }
            }
            let list_file = uix.get_filenames();
            for target in list_file.lines(){
                let target = format!("{}.{}", target,format.to_string().to_lowercase());

                for data in &files_in_folder{
                    if data.contains(&target){
                        let src_file = Path::new(&src).join(&data);
                        let dst_file = Path::new(&dst).join(&data);
                        if fs::rename(src_file, dst_file).is_ok(){
                            log.push_str(format!("[OK] Success : {}", target).as_str());
                            success += 1;
                            break; 
                        }else{
                            fail += 1;
                        }
                    }
                }
                
            }
            uix.set_log(log.into());
            uix.set_success_count(success);
            uix.set_fail_count(fail);
        }

    });


    ui.run()
}