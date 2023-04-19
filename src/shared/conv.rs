pub trait Signed<T>: Sized {
    type Unsigned: Unsigned<T>;

    fn to_unsigned(self) -> Self::Unsigned;
}

pub trait Unsigned<T>: Sized {
    type Signed: Signed<T>;

    fn to_signed(self) -> Self::Signed;
}
