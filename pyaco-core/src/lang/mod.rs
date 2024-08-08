use std::{borrow::Cow, collections::HashSet, path::Component, path::Path, str::FromStr};

use askama::Template;
use async_trait::async_trait;
use compact_str::CompactString;
use gleam::Gleam;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::info;

use crate::resolve_path;
use crate::Result;

pub use super::elm::Elm;
pub use super::purescript::Purescript;
pub use super::rescript::Rescript;
pub use super::rescript::Rescripti;
pub use super::rescript_type1::RescriptType1;
pub use super::rescript_type2::RescriptType2;
pub use super::typescript::Typescript;
pub use super::typescript_type_1::TypescriptType1;
pub use super::typescript_type_2::TypescriptType2;

pub mod elm;
pub mod gleam;
pub mod purescript;
pub mod rescript;
pub mod rescript_type1;
pub mod rescript_type2;
pub mod typescript;
pub mod typescript_type_1;
pub mod typescript_type_2;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Lang {
    Elm,
    Gleam,
    Purescript,
    Rescript,
    RescriptType1,
    RescriptType2,
    Typescript,
    TypescriptType1,
    TypescriptType2,
}

impl Lang {
    #[allow(clippy::missing_errors_doc)]
    pub async fn render_to_file(
        self,
        output_directory: &Path,
        output_filename: &str,
        classes: &HashSet<CompactString>,
    ) -> Result<()> {
        match self {
            Self::Elm => {
                let template = Elm::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "elm"))
                    .await?;
            }
            Self::Gleam => {
                let template = Gleam::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "gleam"))
                    .await?;
            }
            Self::Purescript => {
                let template = Purescript::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "purs"))
                    .await?;
            }
            Self::Rescript => {
                let template = Rescript::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "res"))
                    .await?;

                let template = Rescripti::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "resi"))
                    .await?;
            }
            Self::RescriptType1 => {
                let template = RescriptType1::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "res"))
                    .await?;
            }
            Self::RescriptType2 => {
                let template = RescriptType2::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "res"))
                    .await?;
            }
            Self::Typescript => {
                let template = Typescript::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "ts"))
                    .await?;
            }
            Self::TypescriptType1 => {
                let template = TypescriptType1::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "ts"))
                    .await?;
            }
            Self::TypescriptType2 => {
                let template = TypescriptType2::new(output_directory, output_filename, classes)?;

                template
                    .write_to_file(&resolve_path(output_directory, output_filename, "ts"))
                    .await?;
            }
        }

        Ok(())
    }
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "elm" => Ok(Lang::Elm),
            "gleam" => Ok(Lang::Gleam),
            "purescript" => Ok(Lang::Purescript),
            "rescript" => Ok(Lang::Rescript),
            "rescript-type-1" => Ok(Lang::RescriptType1),
            "rescript-type-2" => Ok(Lang::RescriptType2),
            "typescript" => Ok(Lang::Typescript),
            "typescript-type-1" => Ok(Lang::TypescriptType1),
            "typescript-type-2" => Ok(Lang::TypescriptType2),
            unknown_lang => Err(format!(
                "\"{unknown_lang}\" is not a valid lang, should be one of (elm|purescript|rescript|rescript-type|typescript|typescript-type-1|typescript-type-2)"
            )),
        }
    }
}

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait LangTemplate<'a>: Template + Sized {
    /// ## Errors
    ///
    /// A template creation typically fails when the directory/filenames are not present or can't be accessed
    fn new(
        output_directory: &'a Path,
        output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self>;

    async fn write_to_file(&self, path: &Path) -> Result<()> {
        info!("Writing code into {}", path.to_string_lossy());

        let code = self.render()?;

        let mut output = File::create(path).await?;

        output.write_all(code.as_bytes()).await?;

        Ok(())
    }
}

/// Used by Elm and PureScript to generate their module name based on the directory and the filename
pub(crate) fn generate_module_name<'a>(
    output_directory: &'a Path,
    output_filename: &'a str,
) -> Result<Cow<'a, str>> {
    let base = output_directory.components().try_fold(
        "".into(),
        |acc, component| -> Result<Cow<'a, str>> {
            if let Component::Normal(part) = component {
                let part = part.to_string_lossy();

                if acc.is_empty() {
                    return Ok(part);
                }

                return Ok(format!("{acc}.{part}").into());
            }

            Ok(acc)
        },
    )?;

    if base.is_empty() {
        return Ok(output_filename.into());
    }

    Ok(format!("{base}.{output_filename}").into())
}
