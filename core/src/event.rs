pub enum Event {
    PointerEvent(PointerEvent),
    WindowEvent(WindowEvent),
    KeyboardEvent(KeyboardEvent),
    ClipboardEvent(ClipboardEvent)
}

pub enum PointerEvent {
    Touch,
    MouseMoved,
    MouseEntered,
    MouseExited,
    MouseInput
}

pub enum WindowEvent {
    Resized,
    Moved,
    CloseRequested,
    FocusChanged,
    DroppedFile,
    HoveredFile
}

pub enum KeyboardEvent {
    KeyboardInput,
}

pub enum ClipboardEvent {

}