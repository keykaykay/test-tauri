// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use playwright::Playwright;

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// #[tauri::command]
// async fn pw() -> Result<(), playwright::Error> {
//     let playwright = Playwright::initialize().await?;
//     playwright.prepare()?;
//     let chromium = playwright.chromium();
//     let browser = chromium.launcher().launch().await?;
//     let context = browser.context_builder().build().await?;
//     let page = context.new_page().await?;
//     page.goto_builder("https://www.baidu.com").goto().await?;

//     page.eval("() => alert('Hello World!')").await?;
//     Ok(())
// }

use tauri::Manager;


fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.eval("
                function sleep(ms) {
                    return new Promise(resolve => setTimeout(resolve, ms))
                }
                setTimeout(async () => {
                    const spanEl = document.querySelector('#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.tab-box > div:nth-child(2) > span');
                    spanEl.click();
                    await sleep(500);
                    const inputEl = document.querySelector('#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.form-blk > form > div:nth-child(1) > div > div > input');
                    inputEl.value = '';
                    await sleep(500);
                    inputEl.value = '13717955328';
                    inputEl.dispatchEvent(new Event('input'));
                    await sleep(500);
                    const passwordEl = document.querySelector('#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.form-blk > form > div.el-form-item.psput.is-required > div > div > input');
                    passwordEl.value = 'cps63072281';
                    passwordEl.dispatchEvent(new Event('input'));
                    await sleep(500);
                    const ruleEl = document.querySelector('#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.form-blk > form > div.note-line > span > span > div > label > span > span');
                    ruleEl.click();
                    await sleep(500);
                    const loginEl = document.querySelector('#__layout > div > div.login-container.layout-content-main > div.main-blk > div > div > div > div > div > div > div.form-blk > form > div.el-form-item.btn-line > div > button');
                    loginEl.click();
                }, 1500);
            ").unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
