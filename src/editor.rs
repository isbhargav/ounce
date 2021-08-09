mod buffer;
mod termianl;
enum Mode {
    INSERT,
    NORMAL,
    EXIT,
}

struct Ounce {
    terminal: Terminal,
    _mode: Mode,
    buffers: Option<FileBuffer>,
}
