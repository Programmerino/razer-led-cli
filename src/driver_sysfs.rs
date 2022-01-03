use lazy_static::lazy_static;

use std::fs;

// Driver path
pub const DRIVER_DIR: &'static str =
    "/sys/module/razercontrol/drivers/hid:Razer laptop System control driver";

lazy_static! {
    static ref SYSFS_PATH: Option<String> = {
        match fs::read_dir(DRIVER_DIR)
            .unwrap()
            .find(|x| {
                x.as_ref()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .starts_with("000")
            })
            .unwrap()
        {
            Ok(p) => Some(String::from(p.path().to_str().unwrap())),
            Err(_) => None,
        }
    };
}

/// Writes a byte array to a sysfs file
fn write_to_sysfs_raw(sysfs_name: &str, val: Vec<u8>) -> bool {
    match fs::write(SYSFS_PATH.clone().unwrap() + "/" + sysfs_name, val) {
        Ok(_) => true,
        Err(x) => {
            eprintln!("SYSFS write to {} failed! - {}", sysfs_name, x);
            false
        }
    }
}

// RGB Map is write only
pub fn write_rgb_map(map: Vec<u8>) -> bool {
    return write_to_sysfs_raw("key_colour_map", map);
}
