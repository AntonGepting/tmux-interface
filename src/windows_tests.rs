#[test]
fn windows_parse() {
    use crate::Window;
    use crate::Windows;

    let windows_str =
        "1559064235'0'0'0''''1'64'@0'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'2''\
         c3bd,177x64,0,0,0'177'0\n\
         1559064235'0'0'0''''1'64'@1'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'2''c3bd,177x64,0,0,0'177'0";

    let windows = Windows::from_str(windows_str, Window::WINDOW_ALL).unwrap();
    assert_eq!(windows[0].id, Some(0));
    assert_eq!(windows[1].id, Some(1));

    let windows_str = "1559064235'0'0'0''''1'64'@0'1'0'c3bd,177x64,0,0,0'0'bash'''1'0'3''c3bd,\
        177x64,0,0,0'177'0\n\
        1559064235'0'0'0''''1'64'@1'2'0'8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'0'vim'''2'0'2\
        ''8b65,177x64,0,0[177x46,0,0,1,177x17,0,47,4]'177'0\n\
        1559064235'0'0'0'''-'1'64'@2'3'1'7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]'0'vim'''2'0'1\
        ''7966,177x64,0,0[177x52,0,0,2,177x11,0,53,3]'177'0\n\
        1559064235'0'1'0'''*'1'64'@4'4'0'c3c3,177x64,0,0,6'0'bash'''1'0'0''c3c3,177x64,0,0,6'177'0";
    let windows = Windows::from_str(windows_str, Window::WINDOW_ALL).unwrap();
    assert_eq!(windows[0].id, Some(0));
    assert_eq!(windows[1].id, Some(1));
    assert_eq!(windows[2].id, Some(2));
    assert_eq!(windows[3].id, Some(4));
}
