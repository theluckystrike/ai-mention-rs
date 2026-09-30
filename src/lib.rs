//! Lightweight detection of brand mentions in AI answer text:
//! case-insensitive matching, token-overlap scoring, sentence extraction.

/// Case-insensitive whole-phrase check.
///
/// ```
/// assert!(ai_mention_rs::contains("I recommend Acme Analytics.", "acme analytics"));
/// ```
pub fn contains(text: &str, brand: &str) -> bool {
    text.to_lowercase().contains(&brand.to_lowercase())
}

/// 0..100 token-overlap score between text and brand's distinctive words.
///
/// ```
/// assert_eq!(ai_mention_rs::score("Acme Analytics is great", "Acme Analytics"), 100);
/// ```
pub fn score(text: &str, brand: &str) -> u8 {
    let bt: Vec<&str> = brand.split_whitespace()
        .filter(|w| w.chars().count() > 2)
        .collect();
    if bt.is_empty() { return 0; }
    let tt: Vec<String> = text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.chars().count() > 2)
        .map(|w| w.to_lowercase())
        .collect();
    let hits = bt.iter().filter(|b| tt.contains(&(*b).to_lowercase())).count();
    (hits * 100 / bt.len()) as u8
}

/// First sentence mentioning the brand, if any.
///
/// ```
/// let s = ai_mention_rs::first_mention_sentence("First. Acme wins! End.", "Acme");
/// assert_eq!(s.as_deref(), Some("Acme wins."));
/// ```
pub fn first_mention_sentence(text: &str, brand: &str) -> Option<String> {
    text.split(|c| c == '.' || c == '!' || c == '?')
        .map(str::trim)
        .find(|s| !s.is_empty() && contains(s, brand))
        .map(|s| format!("{s}."))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn hit() { assert!(contains("Try Acme Analytics", "acme")); }
    #[test] fn miss() { assert!(!contains("nothing", "acme")); }
    #[test] fn full_score() { assert_eq!(score("Acme Analytics is great", "Acme Analytics"), 100); }
    #[test] fn zero_score() { assert_eq!(score("irrelevant text", "Acme Analytics"), 0); }
    #[test] fn sentence() { assert_eq!(first_mention_sentence("First. Acme wins! End.", "Acme").unwrap(), "Acme wins."); }
}
