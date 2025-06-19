

#[cfg(target_arch = "wasm32")]
pub(crate) enum MouseButton {
    Left,
    Middle,
    Right
}

#[cfg(target_arch = "wasm32")]
pub(crate) enum MouseState {
    Pressed(MouseButton),
    Released,
    None
} 
