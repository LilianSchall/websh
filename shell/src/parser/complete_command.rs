use super::list::List;
use super::separator::Separator;

#[derive(Debug, Clone, PartialEq)]
pub struct CompleteCommand {
	pub list: List,
	pub separator: Option<Separator>,
}