#pragma once

#include "CoreMinimal.h"
#include "UObject/NoExportTypes.h"
#include <Rectangle.h>
#include "URectangle.generated.h"

UCLASS(Blueprintable, BlueprintType)
class URectangle : public UObject {
public:
	GENERATED_BODY()
	~URectangle();

	UFUNCTION(BlueprintCallable, Category = "Rectangle")
	static URectangle* CreateRectangle(int width, int height);

	UFUNCTION(BlueprintCallable, Category = "Rectangle")
	int get_width() const;

	UFUNCTION(BlueprintCallable, Category = "Rectangle")
	int get_height() const;

	UFUNCTION(BlueprintCallable, Category = "Rectangle")
	static void print_type();

	UFUNCTION(BlueprintCallable, Category = "Rectangle")
	URectangle* add(const URectangle* rect) const;
private:
	Rectangle object;
	static URectangle* CreateRectangle(Rectangle object);
};
