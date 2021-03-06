//! Represents the usage of a variable, for example when returning from
//! a block.

use super::{InstrKind, Instruction};
use crate::{Interpreter, JinkoError};

#[derive(Clone)]
pub struct Var {
    name: String,
}

impl Var {
    /// Create a new variable usage with the given name
    pub fn new(name: String) -> Var {
        Var { name }
    }

    /// Return the name of the variable
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Instruction for Var {
    fn kind(&self) -> InstrKind {
        // FIXME: Add logic
        InstrKind::Statement
    }

    fn print(&self) -> String {
        self.name.clone()
    }

    fn execute(&self, interpreter: &mut Interpreter) -> Result<(), JinkoError> {
        // FIXME: Add logic

        interpreter.debug("VAR", self.name());

        Ok(())
    }
}

impl Default for Var {
    fn default() -> Self {
        Var::new(String::new())
    }
}
