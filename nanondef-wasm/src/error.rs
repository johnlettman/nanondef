#[derive(Debug, thiserror::Error)]
pub enum Error {
    NanondefError(#[from] nanondef::Error),
}

impl Into<wasm_bindgen::JsValue> for Error {
    fn into(self) -> wasm_bindgen::JsValue {
        self.to_string().into()
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
