pub(crate) fn is_literalchar(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '.'
}