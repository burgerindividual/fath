pub trait Signed<Output>: Sized {
    type Unsigned: Output;

    fn to_unsigned(self) -> Output;
}

pub trait Unsigned<Output>: Sized {
    type Signed: Output;

    fn to_signed(self) -> Output;
}
