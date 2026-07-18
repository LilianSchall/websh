use super::compound_list::CompoundList;

#[derive(Debug, Clone, PartialEq)]
pub struct Subshell {
	pub compound_list: CompoundList,
}
