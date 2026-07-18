use super::compound_list::CompoundList;
use super::else_part::ElsePart;

#[derive(Debug, Clone, PartialEq)]
pub struct IfClause {
    pub condition: CompoundList,
    pub then_body: CompoundList,
    pub else_part: Option<ElsePart>,
}
