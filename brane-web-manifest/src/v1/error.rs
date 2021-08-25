#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseJson(serde_json::Error),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ParseJson(err)
    }
}
