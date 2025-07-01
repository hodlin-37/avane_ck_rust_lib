use chrono::Local;
use fern::Dispatch;
use std::fs;
use std::path::Path;


pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    // logs klasörü yoksa oluştur
    let log_dir = Path::new("logs");
    if !log_dir.exists() {
        fs::create_dir_all(log_dir)?;
    }

    // Günlük log dosyası adı
    let log_file_name = format!("logs/output-{}.log", Local::now().format("%Y-%m-%d"));

    // Fern dispatch
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("kevgir", log::LevelFilter::Debug)
        .level_for("reqwest", log::LevelFilter::Warn)
        .chain(fern::log_file(log_file_name)?) // Dosya çıktısı
        .apply()?;

    Ok(())
}