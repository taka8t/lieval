pub(crate) fn is_literalchar(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.'
}

pub(crate) fn is_identstr(s: &str) -> bool {
    if let Some(t) = s.chars().next() {
        (t.is_alphabetic() || t == '_') && s.chars().all(|c| c.is_alphanumeric() || c == '_')
    }
    else {
        false
    }
}