// build.rs

fn main() {
  cc::Build::new()
      .file("src/bin/fib.cpp")
      .compile("fib");
  println!("cargo:rerun-if-changed=src/bin/fib.cpp");
}
