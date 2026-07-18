use super::newline_list::NewlineList;
use super::separator::Separator;
use super::term::Term;

#[derive(Debug, Clone, PartialEq)]
pub struct CompoundList {
    pub leading_newlines: Option<NewlineList>,
    pub term: Term,
    pub trailing_separator: Option<Separator>,
}
