use super::compound_list::CompoundList;
use super::linebreak::Linebreak;
use super::pattern::Pattern;

#[derive(Debug, Clone, PartialEq)]
pub struct CaseItemNs {
	pub has_open_paren: bool,
	pub pattern: Pattern,
	pub compound_list: Option<CompoundList>,
	pub linebreak: Linebreak,
}
