use ligen_cpp::ast::{AST, Statement, FunctionStatement, MemberFunction, MemberFunctionImpl, Expression, Assignment, Type, Modifier, Parameter, Parameters, Constructor, FunctionBlock, ConstructorImpl, Identifier, EMemberFunctionImpl};
use crate::generators::{EMemberFunctionImplGenerator};

pub struct ClassImplGenerator {}

impl ClassImplGenerator {
    pub fn generate(object : &ligen_core::Object, sized_integer : bool) -> AST {
        let mut statements = Vec::new();

        statements.push(Statement::Uncategorized(format!("U{name}* U{name}::Create{name}({name} object) {{\n\tU{name}* ret_value = NewObject<U{name}>(U{name}::StaticClass());\n\tret_value->object = object;\n\treturn ret_value;\n}}", name = object.typ.identifier.name)));
        for method in &object.methods {
            statements.append(&mut EMemberFunctionImplGenerator::generate(&method, sized_integer));
        }
        AST::new(statements)
    }
}
