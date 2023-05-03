macro_rules! use_available_arch {
    () => {
        #[allow(unused_imports)]
        #[cfg(target_arch = "aarch64")]
        use core::arch::aarch64::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "arm")]
        use core::arch::arm::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "mips")]
        use core::arch::mips::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "mips64")]
        use core::arch::mips64::*;
        #[allow(unused_imports)]
        #[cfg(any(target_arch = "nvptx", target_arch = "nvptx64"))]
        use core::arch::nvptx::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "powerpc")]
        use core::arch::powerpc::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "powerpc64")]
        use core::arch::powerpc64::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "riscv32")]
        use core::arch::riscv32::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "riscv64")]
        use core::arch::riscv64::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "wasm32")]
        use core::arch::wasm32::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "wasm64")]
        use core::arch::wasm64::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[allow(unused_imports)]
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;
    };
}

pub(crate) use use_available_arch;
