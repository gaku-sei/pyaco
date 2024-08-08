use std::{collections::HashSet, path::Path};

use askama::Template;
use compact_str::CompactString;

use crate::Result;

use super::LangTemplate;

#[derive(Template)]
#[template(path = "rescript_type1.txt")]
pub struct RescriptType1<'a> {
    classes: &'a HashSet<CompactString>,
}

impl<'a> LangTemplate<'a> for RescriptType1<'a> {
    fn new(
        _output_directory: &'a Path,
        _output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self> {
        Ok(Self { classes })
    }
}
