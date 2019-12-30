use ligen_cpp::ast::*;
use crate::generators::*;

pub struct HGenerator {}

impl HGenerator {
    pub fn generate(object : &ligen_core::Object) -> AST {
        let name = &object.typ.identifier.name;
        let mut ast = Vec::new();
        ast.push(Statement::Macro(Macro::new(Identifier::new("pragma"), "once")));
        ast.push(Statement::Macro(Macro::new(Identifier::new("include"), "\"CoreMinimal.h\"")));
        ast.push(Statement::Macro(Macro::new(Identifier::new("include"), "\"UObject/NoExportTypes.h\"")));
        ast.push(Statement::Macro(Macro::new(Identifier::new("include"), &format!("<{}.h>", name))));
        for dependency in &object.dependencies { ast.push(Statement::Macro(Macro::new(Identifier::new("include"), &format!("\"U{}.h\"", dependency.name)))); }
        ast.push(Statement::Macro(Macro::new(Identifier::new("include"), &format!("\"U{}.generated.h\"", name))));
        ast.push(Statement::Uncategorized(String::from("UCLASS(Blueprintable, BlueprintType)")));
        ast.push(Statement::ClassDecl(ClassDeclGenerator::generate(&object)));

        AST::new(ast)
    }
}

// #pragma once
//
// #include "CoreMinimal.h"
// #include "UObject/NoExportTypes.h"
// #include <Rectangle.h>
// #include "URectangle.generated.h"
//
// UCLASS(Blueprintable, BlueprintType)
// class URectangle : public UObject {
// public:
// 	GENERATED_BODY()
// 	~URectangle();
//
// 	UFUNCTION(BlueprintCallable, Category = "Rectangle")
// 	static URectangle* CreateRectangle(int width, int height);
//
// 	UFUNCTION(BlueprintCallable, Category = "Rectangle")
// 	int get_width() const;
//
// 	UFUNCTION(BlueprintCallable, Category = "Rectangle")
// 	int get_height() const;
//
// 	UFUNCTION(BlueprintCallable, Category = "Rectangle")
// 	static void print_type();
//
// 	UFUNCTION(BlueprintCallable, Category = "Rectangle")
// 	URectangle* add(const URectangle* rect) const;
// private:
// 	Rectangle object;
// 	static URectangle* CreateRectangle(Rectangle object);
// };
