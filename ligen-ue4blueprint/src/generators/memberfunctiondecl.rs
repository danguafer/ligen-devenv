use ligen_cpp::ast::{EMemberFunctionDecl, Delete, MemberFunctionDecl, ClassStatement};
use crate::generators::{MemberFunctionGenerator, ConstructorDeclGenerator, DestructorDeclGenerator};

pub struct EMemberFunctionDeclGenerator {}

impl EMemberFunctionDeclGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> Vec<ClassStatement> {
        let mut statements = Vec::new();
        let (blueprintable, member_function) = match method.identifier.name.as_str() {
            _ => (true, EMemberFunctionDecl::MemberFunction(MemberFunctionDeclGenerator::generate(&method, sized_integer), Delete::False))
        };
        if blueprintable {
            statements.push(ClassStatement::Uncategorized(format!("UFUNCTION(BlueprintCallable, Category = \"{}\")", method.owner.identifier.name)));
        }
        statements.push(ClassStatement::MemberFunction(member_function));
        statements
    }
}

pub struct MemberFunctionDeclGenerator {}

impl MemberFunctionDeclGenerator {
    pub fn generate(method : &ligen_core::Method, sized_integer : bool) -> MemberFunctionDecl {
        let staticness = if let Some(_ty) = &method.inputs.self_type { false } else { true };
        MemberFunctionDecl::new(staticness, MemberFunctionGenerator::generate(&method, sized_integer))
    }
}
