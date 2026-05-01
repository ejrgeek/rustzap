mod notifications;
mod tray;
mod wayland_fix;

fn main() {
    wayland_fix::setup();

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(move |app| {
            let _window = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External(
                    "https://web.whatsapp.com/"
                        .parse()
                        .unwrap(),
                ),
            )
            .title("[RustZap] WhatsApp")
            .inner_size(1200.0, 800.0)
            .resizable(true)
            .fullscreen(false)
            .visible(true)
            .decorations(true)
            .center()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .initialization_script(include_str!("../inject.js"))
            .build()?;

            tray::setup_tray(app.handle())?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![notifications::send_notification])
        .run(context)
        .expect("Erro ao executar");
}