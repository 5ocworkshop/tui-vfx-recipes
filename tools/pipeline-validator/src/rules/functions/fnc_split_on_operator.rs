// <FILE>tools/pipeline-validator/src/rules/functions/fnc_split_on_operator.rs</FILE> - <DESC>Split expression on operator handling nesting</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Pipeline debugging tools - OFPF refactor</WCTX>
// <CLOG>Extracted from evaluator.rs</CLOG>

/// Split expression on an operator (handles nested parentheses and quotes)
pub fn split_on_operator<'a>(expr: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let mut depth = 0;
    let mut in_quotes = false;
    let mut quote_char = ' ';

    let chars: Vec<char> = expr.chars().collect();
    let op_chars: Vec<char> = op.chars().collect();

    for i in 0..chars.len() {
        match chars[i] {
            '(' if !in_quotes => depth += 1,
            ')' if !in_quotes => depth -= 1,
            '"' | '\'' => {
                if !in_quotes {
                    in_quotes = true;
                    quote_char = chars[i];
                } else if chars[i] == quote_char {
                    in_quotes = false;
                }
            }
            _ => {}
        }

        if depth == 0 && !in_quotes {
            // Check if operator starts here
            if i + op_chars.len() <= chars.len() {
                let slice: String = chars[i..i + op_chars.len()].iter().collect();
                if slice == op {
                    return Some((&expr[..i], &expr[i + op_chars.len()..]));
                }
            }
        }
    }

    None
}

// <FILE>tools/pipeline-validator/src/rules/functions/fnc_split_on_operator.rs</FILE> - <DESC>Split expression on operator handling nesting</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
