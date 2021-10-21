#[cfg(all(
  not(debug_assertions),
  not(all(target_os = "windows", target_arch = "aarch64"))
))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;
