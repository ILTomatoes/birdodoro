use tauri::{WebviewWindowBuilder, WebviewUrl};

pub fn show_bird(app: &tauri::AppHandle) -> Result<(), String> {
    let monitor = app.primary_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("No primary monitor found".to_string())?;
    let screen_size = monitor.size();
    let screen_width = screen_size.width as i32;
    let screen_height = screen_size.height as i32;

    let window_height = 120i32;

    let label = format!("bird-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis());

    let _window = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("bird.html".into()))
        .title("Bird")
        .inner_size((screen_width as f64) + 200.0, window_height as f64)
        .position(-100.0, ((screen_height - window_height) / 2) as f64)
        .decorations(false)
        .transparent(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .focusable(false)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}
