use super::case_item_ns::CaseItemNs;

#[derive(Debug, Clone, PartialEq)]
pub struct CaseListNs(pub Vec<CaseItemNs>);
