#include "hello.h"

#include <iostream>

const std::string& hello(const std::string& name) {
  static std::string message = "Hello " + name + "!";
  return message;
}