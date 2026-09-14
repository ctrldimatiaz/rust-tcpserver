// Available commands across the protocol
pub enum Command {
    Get(String),
    Set(String, String),
    Delete(String),
}

#[derive(Debug)]
// Available responses across the protocol
pub enum Response {
    Value(String),
    Ok,
    NotFound,
}

impl Response {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Response::Value(v) => v.as_bytes(),
            Response::Ok => "OK".as_bytes(),
            Response::NotFound => "NOT FOUND".as_bytes(),
        }
    }
}
