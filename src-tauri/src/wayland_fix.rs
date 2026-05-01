use std::env;

pub fn setup() {
    env::set_var("GDK_BACKEND", "wayland,x11");

    // Flags solicitadas para HiDPI, touchscreen e aceleração (Dell XPS)
    env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    env::set_var("WEBKIT_FORCE_SANDBOX", "1");
    env::set_var("WEBP_FORCE_ACCELERATION", "1");

    if let Ok(session) = env::var("XDG_SESSION_TYPE") {
        if session == "wayland" {
        }
    }
}