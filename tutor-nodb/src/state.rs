use std::sync::Mutex;

pub struct AppState {
    // shared imut state
    pub health_check_response: String,
    // shared mut state
    pub visit_count: Mutex<u32>,
}
