use super::fname::Fname;
use super::function_body::FunctionBody;
use super::linebreak::Linebreak;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub fname: Fname,
    pub linebreak: Linebreak,
    pub body: FunctionBody,
}
