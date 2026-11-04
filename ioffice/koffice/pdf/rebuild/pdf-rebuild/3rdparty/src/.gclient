solutions = [
  { "name"        : 'pdfium',
    "url"         : 'https://pdfium.googlesource.com/pdfium',
    "deps_file"   : 'DEPS',
    "managed"     : False,
    "custom_deps" : {
      "pdfium/testing/corpus": None,
      "pdfium/third_party/llvm-build/Release+Asserts": None,
      "pdfium/third_party/cpython3/host": None,
      "pdfium/buildtools/mac": None,
      "pdfium/buildtools/reclient": None,
    },
    "custom_vars": {},
  },
]
