use crate::pipeline::input::Expr;

use super::input::Input;

#[test]
fn test_input_parse_file() {
    let input = r#"
        src: file
        name: foo
        uniform: bar
    "#;
    let input: Input = serde_yaml::from_str(input).unwrap();

    let expected = Input::File {
        name: "foo".into(),
        uniform: "bar".into(),
    };

    assert_eq!(input, expected);
}

#[test]
fn test_input_parse_memory() {
    let input = r#"
        src: memory
        name: foo
        uniform: bar
    "#;
    let input: Input = serde_yaml::from_str(input).unwrap();

    let expected = Input::Memory {
        name: "foo".into(),
        uniform: "bar".into(),
    };

    assert_eq!(input, expected);
}

#[test]
fn test_input_parse_expr() {
    let input = r#"
        src: expr
        expr: #ff00ff
        uniform: bar
    "#;
    let input: Input = serde_yaml::from_str(input).unwrap();

    let expected = Input::Expr {
        uniform: "bar".into(),
        expr: Expr::String("#ff00ff".into()),
    };

    assert_eq!(input, expected);
}
