pub trait Signed: Sized {
    type Unsigned: Unsigned<Signed = Self>;

    fn to_unsigned(self) -> Self::Unsigned;
}

pub trait Unsigned: Sized {
    type Signed: Signed<Unsigned = Self>;

    fn to_signed(self) -> Self::Signed;
}
