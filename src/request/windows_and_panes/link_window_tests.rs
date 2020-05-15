#[test]
fn link_window() {
    use crate::{Error, LinkWindow, LinkWindowBuilder, TargetWindow, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux ^2.1:
        // ```text
        // tmux link-window [-adk] [-s src-window] [-t dst-window]
        // (alias: linkw)
        // ```
        //
        // tmux ^0.8:
        // ```text
        // tmux link-window [-dk] [-s src-window] [-t dst-window]
        // (alias: linkw)
        // ```
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["link-window", "-a", "-d", "-k", "-s", "1", "-t", "2"]"#
        );
        Err(Error::Hook)
    }));

    let link_window = LinkWindow {
        #[cfg(feature = "tmux_2_1")]
        add: Some(true),
        detached: Some(true),
        kill: Some(true),
        src_window: Some(&TargetWindow::Raw("1")),
        dst_window: Some(&TargetWindow::Raw("2")),
    };
    tmux.link_window(Some(&link_window)).unwrap_err();

    let mut builder = LinkWindowBuilder::new();
    #[cfg(feature = "tmux_2_1")]
    builder.add();
    builder.detached();
    builder.kill();
    builder.src_window(&TargetWindow::Raw("1"));
    builder.dst_window(&TargetWindow::Raw("2"));
    let link_window = builder.build();
    tmux.link_window(Some(&link_window)).unwrap_err();
}
