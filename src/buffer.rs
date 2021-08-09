#[derive(Debug, Copy, Clone)]
struct Position {
    x: u16,
    y: u16,
}

struct FileBuffer {
    file_name: String,    // current filename
    content: Vec<String>, // maybe Vec<String> ??
    cursor: Position,     // current position of cursor
    scroll: Position,     // Scroll Position
    dirty: bool,          // is File not saved after last change
}
impl FileBuffer {
    fn insert_line(&mut self, idx: usize, line: String) {
        self.content.insert(idx, line);
    }
    fn delete_line(&mut self, idx: usize) {
        self.content.remove(idx);
    }
    fn append_line(&mut self, line: String) {
        self.content.push(line);
    }
}
