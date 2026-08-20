use crate::global::constants::ERROR_CTRLC;
use std::io::Write;
use std::sync::{
  Arc,
  atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

pub struct Shutdown {
  running: Arc<AtomicBool>,
}

impl Shutdown {
  pub fn new(timeout: Option<u64>) -> Self {
    let running = Arc::new(AtomicBool::new(true));

    {
      let r = Arc::clone(&running);
      ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        print!("\r\x1b[2K");
        let _ = std::io::stdout().flush();
      })
      .expect(ERROR_CTRLC);
    }

    if let Some(seconds) = timeout {
      let r = Arc::clone(&running);
      std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(seconds));
        r.store(false, Ordering::SeqCst);
      });
    }

    Self { running }
  }

  #[inline]
  pub fn is_running(&self) -> bool {
    self.running.load(Ordering::SeqCst)
  }

  #[cfg(target_os = "windows")]
  #[inline]
  pub fn handle(&self) -> Arc<AtomicBool> {
    Arc::clone(&self.running)
  }
}
