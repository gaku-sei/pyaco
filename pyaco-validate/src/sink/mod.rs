use std::path::PathBuf;

use async_trait::async_trait;
use compact_str::CompactString;

pub mod console;

#[async_trait]
pub trait Sink {
    type Event;

    fn send(&mut self, event: Self::Event);

    async fn done(self) -> bool;
}

pub struct SearchFileEvent {
    pub line_number: Option<u64>,
    pub path: PathBuf,
    pub class: CompactString,
}

impl SearchFileEvent {
    pub fn new(path: impl Into<PathBuf>, class: impl Into<CompactString>) -> Self {
        Self {
            line_number: None,
            path: path.into(),
            class: class.into(),
        }
    }

    pub fn with_line_number(mut self, line_number: u64) -> Self {
        self.line_number = Some(line_number);
        self
    }
}
