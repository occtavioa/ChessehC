use tauri::InvokeError;

pub struct RusqliteToInvokeError(rusqlite::Error);

impl Into<InvokeError> for RusqliteToInvokeError {
    fn into(self) -> InvokeError {
        self.0.to_string().into()
    }
}

impl From<rusqlite::Error> for RusqliteToInvokeError {
    fn from(value: rusqlite::Error) -> Self {
        RusqliteToInvokeError(value)
    }
}
