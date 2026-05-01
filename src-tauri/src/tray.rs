use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

pub fn setup_tray(app: &AppHandle) -> tauri::Result<()> {
    let icon = Image::from_bytes(include_bytes!("../icons/32x32.png"))?;

    let show_hide = MenuItemBuilder::with_id("show_hide", "Mostrar/Ocultar").build(app)?;
    let settings = MenuItemBuilder::with_id("settings", "Configurações").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Sair").build(app)?;
    let menu = MenuBuilder::new(app)
        .item(&show_hide)
        .separator()
        .item(&settings)
        .separator()
        .item(&quit)
        .build()?;

    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .tooltip("WhatsApp")
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id().as_ref() {
                "show_hide" => {
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
                "settings" => eprintln!("Configurações (não implementado)"),
                "quit" => std::process::exit(0),
                _ => {}
            }
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
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}