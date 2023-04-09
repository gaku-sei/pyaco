use std::{borrow::Cow, collections::HashSet, path::Path};

use askama::Template;
use compact_str::CompactString;

use crate::Result;

use super::{generate_module_name, LangTemplate};

#[derive(Template)]
#[template(path = "elm.txt")]
pub struct Elm<'a> {
    classes: &'a HashSet<CompactString>,
    module_name: Cow<'a, str>,
}

impl<'a> LangTemplate<'a> for Elm<'a> {
    fn new(
        output_directory: &'a Path,
        output_filename: &'a str,
        classes: &'a HashSet<CompactString>,
    ) -> Result<Self> {
        Ok(Elm {
            classes,
            module_name: generate_module_name(output_directory, output_filename)?,
        })
    }
}

mod filters {
    use askama::Result;
    use convert_case::{Case, Casing};

    use crate::utils::escape_class_name;

    #[allow(clippy::unnecessary_wraps)]
    pub fn name(class: &str) -> Result<String> {
        Ok(escape_class_name(class).to_case(Case::Camel))
    }
}
