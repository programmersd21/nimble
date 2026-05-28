#[cfg(feature = "jit")]
pub mod repl_jit;

#[cfg(not(feature = "jit"))]
pub mod repl_simple;
