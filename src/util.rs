pub(crate) fn is_literalchar(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.'
}

pub(crate) fn is_identstr(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    && s.chars().find(|c| c.is_ascii_alphanumeric()).filter(|c| c.is_ascii_alphabetic()).is_some()
}