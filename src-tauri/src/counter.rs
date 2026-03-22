use tauri::command;

#[command]
pub fn increment(counter: i32) -> i32 {
  counter + 1
}
