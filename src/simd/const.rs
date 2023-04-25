#[macro_export]
macro_rules! wrap_auto_vectorize {
    ($func:expr, $lanes:expr, $($x:ident),+) => {
        {
            let mut vec_uninit: core::mem::MaybeUninit<Simd<_, $lanes>> = core::mem::MaybeUninit::uninit();
            let vec_ptr = vec_uninit.as_mut_ptr();

            let mut i = 0;
            while i < $lanes {
                let evaluated = $func($($x[i]),+);
                #[allow(unused_unsafe)]
                unsafe {
                    (*vec_ptr)[i] = evaluated;
                }
                i += 1;
            }

            #[allow(unused_unsafe)]
            unsafe { vec_uninit.assume_init() }
        }
    }
}
