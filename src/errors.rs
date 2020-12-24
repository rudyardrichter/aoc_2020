use std::error::Error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct AOCError<'a> {
    msg: &'a str,
}

impl<'a> AOCError<'a> {
    pub fn new(msg: &'a str) -> Self {
        Self { msg: msg }
    }
}

impl<'a> fmt::Display for AOCError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<'a> Error for AOCError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
