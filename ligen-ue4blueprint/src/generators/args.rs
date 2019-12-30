use ligen_cpp::ast::{Args, Identifier};

pub struct ArgsGenerator {}

impl ArgsGenerator {
    pub fn generate(inputs : &ligen_core::Inputs) -> Args {
        let mut args = Vec::new();
        for input in &inputs.inputs {
            if input.typ.is_atomic() {
                args.push(Identifier::new(&input.identifier.name));
            } else {
                args.push(Identifier::new(&format!("{}->object", input.identifier.name)));
            }
        }
        Args::new(args)
    }
}
