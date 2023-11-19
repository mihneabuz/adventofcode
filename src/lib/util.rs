use std::io;

pub fn into_io_error(err: impl ToString) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err.to_string())
}
