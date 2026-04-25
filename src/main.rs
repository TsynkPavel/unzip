use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use ripunzip::{UnzipEngine, UnzipOptions, UnzipProgressReporter};

#[derive(Parser)]
#[command(version, about = "Параллельный распаковщик zip-архивов")]
struct Args {
    /// Путь к zip-архиву
    archive: PathBuf,

    /// Каталог, в который распаковывать (по умолчанию — текущий)
    #[arg(short = 'd', long = "dest")]
    dest: Option<PathBuf>,
}

struct ProgressReporter {
    bar: ProgressBar,
    extracted: AtomicU64,
}

impl ProgressReporter {
    fn new(total: u64) -> Self {
        let bar = ProgressBar::new(total);
        bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
            )
            .unwrap()
            .progress_chars("=>-"),
        );
        Self {
            bar,
            extracted: AtomicU64::new(0),
        }
    }
}

impl UnzipProgressReporter for ProgressReporter {
    fn total_bytes_expected(&self, total: u64) {
        self.bar.set_length(total);
    }

    fn bytes_extracted(&self, count: u64) {
        let prev = self.extracted.fetch_add(count, Ordering::Relaxed);
        self.bar.set_position(prev + count);
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.archive)
        .with_context(|| format!("не удалось открыть архив {:?}", args.archive))?;
    let engine = UnzipEngine::for_file(file)?;

    let total = engine.zip_length();
    let reporter = ProgressReporter::new(total);
    let bar = reporter.bar.clone();

    let options = UnzipOptions {
        output_directory: args.dest,
        password: None,
        single_threaded: false,
        filename_filter: None,
        progress_reporter: Box::new(reporter),
    };

    let started = Instant::now();
    engine.unzip(options).context("ошибка при распаковке")?;
    let elapsed = started.elapsed();

    bar.finish_and_clear();
    println!("Готово. Распаковка заняла {:.3} с", elapsed.as_secs_f64());
    Ok(())
}
