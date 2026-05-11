pub type Result<T> = anyhow::Result<T, NodeSoundError>;

#[derive(Debug)]
pub struct NodeSoundError(anyhow::Error);

impl std::fmt::Display for NodeSoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for NodeSoundError {}

impl NodeSoundError {
    pub fn new(error: impl Into<anyhow::Error>) -> Self {
        Self(error.into())
    }
}

impl From<anyhow::Error> for NodeSoundError {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}
