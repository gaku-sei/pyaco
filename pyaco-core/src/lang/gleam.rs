use std::{collections::HashSet, path::Path};

use askama::Template;
use compact_str::CompactString;

use crate::Result;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "gleam.txt")]
pub struct Gleam<'a> {
    classes: &'a HashSet<CompactString>,
}

impl<'a> LangTemplate<'a> for Gleam<'a> {
    fn new(
        _output_directory: &'a Path,
        _output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}

mod filters {
    use askama::Result;
    use convert_case::{Case, Casing};

    use crate::utils::escape_class_name;

    #[allow(clippy::unnecessary_wraps)]
    pub fn name(class: &str) -> Result<String> {
        Ok(escape_class_name(class).to_case(Case::Pascal))
    }
}
