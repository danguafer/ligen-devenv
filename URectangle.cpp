#include "URectangle.h"

URectangle* URectangle::CreateRectangle(Rectangle object) {
	URectangle* rectangle = NewObject<URectangle>(URectangle::StaticClass());
	rectangle->object = object;
	return rectangle;
}

URectangle* URectangle::CreateRectangle(int width, int height) {
	return CreateRectangle(Rectangle_new(width, height));
}

URectangle::~URectangle() {
	Rectangle_destroy(object);
}

int URectangle::get_width() const {
	return Rectangle_get_width(object);
}

int URectangle::get_height() const {
	return Rectangle_get_height(object);
}

void URectangle::print_type() {
	Rectangle_print_type();
}

URectangle* URectangle::add(const URectangle* rectangle) const {
	return CreateRectangle(Rectangle_add(object, rectangle->object));
}
