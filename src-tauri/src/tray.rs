#![cfg(all(desktop, not(test)))]

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime, WebviewUrl,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let open_window_i = MenuItem::with_id(app, "open-window", "Open window", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu1 = Menu::with_items(
        app,
        &[
            &open_window_i,
            &quit_i,
        ],
    )?;
    let _ = TrayIconBuilder::with_id("tray-1")
        .tooltip("Tauri")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu1)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "open-window" => {
                let _webview =
                    tauri::WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                        .title("Therapits")
                        .build()
                        .unwrap();
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app);

    Ok(())
}