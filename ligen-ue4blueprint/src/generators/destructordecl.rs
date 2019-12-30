use ligen_cpp::ast::{DestructorDecl, Identifier};

pub struct DestructorDeclGenerator {}

impl DestructorDeclGenerator {
    pub fn generate(method : &ligen_core::Method) -> DestructorDecl {
        DestructorDecl::new(Identifier::new(&format!("U{}", method.owner.identifier.name)))
    }
}
