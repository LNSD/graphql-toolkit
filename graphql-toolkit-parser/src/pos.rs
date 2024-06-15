use std::str::Chars;

use graphql_toolkit_ast::Pos;
use pest::{iterators::Pair, RuleType};

pub(crate) struct PositionCalculator<'a> {
    input: Chars<'a>,
    pos: usize,
    line: usize,
    column: usize,
}

impl<'a> PositionCalculator<'a> {
    pub(crate) fn new(input: &'a str) -> PositionCalculator<'a> {
        Self {
            input: input.chars(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub(crate) fn step<R: RuleType>(&mut self, pair: &Pair<R>) -> Pos {
        let pos = pair.as_span().start();
        debug_assert!(pos >= self.pos);
        for _ in 0..pos - self.pos {
            match self.input.next() {
                Some('\r') => {
                    self.column = 1;
                }
                Some('\n') => {
                    self.line += 1;
                    self.column = 1;
                }
                Some(_) => {
                    self.column += 1;
                }
                None => break,
            }
        }
        self.pos = pos;
        Pos {
            line: self.line,
            column: self.column,
        }
    }
}
