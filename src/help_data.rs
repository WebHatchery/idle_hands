//! Shared cabinet guidance loaded from the authored content catalog.

use crate::content::GameContent;

pub fn paragraphs(content: &GameContent) -> &[String] {
    &content.labels.help_paragraphs
}

pub fn navigation(content: &GameContent) -> &[String] {
    &content.labels.help_navigation
}

#[cfg(test)]
#[path = "../tests/legacy/help_data/tests.rs"]
mod tests;
