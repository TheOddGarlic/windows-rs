use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeRef(pub ScopeKey);

impl TypeRef {
    pub fn name<'a>(&self, scope: &'a Scope) -> &'a str {
        scope.str(self.0, 1)
    }
    pub fn namespace<'a>(&self, scope: &'a Scope) -> &'a str {
        scope.str(self.0, 2)
    }
    pub fn type_name<'a>(&self, scope: &'a Scope) -> TypeName<'a> {
        TypeName::new(self.namespace(scope), self.name(scope))
    }
}