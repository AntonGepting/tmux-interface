use crate::TmuxInterface;


#[test]
fn new() {
    let tmux = TmuxInterface::new();
    assert_eq!(tmux.tmux, None);
    let mut tmux = TmuxInterface::new();
    tmux.tmux = Some("tmux_mock");
    assert_eq!(tmux.tmux, Some("tmux_mock"));
}


#[test]
fn exec_io_error() {
    let mut tmux = TmuxInterface::new();
    tmux.tmux = Some("nonexistent_binary");
    let err = tmux.version().unwrap_err();
    assert_eq!(err.message, "io");
}


#[test]
fn subcommand() {
    let tmux = TmuxInterface::new();
    tmux.subcommand("has", &["-t", "session_name"]).unwrap();
}


#[test]
fn version() {
    let tmux = TmuxInterface::new();
    let version = tmux.version().unwrap();
    assert_eq!(version.prog_name, "tmux");
    assert!(version.major >= 1);
}


#[test]
fn version_io_error() {
    let mut tmux = TmuxInterface::new();
    tmux.tmux = Some("nonexistent_binary");
    let err = tmux.version().unwrap_err();
    assert_eq!(err.message, "io");
}


#[test]
fn version_parse_error() {
    let mut tmux = TmuxInterface::new();
    tmux.tmux = Some("./tests/tmux_error_mock.sh");
    let err = tmux.version().unwrap_err();
    assert_eq!(err.message, "parse num");
}
