use proc_macro::Span;

pub trait SpanLen {
    fn len(&self) -> usize;
}

impl SpanLen for Span {
    /// why is this not part of `proc_macro`?
    fn len(&self) -> usize {
        self.source_text()
            .map(|s| s.len())
            .expect("span text should exist")
    }
}
