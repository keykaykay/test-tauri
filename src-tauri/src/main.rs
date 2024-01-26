// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::{Manager, Window};

#[tauri::command]
fn handle_window(window: Window, payload: &str) {
    let loading_window = window.get_window("loading").expect("loading 窗口不存在");
    loading_window.close().unwrap();
    let qzd_window = window.get_window("qizhidao").expect("企知道 窗口不存在");
    qzd_window.show().expect("企知道 窗口显示失败");
    qzd_window.eval(payload).expect("企知道 窗口执行脚本失败");
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let loading_window = _app.get_window("loading").unwrap();
                loading_window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![handle_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
