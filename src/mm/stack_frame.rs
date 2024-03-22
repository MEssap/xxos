use super::page_frame::{alloc_page, PageFrame};

#[derive(Default)]
pub struct StackFrame {
    page: PageFrame,
}

impl StackFrame {
    pub fn new() -> Self {
        Self { page: alloc_page() }
    }
}
