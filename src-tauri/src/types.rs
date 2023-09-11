use std::io;

use tauri::InvokeError;

pub struct InvokeErrorBind(String);

impl Into<InvokeError> for InvokeErrorBind {
    fn into(self) -> InvokeError {
        self.0.to_string().into()
    }
}

impl From<rusqlite::Error> for InvokeErrorBind {
    fn from(value: rusqlite::Error) -> Self {
        InvokeErrorBind(value.to_string())
    }
}

impl From::<io::Error> for InvokeErrorBind {
    fn from(value: io::Error) -> Self {
        InvokeErrorBind(value.to_string())
    }
}
