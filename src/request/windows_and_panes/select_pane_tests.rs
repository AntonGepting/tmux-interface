#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn select_pane() {
    use crate::{Error, SelectPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
        // (alias: selectp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["select-pane", "-D", "-d", "-e", "-L", "-l", "-M", "-m", "-R", "-U", "-Z", "-T", "1", "-t", "2"]"#
        );
        Err(Error::new("hook"))
    }));
    let select_pane = SelectPane {
        down: Some(true),
        disable: Some(true),
        enable: Some(true),
        left: Some(true),
        last: Some(true),
        set_marked: Some(true),
        clear_marked: Some(true),
        right: Some(true),
        up: Some(true),
        keep_zoomed: Some(true),
        title: Some("1"),
        target_pane: Some("2"),
    };
    tmux.select_pane(Some(&select_pane)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn select_pane() {
    use crate::{Error, SelectPane, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux select-pane [-DdegLlMmRU] [-P style] [-T title] [-t target-pane]
        // (alias: selectp)
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["select-pane", "-D", "-d", "-e", "-g", "-L", "-l", "-M", "-m", "-R", "-U", "-P", "1", "-T", "2", "-t", "3"]"#
        );
        Err(Error::new("hook"))
    }));
    let select_pane = SelectPane {
        down: Some(true),
        disable: Some(true),
        enable: Some(true),
        show_style: Some(true),
        left: Some(true),
        last: Some(true),
        set_marked: Some(true),
        clear_marked: Some(true),
        right: Some(true),
        up: Some(true),
        style: Some("1"),
        title: Some("2"),
        target_pane: Some("3"),
    };
    tmux.select_pane(Some(&select_pane)).unwrap_err();
}