#include <iostream>
#include <ContextBuilder.hpp>

int main(int argc, char **argv) {
  Context context = ContextBuilder().cursor(false).with_display(ContextDisplay::window((int8_t*) "Window name", 800, 600)).build();
  while (context.run());
  return 0;
}
