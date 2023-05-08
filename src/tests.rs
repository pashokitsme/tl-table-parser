use crate::{parse_all, parse_first, Table};

#[test]
fn simple() {
  const HTML: &str = r#"
    <table>
        <tr><th>Header</th><th>Value</th></tr>
        <tr><td>A</td><td>B</td></tr>
    </table>
  "#;
  let expected = Some(vec![vec!["Header".to_string(), "Value".to_string()], vec!["A".to_string(), "B".to_string()]]);
  let result = parse_first(HTML);
  assert_eq!(expected, result.map(|r| r.rows));
}

#[test]
fn multiple_tables() {
  const HTML: &str = r#"
    <table>
        <tr><th>Header</th><th>Value</th></tr>
        <tr><td>A</td><td>B</td></tr>
    </table>
    <table>
        <tr><th>Header</th><th>Value</th></tr>
        <tr><td>A</td><td>B</td></tr>
    </table>
  "#;
  let expected = Some(vec![
    Table { rows: vec![vec!["Header".to_string(), "Value".to_string()], vec!["A".to_string(), "B".to_string()]] },
    Table { rows: vec![vec!["Header".to_string(), "Value".to_string()], vec!["A".to_string(), "B".to_string()]] },
  ]);
  let result = parse_all(HTML);
  assert_eq!(expected, result);
}

#[test]
fn no_table() {
  const HTML: &str = "<div>Hey</div>";
  let expected = None;
  let result = parse_all(HTML);
  assert_eq!(expected, result);
}

#[test]
fn no_rows() {
  const HTML: &str = "<table>Hey</table>";
  let expected = None;
  let result = parse_first(HTML);
  assert_eq!(expected, result.map(|r| r.rows));
}
