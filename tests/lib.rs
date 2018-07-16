extern crate enquote;

#[test]
fn enquote() {
    assert_eq!(
        enquote::enquote('"', r#""Fran & Freddie's Diner	☺\""#),
        r#""\"Fran & Freddie's Diner	☺\\\"""#,
    );
    assert_eq!(enquote::enquote('"', ""), r#""""#);
    assert_eq!(enquote::enquote('"', r#"""#), r#""\"""#);

    assert_eq!(
        enquote::enquote('\'', r#""Fran & Freddie's Diner	☺\""#),
        r#"'"Fran & Freddie\'s Diner	☺\\"'"#,
    );
    assert_eq!(enquote::enquote('\'', ""), "''");
    assert_eq!(enquote::enquote('\'', "'"), r#"'\''"#);

    assert_eq!(enquote::enquote('`', ""), "``");
    assert_eq!(enquote::enquote('`', "`"), r#"`\``"#);
}
