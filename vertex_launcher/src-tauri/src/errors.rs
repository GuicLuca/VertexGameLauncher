#[derive(Debug, thiserror::Error)]
pub enum Verror {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("An error occurred while fetching the store at {0}")]
    StoreAccessError(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Verror {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
