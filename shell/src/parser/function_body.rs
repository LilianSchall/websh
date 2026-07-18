use super::compound_command::CompoundCommand;
use super::redirect_list::RedirectList;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    pub compound_command: CompoundCommand,
    pub redirects: Option<RedirectList>,
}
