
#[derive(Debug, Clone, Copy)]
pub struct LocationTracker {
    pub line_index: usize
}
impl LocationTracker {
    pub fn new() -> Self {
        LocationTracker {
            line_index: 0
        }
    }
    pub fn next_line(&mut self) {
        self.line_index += 1;
    }
}
