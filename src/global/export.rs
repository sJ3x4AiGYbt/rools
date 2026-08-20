use crate::global::constants::{EXPORT_SUCCESS, WARN_FILE_REMOVAL, ZIP_SUCCESS};
use chrono::Local;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use zip::CompressionMethod;
use zip::write::{FileOptions, ZipWriter};

pub struct Export;

impl Export {
  pub fn finalize(export_path: &Option<String>, zip: bool) -> std::io::Result<()> {
    if let Some(file) = export_path {
      eprintln!("\n{EXPORT_SUCCESS} {file}");

      if zip {
        let zip_path = Self::zip_file(file)?;
        if let Err(e) = std::fs::remove_file(file) {
          eprintln!("{WARN_FILE_REMOVAL} {file}: {e}");
        }
        println!("\n{ZIP_SUCCESS} {zip_path}");
      }
    }
    Ok(())
  }

  fn zip_file(path: &str) -> std::io::Result<String> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let zip_path = format!("{path}_{timestamp}.zip");
    let file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    let content = fs::read_to_string(path)?;
    let filename = Path::new(path).file_name().unwrap().to_str().unwrap();

    zip.start_file(filename, options)?;
    zip.write_all(content.as_bytes())?;
    zip.finish()?;

    Ok(zip_path)
  }
}
