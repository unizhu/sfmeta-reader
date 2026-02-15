pub fn estimate_tokens_approx(s: &str) -> usize {
    // Very rough, model-dependent; good enough for quick comparisons.
    s.chars().count() / 4
}
