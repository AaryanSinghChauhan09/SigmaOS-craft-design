use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct DesignTokens { tokens: HashMap<String, String> }
impl DesignTokens {
    pub fn new() -> Self { Self::default() }
    pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) { self.tokens.insert(key.into(), val.into()); }
    pub fn get(&self, key: &str) -> Option<&str> { self.tokens.get(key).map(|s| s.as_str()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_tokens() {
        let mut t = DesignTokens::new();
        t.set("primary", "#0070f3");
        assert_eq!(t.get("primary"), Some("#0070f3"));
    }
}
