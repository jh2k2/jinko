//! A `ScopeMap` is a set of "scopes" used to keep track of the available variables and
//! functions in a given scope.
//! In order to access variables and functions, the scope map first looks in the current
//! scope. If the specified name cannot be found, it searches the other scopes, defined
//! before the current one, until it finds the correct component.

use std::collections::{HashMap, LinkedList};
use std::rc::Rc;

use crate::{
    error::{ErrKind, JinkoError},
    instruction::{FunctionDec, Var},
};

/// A scope contains a set of available variables and functions
struct Scope {
    variables: HashMap<String, Var>,
    functions: HashMap<String, Rc<FunctionDec>>,
}

impl Scope {
    /// Create a new empty Scope
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    /// Get a reference on a variable from the scope map if is has been inserted already
    pub fn get_variable(&self, name: &str) -> Option<&Var> {
        self.variables.get(name)
    }

    /// Get a reference on a function from the scope map if is has been inserted already
    pub fn get_function(&self, name: &str) -> Option<&Rc<FunctionDec>> {
        self.functions.get(name)
    }

    /// Add a variable to the most recently created scope, if it doesn't already exist
    pub fn add_variable(&mut self, var: Var) -> Result<(), JinkoError> {
        match self.get_variable(var.name()) {
            Some(_) => Err(JinkoError::new(
                ErrKind::Interpreter,
                format!("variable already declared: {}", var.name()),
                None,
                var.name().to_owned(),
            )),
            None => Ok({
                self.variables.insert(var.name().to_owned(), var);
            }),
        }
    }

    /// Add a variable to the most recently created scope, if it doesn't already exist
    pub fn add_function(&mut self, func: FunctionDec) -> Result<(), JinkoError> {
        match self.get_function(func.name()) {
            Some(_) => Err(JinkoError::new(
                ErrKind::Interpreter,
                format!("function already declared: {}", func.name()),
                None,
                func.name().to_owned(),
            )),
            None => Ok({
                self.functions.insert(func.name().to_owned(), Rc::new(func));
            }),
        }
    }
}

/// A scope stack is a reversed stack. This alias is made for code clarity
type ScopeStack<T> = LinkedList<T>;

/// A scope map keeps track of the currently available scopes and the current depth
/// level.
pub struct ScopeMap {
    scopes: ScopeStack<Scope>,
}

impl ScopeMap {
    /// Create a new empty scope map, at depth 0
    pub fn new() -> ScopeMap {
        ScopeMap {
            scopes: ScopeStack::new(),
        }
    }

    /// Enter into a new scope
    pub fn scope_enter(&mut self) {
        self.scopes.push_front(Scope::new());
    }

    /// Exit the last added scope
    pub fn scope_exit(&mut self) {
        // We unwrap since we want the interpreter to crash in case we pop an unexisting
        // scope.
        self.scopes.pop_front().unwrap();
    }

    /// Maybe get a variable in any available scopes
    pub fn get_variable(&self, name: &str) -> Option<&Var> {
        // FIXME: Use find for code quality?
        for scope in self.scopes.iter() {
            match scope.get_variable(name) {
                Some(v) => return Some(v),
                None => continue,
            };
        }

        None
    }

    /// Maybe get a function in any available scopes
    pub fn get_function(&self, name: &str) -> Option<&Rc<FunctionDec>> {
        // FIXME: Use find for code quality?
        for scope in self.scopes.iter() {
            match scope.get_function(name) {
                Some(v) => return Some(v),
                None => continue,
            };
        }

        None
    }

    /// Add a variable to the current scope if it hasn't been added before
    pub fn add_variable(&mut self, var: Var) -> Result<(), JinkoError> {
        match self.scopes.front_mut() {
            Some(head) => head.add_variable(var),
            None => Err(JinkoError::new(
                ErrKind::Interpreter,
                String::from("Adding variable to empty scopemap"),
                None,
                var.name().to_owned(),
            )),
        }
    }

    /// Add a function to the current scope if it hasn't been added before
    pub fn add_function(&mut self, func: FunctionDec) -> Result<(), JinkoError> {
        match self.scopes.front_mut() {
            Some(head) => head.add_function(func),
            None => Err(JinkoError::new(
                ErrKind::Interpreter,
                String::from("Adding function to empty scopemap"),
                None,
                func.name().to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn t_pop_non_existent_scope() {
        let mut s = ScopeMap::new();

        s.scope_enter();
        s.scope_exit();
        s.scope_exit();
    }

    #[test]
    #[should_panic]
    fn t_add_var_non_existent_scope() {
        let mut s = ScopeMap::new();

        s.add_variable(Var::new("Something".to_owned())).unwrap();
    }

    #[test]
    fn t_find_non_existent_var() {
        let s = ScopeMap::new();

        assert!(s.get_variable("a").is_none());
    }

    #[test]
    fn t_add_var_and_get_it() {
        let mut s = ScopeMap::new();

        s.scope_enter();
        s.add_variable(Var::new("a".to_owned())).unwrap();

        assert!(s.get_variable("a").is_some());
    }

    #[test]
    fn t_add_var_and_get_it_from_inner_scope() {
        let mut s = ScopeMap::new();

        s.scope_enter();
        s.add_variable(Var::new("a".to_owned())).unwrap();

        s.scope_enter();
        s.scope_enter();
        s.scope_enter();
        s.scope_enter();
        s.scope_enter();

        assert!(s.get_variable("a").is_some());
    }

    #[test]
    fn t_add_var_and_get_it_from_outer_scope() {
        let mut s = ScopeMap::new();

        s.scope_enter();

        s.add_variable(Var::new("a".to_owned())).unwrap();

        s.scope_exit();

        assert!(s.get_variable("a").is_none());
    }
}
