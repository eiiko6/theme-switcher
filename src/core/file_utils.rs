use dirs::home_dir;
use std::io;

pub fn expand_tilde(path: &str) -> io::Result<String> {
    if let Some(home) = home_dir() {
        Ok(path.replace("~/", &format!("{}/", home.display())))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "could not determine home directory",
        ))
    }
}
