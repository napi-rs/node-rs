#[cfg(all(not(target_env = "musl"), target_arch = "x86_64"))]
#[global_allocator]
static GLOBAL: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;
