use super::and_or::AndOr;
use super::separator::Separator;

#[derive(Debug, Clone, PartialEq)]
pub struct Term {
    pub head: Box<AndOr>,
    pub tail: Vec<(Separator, Box<AndOr>)>,
}
