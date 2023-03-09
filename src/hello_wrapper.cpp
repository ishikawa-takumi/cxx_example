#include "hello_wrapper.h"

#include <iostream>

#include "hello.h"

void hello_wrapper() {
  std::cout << "Hello Wrapper" << std::endl;
  hello();
}