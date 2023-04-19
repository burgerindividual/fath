pub trait Signed<Output = Self>: Sized {
    type Unsigned: Unsigned<Output>;

    fn to_unsigned(self) -> Self::Unsigned;
}

pub trait Unsigned<Output = Self>: Sized {
    type Signed: Signed<Output>;

    fn to_signed(self) -> Self::Signed;
}
