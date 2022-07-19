// Copyright: Elliot Xu<hack00mind@gmail.com>
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use crate::parser::Rule;

use pest::error::{Error, LineColLocation};
use tower_lsp::lsp_types::{Diagnostic, Position, Range};

pub fn diagnostic(err: Error<Rule>) -> Option<Vec<Diagnostic>> {
    let range = get_error_range(&err);
    Some(vec![Diagnostic::new_simple(range, err.to_string())])
}

fn get_error_range(err: &Error<Rule>) -> Range {
    let ((start_line, start_col), (end_line, end_col)) = match err.line_col {
        LineColLocation::Pos((line, column)) => ((line, column), (line, column)),
        LineColLocation::Span((start_line, start_col), (end_line, end_col)) => {
            ((start_line, start_col), (end_line, end_col))
        }
    };

    // Range is zero-based
    Range::new(
        Position::new(start_line as u32 - 1, start_col as u32 - 1),
        Position::new(end_line as u32 - 1, end_col as u32 - 1),
    )
}

mod tests {
    use super::*;
    use crate::parser::{Rule, VimParser};
    use pest::Parser;

    #[test]
    fn test_error_position() {
        let source = r#"vim9script

var foo = 8

final bar = 8

error
        "#;
        let ast = VimParser::parse(Rule::vim9, source);
        assert!(ast.is_err());
        if let Err(error) = ast {
            assert_eq!(
                get_error_range(&error),
                Range::new(Position::new(6, 0), Position::new(6, 0))
            );
        }
    }
}
