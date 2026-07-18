use super::compound_list::CompoundList;
use super::do_group::DoGroup;

#[derive(Debug, Clone, PartialEq)]
pub struct UntilClause {
    pub condition: CompoundList,
    pub do_group: DoGroup,
}
