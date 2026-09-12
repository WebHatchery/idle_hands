//! Shared credits and privacy copy loaded from the authored content catalog.

use crate::content::GameContent;

pub fn title(content: &GameContent) -> &str {
    &content.labels.credits_title
}

pub fn paragraphs(content: &GameContent) -> &[String] {
    &content.labels.credits_paragraphs
}

#[cfg(test)]
mod tests;
