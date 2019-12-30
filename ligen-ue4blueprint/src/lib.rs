use ligen::ligen;
use ligen_core::{Object, Files, File, Attribute, Identifier, Literal, Attributes, LiteralConverter};

pub mod generators;
use generators::*;

pub struct Generator {
    c_generator : ligen_c::Generator
}

#[ligen]
impl Generator {
    pub fn new(attributes: &Attributes) -> Generator {
        let c_attributes = Attributes::from_vec(vec![
            Attribute::Named(Identifier::new("sized_integer"), Literal::Bool(false))
        ]);
        let c_generator = ligen_c::Generator::new(&c_attributes);

        Generator { c_generator }
    }

    pub fn generate(&self, object: &Object) -> Files {
        let mut files = self.c_generator.generate(&object);

        let h = format!("{}", HGenerator::generate(&object));
        let cpp = format!("{}", CPPGenerator::generate(&object));
        let h = File::new(format!("ue4/include/U{}.h", object.typ.identifier.name), h);
        let cpp = File::new(format!("ue4/src/U{}.cpp", object.typ.identifier.name), cpp);

        files.push(h);
        files.push(cpp);
        files
    }
}
