use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;
use ripunzip::{NullProgressReporter, UnzipEngine, UnzipOptions};

#[derive(Parser)]
#[command(version, about = "Параллельный распаковщик zip-архивов")]
struct Args {
    /// Путь к zip-архиву
    archive: PathBuf,

    /// Каталог, в который распаковывать (по умолчанию — текущий)
    #[arg(short = 'd', long = "dest")]
    dest: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.archive)
        .with_context(|| format!("не удалось открыть архив {:?}", args.archive))?;
    let engine = UnzipEngine::for_file(file)?;

    let options = UnzipOptions {
        output_directory: args.dest,
        password: None,
        single_threaded: false,
        filename_filter: None,
        progress_reporter: Box::new(NullProgressReporter),
    };

    let started = Instant::now();
    engine.unzip(options).context("ошибка при распаковке")?;
    let elapsed = started.elapsed();

    println!("Готово. Распаковка заняла {:.3} с", elapsed.as_secs_f64());
    Ok(())
}
