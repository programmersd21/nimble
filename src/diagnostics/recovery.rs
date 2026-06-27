use crate::diagnostics::codes::ErrorCode;
use crate::diagnostics::diagnostic::Diagnostic;
use std::collections::HashSet;

#[derive(Default)]
pub struct RecoveryState {
    in_recovery: bool,
    suppressed_vars: HashSet<String>,
}

impl RecoveryState {
    pub fn new() -> Self {
        RecoveryState::default()
    }

    pub fn enter_recovery(&mut self) {
        self.in_recovery = true;
    }

    pub fn exit_recovery(&mut self) {
        self.in_recovery = false;
    }

    pub fn add_suppressed_var(&mut self, var: String) {
        self.suppressed_vars.insert(var);
    }

    pub fn should_suppress(&self, diagnostic: &Diagnostic) -> bool {
        if let Some(code) = diagnostic.code {
            match code {
                ErrorCode::N2001 => {
                    // Suppress "undefined variable" errors for vars already known to be broken/suppressed
                    for var in &self.suppressed_vars {
                        if diagnostic.message.contains(var) {
                            return true;
                        }
                    }
                }
                ErrorCode::N3001 => {
                    // Suppress "type mismatch" errors cascading from known broken variables
                    for var in &self.suppressed_vars {
                        if diagnostic.message.contains(var) {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        false
    }
}
