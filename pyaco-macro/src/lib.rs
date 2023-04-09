#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::{collections::HashSet, env, path::PathBuf, process};

use compact_str::CompactString;
use once_cell::sync::OnceCell;
use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, emit_call_site_warning, proc_macro_error};
use pyaco_core::{extract_classes_from_file, extract_classes_from_url};
use quote::quote;
use serde::Deserialize;
use syn::{parse_macro_input, LitStr};
use tokio::{fs::File, io::AsyncReadExt, runtime::Runtime};
use tracing::error;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum InputConfig {
    Simple(String),
    Path { path: String },
    Url { url: String },
}

#[derive(Debug, Deserialize)]
struct GeneralConfig {
    input: InputConfig,
}

#[derive(Debug, Deserialize)]
struct Config {
    general: GeneralConfig,
}

static CONFIG_FILE_NAME: &str = "pyaco.toml";
static CONFIG: OnceCell<Config> = OnceCell::new();
static ACCEPTED_CLASSES: OnceCell<HashSet<CompactString>> = OnceCell::new();

#[proc_macro]
#[proc_macro_error]
pub fn css(input: TokenStream) -> TokenStream {
    let input: LitStr = parse_macro_input!(input);

    let input_value = input.value();

    let classes = input_value.split_whitespace();

    let mut out_classes = Vec::new();

    // Validate class names
    for class in classes {
        if out_classes.contains(&class) {
            emit_call_site_warning!("class already in class names list: {}", class);
            continue;
        }

        let accepted_classes = ACCEPTED_CLASSES.get_or_init(init_accepted_classes);
        if !accepted_classes.contains(class) {
            abort_call_site!("invalid class name: {}", class);
        }

        out_classes.push(class);
    }

    let out_classes = out_classes.join(" ");

    quote! {
        #out_classes
    }
    .into()
}

fn init_config() -> Config {
    let Ok(root) = env::var("CARGO_MANIFEST_DIR") else {
        unreachable!("CARGO_MANIFEST_DIR env var not set");
    };

    let root = PathBuf::from(root);

    let filename = root.join(CONFIG_FILE_NAME);

    if !filename.exists() {
        error!("couldn't find required pyaco.toml configuration file");
        process::exit(1);
    }

    let rt = Runtime::new().unwrap_or_else(|err| {
        error!("failed to create tokio runtime: {err}");
        process::exit(1);
    });

    let mut file = rt.block_on(File::open(filename)).unwrap_or_else(|err| {
        error!("failed to read pyaco.toml configuration file: {err}");
        process::exit(1);
    });

    let mut content = String::new();

    if let Err(err) = rt.block_on(file.read_to_string(&mut content)) {
        error!("impossible to read pyaco.toml configuration file: {err}");
        process::exit(1);
    }

    toml::from_str(&content).unwrap_or_else(|err| {
        error!("pyaco.toml configuration is invalid: {err}");
        process::exit(1);
    })
}

fn init_accepted_classes() -> HashSet<CompactString> {
    let config = CONFIG.get_or_init(init_config);

    match &config.general.input {
        InputConfig::Simple(path) | InputConfig::Path { path } => {
            let rt = match Runtime::new() {
                Ok(rt) => rt,
                Err(err) => {
                    error!("couldn't create the tokio runtime: {err}");
                    process::exit(1);
                }
            };

            rt.block_on(extract_classes_from_file(path))
        }
        InputConfig::Url { url } => extract_classes_from_url(url),
    }
    .expect("css could not be loaded")
}
