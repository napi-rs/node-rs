#[cfg(all(not(target_env = "musl"), not(target_arch = "aarch64")))]
#[global_allocator]
static GLOBAL: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;
