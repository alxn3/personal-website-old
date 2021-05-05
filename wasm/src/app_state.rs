use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
  static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_dynamic_data(time: f32, canvas_weight: f32, canvas_height: f32) {
  let mut data = APP_STATE.lock().unwrap();

  *data = Arc::new(AppState {
    canvas_width: canvas_weight,
    canvas_height: canvas_height,
    time: time,
    ..*data.clone()
  });
}

pub fn get_curr_state() -> Arc<AppState> {
  APP_STATE.lock().unwrap().clone()
}

pub struct AppState {
  pub canvas_width: f32,
  pub canvas_height: f32,
  pub time: f32
}

impl AppState {
  fn new() -> Self {
    Self{
      canvas_width: 0.,
      canvas_height: 0.,
      time: 0.,
    }
  }
}
