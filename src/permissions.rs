#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Allow,
    Ask,
    Deny,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Policy;

impl Policy {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&self, _action: &str) -> Permission {
        Permission::Allow
    }
}
