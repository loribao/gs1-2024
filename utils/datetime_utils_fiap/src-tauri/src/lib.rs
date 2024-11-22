// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use chrono::prelude::*;

// Função para obter a data e hora no formato dia/mês/ano
#[tauri::command]
fn get_datetime() -> String {
    let local: DateTime<Local> = Local::now();
    let day_of_week = match local.weekday() {
        Weekday::Mon => "Segunda",
        Weekday::Tue => "Terça",
        Weekday::Wed => "Quarta",
        Weekday::Thu => "Quinta",
        Weekday::Fri => "Sexta",
        Weekday::Sat => "Sabado",
        Weekday::Sun => "Domingo",
    };
    let date_time = local.format("%H:%M:%S %d/%m/%Y").to_string();
    format!("{} {}", date_time,day_of_week)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_datetime])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
