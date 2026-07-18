use super::case_list::CaseList;
use super::case_list_ns::CaseListNs;
use super::in_clause::InClause;
use super::linebreak::Linebreak;

#[derive(Debug, Clone, PartialEq)]
pub enum CaseClauseBody {
    CaseList(CaseList),
    CaseListNs(CaseListNs),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaseClause {
    pub word: String,
    pub before_in_linebreak: Linebreak,
    pub in_clause: InClause,
    pub after_in_linebreak: Linebreak,
    pub body: CaseClauseBody,
}
