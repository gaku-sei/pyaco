#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{
    convert::TryInto,
    path::{Path, PathBuf},
    time::Duration,
};

use clap::Parser as ClapParser;
use notify::RecursiveMode;
use pyaco_core::{async_debounced_watcher, InputType, Lang};
use serde::Deserialize;
use tokio::fs::create_dir_all;
use tracing::{enabled, info, warn, Level};

pub use crate::errors::*;

mod errors;

#[derive(ClapParser, Debug, Deserialize)]
pub struct Options {
    /// CSS file path and/or URL to parse and generate code from
    #[clap(short, long)]
    pub input: String,

    /// Directory for generated code
    #[clap(short, long, default_value = "./")]
    pub output_directory: PathBuf,

    /// Filename (without extension) used for the generated code
    #[clap(short = 'f', long)]
    pub output_filename: String,

    /// Language used in generated code (elm|purescript|rescript|typescript|typescript-type-1|typescript-type-2)
    #[clap(short, long)]
    pub lang: Lang,

    /// Watch for changes in the provided css file and regenarate the code (doesn't work with URL)
    #[clap(short, long)]
    pub watch: bool,

    /// Watch debounce duration (in ms), if files are validated twice after saving the css file, you should try to increase this value
    #[clap(long, default_value = "10")]
    pub watch_debounce_duration: u64,
}

#[allow(clippy::missing_errors_doc)]
pub async fn run(options: Options) -> Result<()> {
    let input = options.input.as_str().try_into()?;

    if enabled!(Level::INFO) || enabled!(Level::WARN) {
        match input {
            InputType::Path(ref path) => info!("Extracting from file {:?}", path),
            InputType::Url(ref url) => {
                info!("Extracting from URL {}", url);

                if options.watch {
                    warn!("You provided an URL as the css input, watch mode will not be activated");
                }
            }
        }
    };

    info!(
        "Creating directory {} if needed",
        options.output_directory.to_string_lossy()
    );

    create_dir_all(&options.output_directory).await?;

    // Always run at least once, even in watch mode
    run_once(
        &input,
        &options.lang,
        &options.output_directory,
        &options.output_filename,
    )
    .await?;

    if options.watch {
        if let InputType::Path(ref path) = input {
            run_watch(
                path,
                &options.lang,
                &options.output_directory,
                &options.output_filename,
                options.watch_debounce_duration,
            )
            .await?;
        }
    }

    Ok(())
}

async fn run_once(
    input: &InputType,
    lang: &Lang,
    output_directory: &Path,
    output_filename: &str,
) -> Result<()> {
    let allowed_classes = input.extract_classes().await?;
    lang.render_to_file(output_directory, output_filename, &allowed_classes)
        .await?;

    Ok(())
}

async fn run_watch(
    path: &Path,
    lang: &Lang,
    output_directory: &Path,
    output_filename: &str,
    watch_debounce_duration: u64,
) -> Result<()> {
    let (mut debouncer, mut rx) =
        async_debounced_watcher(Duration::from_millis(watch_debounce_duration))?;
    debouncer
        .watcher()
        .watch(path, RecursiveMode::NonRecursive)?;

    while let Some(event) = rx.recv().await {
        match event {
            Ok(events) if !events.is_empty() => {
                run_once(
                    &InputType::Path(path.to_owned()),
                    lang,
                    output_directory,
                    output_filename,
                )
                .await?;
            }
            _ => {}
        }
    }

    Ok(())
}
