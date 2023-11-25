#[cfg(target_os = "linux")]
pub static BASEMENT_FOLDER: &str = "/home/pi/.config/hitman_basement";

#[cfg(target_os = "windows")]
pub static basement_folder: &str = "C:/config";

#[cfg(target_os = "macos")]
pub static basement_folder: &str = "macos folder";
