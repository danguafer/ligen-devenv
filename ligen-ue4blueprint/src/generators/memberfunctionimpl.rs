use ligen_cpp::ast::{Statement, EMemberFunctionImpl, Args, MemberFunctionImpl, FunctionStatement, Expression, Identifier, FunctionBlock, FunctionCall, Modifier, Assignment, Type};
use crate::generators::{MemberFunctionGenerator, ConstructorImplGenerator, DestructorImplGenerator, ArgsGenerator};

pub struct EMemberFunctionImplGenerator {}

impl EMemberFunctionImplGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> Vec<Statement> {
        let mut statements = Vec::new();
        match method.identifier.name.as_str() {
            "destroy" => {
                statements.push(Statement::Uncategorized(format!("void U{name}::BeginDestroy() {{ Super::BeginDestroy(); if (object.self) {name}_destroy(object); }}", name = method.owner.identifier.name)));
                statements.push(Statement::Uncategorized(format!("void U{name}::destroy() {{ ConditionalBeginDestroy(); }}", name = method.owner.identifier.name)));
            },
            _ => statements.push(Statement::MemberFunctionImpl(EMemberFunctionImpl::MemberFunction(MemberFunctionImplGenerator::generate(&method, sized_integer))))
        }

        statements
    }
}

pub struct MemberFunctionImplGenerator {}

impl MemberFunctionImplGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> MemberFunctionImpl {
        let class_identifier = Identifier::new(&format!("U{}", method.owner.identifier.name));
        let mut statements = Vec::new();
        let mut args = ArgsGenerator::generate(&method.inputs);

        let is_moving_self = if let Some(typ) = &method.inputs.self_type {
            args.args.insert(0, Identifier::new("object"));
            match &typ.modifier {
                ligen_core::TypeModifier::None => true,
                _ => false
            }
        } else { false };

        let function_call = FunctionCall::new(Identifier::new(&format!("{}_{}", method.owner.identifier.name, method.identifier.name)), args);
        if let Some(typ) = &method.output.typ {
            let expression = if !typ.is_atomic() {
                let args = Args::new(vec![Identifier::new(&format!("{}", function_call))]);
                let constructor_call = FunctionCall::new(Identifier::new(&format!("U{name}::Create{name}", name = typ.identifier.name)), args);
                Expression::FunctionCall(constructor_call)
            } else {
                Expression::FunctionCall(function_call)
            };
            statements.push(FunctionStatement::Assignment(Assignment::new(Identifier::new("auto ret_value"), expression)));
            if is_moving_self {
                statements.push(FunctionStatement::Assignment(Assignment::new(Identifier::new("object.self"), Expression::Identifier(Identifier::new("0")))));
            }
            for input in &method.inputs.inputs {
                match &input.typ.modifier {
                    ligen_core::TypeModifier::None => {
                        if (!input.typ.is_atomic()) {
                            statements.push(FunctionStatement::Assignment(Assignment::new(Identifier::new(&format!("{}->object.self", input.identifier.name)), Expression::Identifier(Identifier::new("0")))))
                        }
                    },
                    _ => ()
                }
            }
            statements.push(FunctionStatement::Return(Expression::Identifier(Identifier::new("ret_value"))));
        } else {
            statements.push(FunctionStatement::Expression(Expression::FunctionCall(function_call)));
        }
        let mut function_block = FunctionBlock::new(statements);
        MemberFunctionImpl::new(class_identifier, MemberFunctionGenerator::generate(&method, sized_integer), function_block)
    }
}
