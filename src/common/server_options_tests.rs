#[test]
fn set_clipboard() {
    use crate::common::server_options::SetClipboard;

    assert_eq!(SetClipboard::On.to_string(), "on");
    assert_eq!(SetClipboard::Off.to_string(), "off");
    assert_eq!(SetClipboard::External.to_string(), "external");
}

#[test]
fn parse() {
    use crate::ServerOptions;

    let server_options_str = r#"
        buffer-limit 50
        command-alias[0] "split-pane=split-window"
        command-alias[1] "splitp=split-window"
        command-alias[2] "server-info=show-messages -JT"
        command-alias[3] "info=show-messages -JT"
        command-alias[4] "choose-window=choose-tree -w"
        command-alias[5] "choose-session=choose-tree -s"
        default-terminal "screen-256color"
        escape-time 500
        exit-empty on
        exit-unattached off
        focus-events off
        history-file ""
        message-limit 100
        set-clipboard external
        terminal-overrides[0] ""
        terminal-overrides[1] ""
    "#;
    let _server_options = server_options_str.parse::<ServerOptions>().unwrap();
    //dbg!(&server_options);
}

#[test]
fn to_string() {
    use crate::ServerOptionsBuilder;

    let _server_options = ServerOptionsBuilder::new().buffer_limit(50).build();
    //dbg!(&server_options.to_string());
}

#[test]
fn get_all() {
    use crate::ServerOptions;
    let _server_options = ServerOptions::get_all().unwrap();
}

#[test]
fn get_single() {
    use crate::common::server_options::ESCAPE_TIME;
    use crate::ServerOptions;
    let server_options = ServerOptions::get(ESCAPE_TIME).unwrap();
    //assert_eq!(server_options.escape_time, Some(500));
}

//#[test]
//fn set_single() {
//use crate::common::server_options::ESCAPE_TIME;
//use crate::{ServerOptions, ServerOptionsBuilder};

//let server_options = ServerOptionsBuilder::new().escape_time(600).build();
//server_options.set(ESCAPE_TIME).unwrap();
//let server_options = ServerOptions::get(ESCAPE_TIME).unwrap();
//assert_eq!(server_options.escape_time, Some(600));

//let server_options = ServerOptionsBuilder::new().escape_time(500).build();
//server_options.set(ESCAPE_TIME).unwrap();
//}
