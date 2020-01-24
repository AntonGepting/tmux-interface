#[cfg(not(feature = "tmux_2_6"))]
#[test]
fn choose_buffer() {
    use crate::{ChooseBuffer, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-buffer [-NZr] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-buffer", "-N", "-Z", "-r", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::new("hook"))
    }));
    let choose_buffer = ChooseBuffer {
        no_preview: Some(true),
        zoom: Some(true),
        reverse_sort_order: Some(true),
        format: Some("1"),
        filter: Some("2"),
        sort_order: Some("3"),
        target_pane: Some("4"),
        template: Some("5"),
    };
    tmux.choose_buffer(Some(&choose_buffer)).unwrap_err();
}

#[cfg(feature = "tmux_2_6")]
#[test]
fn choose_buffer() {
    use crate::{ChooseBuffer, Error, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        // tmux choose-buffer [-N] [-F format] [-f filter] [-O sort-order] [-t target-pane] [template]
        assert_eq!(
            format!(r#"{:?} {:?} {:?}"#, bin, options, subcmd),
            r#""tmux" [] ["choose-buffer", "-N", "-F", "1", "-f", "2", "-O", "3", "-t", "4", "5"]"#
        );
        Err(Error::new("hook"))
    }));
    let choose_buffer = ChooseBuffer {
        no_preview: Some(true),
        format: Some("1"),
        filter: Some("2"),
        sort_order: Some("3"),
        target_pane: Some("4"),
        template: Some("5"),
    };
    tmux.choose_buffer(Some(&choose_buffer)).unwrap_err();
}