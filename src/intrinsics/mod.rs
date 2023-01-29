pub mod scalar;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse2;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse41;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx2;
