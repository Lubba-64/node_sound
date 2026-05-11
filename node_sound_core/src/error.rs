pub type Result<T> = anyhow::Result<T, NodeSoundError>;

#[derive(Debug)]
pub struct NodeSoundError(anyhow::Error);

impl Clone for NodeSoundError {
    fn clone(&self) -> Self {
        Self(anyhow::anyhow!("{:#?}", self.0))
    }
}

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

impl From<Box<dyn std::error::Error>> for NodeSoundError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        anyhow::anyhow!("{:#?}", value).into()
    }
}
