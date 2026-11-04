extern "C" int call_from_c();

int c_lib_get_value() {
  return call_from_c();
}
