use wasm_bindgen::JsValue;


#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("JS Binding: {0}")]
    Binding(#[from] JsValue),
    #[error("JSON: {0}")]
    Serde(#[from] serde_wasm_bindgen::Error),
    #[cfg(any(feature = "event", feature = "window"))]
    #[error("Oneshot cancelled: {0}")]
    OneshotCanceled(#[from] futures::channel::oneshot::Canceled)
}
