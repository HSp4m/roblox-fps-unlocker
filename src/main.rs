#![windows_subsystem = "windows"]
use std::fs;
use std::io::Write;
use std::path::Path;


slint::include_modules!();

fn path_exists(folder_path: &Path) -> bool {

    if Path::exists(folder_path) {
        return true;
    }

    false
}

fn get_method_name(method: i32) -> String {
    if method == 0 {
        "low".to_string()
    } else if method == 1 {
        "normal".to_string()
    } else {
        "uninstall".to_string()
    }
}

fn get_latest_folder(user: String) -> String {
    let last_modified_dir = std::fs::read_dir(format!("C:\\Users\\{}\\AppData\\Local\\Roblox\\Versions\\", user))
    .expect("Couldn't the roblox directory")
    .flatten() // Remove failed
    .filter(|f| f.metadata().unwrap().is_dir())
    .max_by_key(|x| x.metadata().unwrap().modified().unwrap());
    let folder = last_modified_dir.unwrap().file_name();
    folder.to_str().unwrap().to_string()

}


fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;
    let ui_handle: slint::Weak<AppWindow> = ui.as_weak();

    ui.on_request_unlock(move |_string: slint::SharedString| {
        let low_config = reqwest::blocking::get("https://raw.githubusercontent.com/HSp4m/roblox-fps-unlocker/main/src/configs/low.json").unwrap();
        let normal_config = reqwest::blocking::get("https://raw.githubusercontent.com/HSp4m/roblox-fps-unlocker/main/src/configs/normal.json").unwrap();

        let ui: AppWindow = ui_handle.unwrap();
        let method: i32 = _string.trim().parse().unwrap();

        let user: String = whoami::username();

        let roblox_dir: String = format!("C:\\Users\\{}\\AppData\\Local\\Roblox\\Versions\\", user);
        let path_roblox: &Path = Path::new(&roblox_dir);


        if path_exists(path_roblox) {

            let path_roblox_dir = get_latest_folder(user);

            let settings_dir: String = format!("{}{}\\ClientSettings\\", roblox_dir,path_roblox_dir);
            let path_settings: &Path = Path::new(&settings_dir);
            let settings_file: String = format!("{}ClientAppSettings.json", settings_dir);
            let path_settings_file: &Path = Path::new(&settings_file);

            ui.set_result("[*] Starting process".into());

            if !path_exists(path_settings) {
                ui.set_result("[+] Creating folder".into());
                fs::create_dir(path_settings).expect("An error ocurred");
            }
            
            if path_exists(path_settings) {
                if get_method_name(method) != "uninstall" {
                    ui.set_result("[+] Creating files".into());
                    let mut create_file = fs::File::create(path_settings_file).expect("An error ocurred while trying to create the config file.");
                    
                    if path_exists(path_settings_file) {
                        if get_method_name(method) == "low" {
                            create_file.write_all(low_config.text().unwrap().as_bytes()).expect("An error ocurred while trying to write to the config");
                        } else if get_method_name(method) == "normal" {
                            create_file.write_all(normal_config.text().unwrap().as_bytes()).expect("An error ocurred while trying to write to the config");
                        }
                    }
                    
                    ui.set_result("[OK] Finished.\n- Start roblox to see the change".into());
                } else {

                    ui.set_result("[-] Removing folders".into());
                    fs::remove_dir_all(path_settings).expect("An error ocurred while trying remove the config folder.");

                    if path_exists(path_settings) {
                        ui.set_result("[-] Something went wrong while trying to remove.\n- Verify if the app has permissions".into());
                    } else {
                        ui.set_result("[OK] Finished.\n- Config has been successfully removed".into());
                    }

                }
                
            }
            

        } else {
            ui.set_result("[!] Can't find roblox path\n- Verify if roblox is installed \n- Try download Roblox Player from Roblox website".into());
        }

        
    });


    ui.run()
}
