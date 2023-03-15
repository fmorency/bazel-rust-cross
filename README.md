# bazel-rust-cross

```
bazel build --platforms=//:linux-aarch64 --extra_toolchains=@llvm_toolchain_with_sysroot//:cc-toolchain-aarch64-linux //:foo
```

This is not working at the moment.
