use tauri;

#[tauri::command]
pub fn send_notification(title: String, body: String) {
    if let Err(e) = notify_rust::Notification::new()
        .summary(&title)
        .body(&body)
        .appname("WhatsApp")
        .icon("whatsapp") // pode não funcionar em todos os DE;
        .show()
    {
        eprintln!("Falha ao exibir notificação nativa: {}", e);
    }
}