use super::brace_group::BraceGroup;
use super::case_clause::CaseClause;
use super::for_clause::ForClause;
use super::if_clause::IfClause;
use super::subshell::Subshell;
use super::until_clause::UntilClause;
use super::while_clause::WhileClause;

#[derive(Debug, Clone, PartialEq)]
pub enum CompoundCommand {
    BraceGroup(BraceGroup),
    Subshell(Subshell),
    ForClause(ForClause),
    CaseClause(CaseClause),
    IfClause(IfClause),
    WhileClause(WhileClause),
    UntilClause(UntilClause),
}
