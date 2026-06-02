use tauri::{State, Emitter, Manager, WebviewWindowBuilder, WebviewUrl};
use crate::{AppState, config::AppConfig, timer::SessionType};

#[tauri::command]
pub fn open_settings(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        WebviewWindowBuilder::new(&app, "settings", WebviewUrl::App("settings.html".into()))
            .title("设置")
            .inner_size(400.0, 420.0)
            .resizable(false)
            .center()
            .build()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn start_timer(session_type: String, state: State<'_, AppState>) -> Result<crate::timer::TimerState, String> {
    let config = state.config.lock().unwrap();
    let duration_mins = match session_type.as_str() {
        "Work" => config.work_duration,
        "ShortBreak" => config.short_break_duration,
        "LongBreak" => config.long_break_duration,
        _ => config.work_duration,
    };
    drop(config);

    let st = match session_type.as_str() {
        "Work" => SessionType::Work,
        "ShortBreak" => SessionType::ShortBreak,
        "LongBreak" => SessionType::LongBreak,
        _ => SessionType::Work,
    };

    Ok(state.timer.start(st, duration_mins as u64 * 60))
}

#[tauri::command]
pub fn pause_timer(state: State<'_, AppState>) -> Result<Option<crate::timer::TimerState>, String> {
    Ok(state.timer.pause())
}

#[tauri::command]
pub fn resume_timer(state: State<'_, AppState>) -> Result<Option<crate::timer::TimerState>, String> {
    Ok(state.timer.resume())
}

#[tauri::command]
pub fn stop_timer(state: State<'_, AppState>) -> Result<Option<crate::timer::TimerState>, String> {
    let result = state.timer.stop();
    state.cycle.lock().unwrap().reset();
    Ok(result)
}

#[tauri::command]
pub fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    Ok(state.config.lock().unwrap().clone())
}

#[tauri::command]
pub fn update_config(new_config: AppConfig, app: tauri::AppHandle, state: State<'_, AppState>) -> Result<AppConfig, String> {
    {
        let mut cycle = state.cycle.lock().unwrap();
        cycle.set_pomodoros_per_cycle(new_config.pomodoros_per_cycle);
    }
    new_config.save(&app)?;
    {
        let mut config = state.config.lock().unwrap();
        *config = new_config.clone();
    }
    let _ = app.emit("config:updated", &new_config);
    Ok(new_config)
}

#[tauri::command]
pub fn get_cycle_state(state: State<'_, AppState>) -> Result<crate::cycle::CycleState, String> {
    Ok(state.cycle.lock().unwrap().state())
}
