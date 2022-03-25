#[cfg(all(
  all(not(target_env = "musl"), target_arch = "x86_64"),
  debug_assertions
))]
#[global_allocator]
static GLOBAL: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;
