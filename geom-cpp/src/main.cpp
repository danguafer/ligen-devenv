// #include <iostream>
// #include <ContextBuilder.h>
//
// int main(int argc, char **argv) {
//   Context context = ContextBuilder_build(ContextBuilder_with_display(ContextBuilder_cursor(ContextBuilder_new(), false), ContextDisplay_window((int8_t*) "Caraio", 800, 600)));
//   while (Context_run(context));
//   return 0;
// }

#include <iostream>
#include <ContextBuilder.hpp>

int main(int argc, char **argv) {
  Context context = ContextBuilder().cursor(false).with_display(ContextDisplay::window((int8_t*) "Window name", 800, 600)).build();
  while (context.run());
  return 0;
}
