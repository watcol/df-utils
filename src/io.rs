//! IO Control
use std::fs::File;
use std::io;
use std::path::Path;

/// Unified input stream
#[derive(Debug)]
pub enum Input {
    Stdin(io::Stdin),
    File(File),
}

impl io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Stdin(s) => s.read(buf),
            Self::File(f) => f.read(buf),
        }
    }
}

impl Input {
    pub fn from_path<P: AsRef<Path>>(path: Option<P>) -> io::Result<Self> {
        match &path {
            Some(path) if path.as_ref() == Path::new("-") => Ok(Self::Stdin(io::stdin())),
            Some(path) => File::open(path).map(Self::File),
            None => Ok(Self::Stdin(io::stdin())),
        }
    }
}

/// Unified output stream
#[derive(Debug)]
pub enum Output {
    Stdout(io::Stdout),
    File(File),
}

impl io::Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Stdout(s) => s.write(buf),
            Self::File(f) => f.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Stdout(s) => s.flush(),
            Self::File(f) => f.flush(),
        }
    }
}

impl Output {
    pub fn from_path<P: AsRef<Path>>(path: Option<P>) -> io::Result<Self> {
        match &path {
            Some(path) if path.as_ref() == Path::new("-") => Ok(Self::Stdout(io::stdout())),
            Some(path) => File::create(path).map(Self::File),
            None => Ok(Self::Stdout(io::stdout())),
        }
    }
}
