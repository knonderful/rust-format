use std::path::Path;

/// A textual I/O stream.
#[derive(Debug)]
pub enum IoStream {
    InvalidUtf8,
    Value(String),
}

impl From<Result<String, std::string::FromUtf8Error>> for IoStream {
    fn from(result: Result<String, std::string::FromUtf8Error>) -> Self {
        match result {
            Ok(string) => IoStream::Value(string),
            Err(_) => IoStream::InvalidUtf8,
        }
    }
}

/// A formatting error.
#[derive(Debug)]
pub enum Error {
    /// The 'rustfmt' tool is missing from the Rust toolchain.
    ToolMissing(&'static str),
    /// The 'rustfmt' tool terminated with a failure exit code.
    ToolExecutionError {
        /// The exit code.
        code: i32,
        /// The stdout stream.
        stdout: IoStream,
        /// The stderr stream.
        stderr: IoStream,
    },
    /// An I/O error occurred.
    IoError(std::io::Error),
    /// No result code was obtained. This can happen on Unix systems when the process is terminated
    /// by a signal.
    NoResultCode,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

/// Format a Rust source file.
///
/// # Arguments
///
/// * `path`: The path to the target file.
///
/// # Examples
///
/// ```
/// pub fn format_lib() {
///     use std::path::PathBuf;
///     rust_format::format_file(PathBuf::from("lib.rs")).unwrap();
/// }
/// ```
pub fn format_file(path: impl AsRef<Path>) -> Result<(), Error> {
    const TOOL_NAME: &'static str = "rustfmt";
    let rustfmt =
        toolchain_find::find_installed_component(TOOL_NAME).ok_or(Error::ToolMissing(TOOL_NAME))?;

    let process = std::process::Command::new(&rustfmt)
        .arg(path.as_ref().as_os_str())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let out = process.wait_with_output()?;
    let code = out.status.code().ok_or(Error::NoResultCode)?;
    if code != 0 {
        Err(Error::ToolExecutionError {
            code,
            stdout: String::from_utf8(out.stdout).into(),
            stderr: String::from_utf8(out.stderr).into(),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    #[test]
    fn test_format_file() {
        const TARGET_PATH: &str = "target/sample_code.rs";
        std::fs::copy("resources/test/sample_code.rs", TARGET_PATH).unwrap();
        super::format_file(&PathBuf::from(TARGET_PATH)).unwrap();

        use std::fs::read_to_string;
        assert_eq!(
            read_to_string("resources/test/expected.rs").unwrap(),
            read_to_string(TARGET_PATH).unwrap(),
        );
    }
}
