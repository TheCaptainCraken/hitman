#[cfg(target_os = "linux")]
pub static basement_folder: &str = "~/.config/basement";

#[cfg(target_os = "windows")]
pub static basement_folder: &str = "C:/config";

#[cfg(target_os = "macos")]
pub static basement_folder: &str = "macos folder";
