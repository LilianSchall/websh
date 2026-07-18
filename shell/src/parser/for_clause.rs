use super::do_group::DoGroup;
use super::in_clause::InClause;
use super::linebreak::Linebreak;
use super::name::Name;
use super::sequential_sep::SequentialSep;
use super::wordlist::Wordlist;

#[derive(Debug, Clone, PartialEq)]
pub enum ForClause {
    NoIn {
        name: Name,
        linebreak: Linebreak,
        do_group: DoGroup,
    },
    InNoWords {
        name: Name,
        linebreak: Linebreak,
        in_clause: InClause,
        sequential_sep: SequentialSep,
        do_group: DoGroup,
    },
    InWords {
        name: Name,
        linebreak: Linebreak,
        in_clause: InClause,
        wordlist: Wordlist,
        sequential_sep: SequentialSep,
        do_group: DoGroup,
    },
}
