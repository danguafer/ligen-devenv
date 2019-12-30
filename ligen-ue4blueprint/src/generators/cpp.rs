use ligen_cpp::ast::{AST, Statement, Macro, Identifier};
use crate::generators::{ClassImplGenerator};

pub struct CPPGenerator {}

impl CPPGenerator {
    pub fn generate(object : &ligen_core::Object) -> AST {
        let mut statements = Vec::new();
        statements.push(Statement::Macro(Macro::new(Identifier::new("include"), &format!("\"U{}.h\"", object.typ.identifier.name))));
        statements.append(&mut ClassImplGenerator::generate(&object, false).statements);
        AST::new(statements)
    }
}
