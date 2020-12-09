quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IO(err: std::io::Error) {
            source(err)
            from()
        }
        Utf8(err: std::str::Utf8Error) {
            source(err)
            from()
        }
        Reason(reason: String) {}
    }
}

pub type Result<T> = std::result::Result<T, Error>;
