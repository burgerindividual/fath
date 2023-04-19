pub mod conv;
pub mod float;
pub mod int;

#[macro_export]
macro_rules! wrap_auto_vectorize {
    ($func:expr, $lanes:expr, $($x:expr),+) => {
        {
            let mut vec_uninit: core::mem::MaybeUninit<Simd<_, $lanes>> = core::mem::MaybeUninit::uninit();
            let vec_ptr = vec_uninit.as_mut_ptr();

            for i in 0..$lanes {
                let evaluated = $func($($x[i]),+);
                #[allow(unused_unsafe)]
                unsafe {
                    (*vec_ptr)[i] = evaluated;
                }
            }

            #[allow(unused_unsafe)]
            unsafe { vec_uninit.assume_init() }
        }
    }
}
