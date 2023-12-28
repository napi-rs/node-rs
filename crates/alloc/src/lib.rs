#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;
