use super::compound_list::CompoundList;

#[derive(Debug, Clone, PartialEq)]
pub enum ElsePart {
	Elif {
		condition: CompoundList,
		then_part: Box<ElsePart>,
	},
	Else(CompoundList),
}
