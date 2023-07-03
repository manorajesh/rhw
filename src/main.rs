use tauri::{SystemTray, SystemTrayMenu};

fn main() {
  let tray_menu = SystemTrayMenu::new(); // insert the menu items here
  let system_tray = SystemTray::new()
    .with_menu(tray_menu);
  tauri::Builder::default()
    .system_tray(system_tray)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
} 