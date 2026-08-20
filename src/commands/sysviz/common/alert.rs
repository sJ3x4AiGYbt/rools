use crate::commands::sysviz::common::constants::MSG_ALERT;

pub struct Alert {
  target: Option<String>,
}

impl Alert {
  pub fn new(target: Option<String>) -> Self {
    Self { target }
  }

  pub fn check_and_trigger(&self, value: &str) {
    if let Some(ref target_alert) = self.target
      && value == target_alert
    {
      self.trigger_alert(value);
    }
  }

  fn trigger_alert(&self, trigger: &str) {
    eprintln!("\x1b[1;31m{} {}\x1b[0m", MSG_ALERT, trigger.to_uppercase());
  }
}
