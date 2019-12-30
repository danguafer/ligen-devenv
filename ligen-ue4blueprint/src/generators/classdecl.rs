use ligen_cpp::ast::{ClassDecl, Identifier, EMemberFunctionDecl, Inheritance, Access, ConstructorDecl, Constructor, Parameters, Parameter, Type, Delete, Modifier, MemberFunction, MemberFunctionDecl, ClassStatement};
use crate::generators::{EMemberFunctionDeclGenerator};

pub struct ClassDeclGenerator {}

impl ClassDeclGenerator {
    pub fn generate(object : &ligen_core::Object) -> ClassDecl {
        let name = &object.typ.identifier.name;
        let identifier = Identifier::new(&format!("U{}", name));
        let lower_name = name.to_lowercase();
        let inheritances = vec![Inheritance::new(Access::Public, Identifier::new("UObject"))];
        let mut statements = Vec::new();

        statements.push(ClassStatement::Access(Access::Public));
        statements.push(ClassStatement::Uncategorized("GENERATED_BODY()".into()));

        for method in &object.methods {
            statements.append(&mut EMemberFunctionDeclGenerator::generate(&method, false))
        }

        statements.push(ClassStatement::Uncategorized(format!("static U{name}* Create{name}({name} object);", name = name)));
        statements.push(ClassStatement::Uncategorized(format!("{} object;", name)));
        statements.push(ClassStatement::Uncategorized(format!("void BeginDestroy();")));

        ClassDecl::new(identifier, inheritances, statements)
    }
}
