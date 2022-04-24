use std::io::{Read, Result as IoResult};

use byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt};

/// Read a value from a [`Read`].
pub trait UnpackFrom: Sized {
    /// Unpack a single value from `src`.
    /// # Example
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::BigEndian;
    /// use byteorder_pack::UnpackFrom;
    ///
    /// let mut cursor = Cursor::new(vec![0x01, 0x02, 0x00, 0x03, 0x00, 0x04]);
    ///
    /// let (a, b, cd) = <(u8, u8, [u16; 2])>::unpack_from::<BigEndian, _>(&mut cursor).unwrap();
    ///
    /// assert_eq!(a, 1);
    /// assert_eq!(b, 2);
    /// assert_eq!(cd, [3, 4]);
    /// ```
    fn unpack_from<E: ByteOrder, R: Read + ?Sized>(src: &mut R) -> IoResult<Self>;

    /// Unpack binary data contained in `src` to a tuple, in [`BigEndian`] order.
    /// # Example
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder_pack::{UnpackFrom};
    ///
    /// let mut cursor = Cursor::new(vec![0x01, 0x02, 0x00, 0x03, 0x00, 0x04]);
    ///
    /// let (a, b, cd) = <(u8, u8, [u16; 2])>::unpack_from_be(&mut cursor).unwrap();
    ///
    /// assert_eq!(a, 1);
    /// assert_eq!(b, 2);
    /// assert_eq!(cd, [3, 4]);
    /// ```
    fn unpack_from_be<R: Read + ?Sized>(src: &mut R) -> IoResult<Self> {
        Self::unpack_from::<BigEndian, _>(src)
    }

    /// Unpack binary data contained in `src` to a tuple, in [`LittleEndian`] order.
    /// # Example
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder_pack::UnpackFrom;
    ///
    /// let mut cursor = Cursor::new(vec![0x01, 0x02, 0x03, 0x00, 0x04, 0x00]);
    ///
    /// let (a, b, cd) = <(u8, u8, [u16; 2])>::unpack_from_le(&mut cursor).unwrap();
    ///
    /// assert_eq!(a, 1);
    /// assert_eq!(b, 2);
    /// assert_eq!(cd, [3, 4]);
    /// ```
    fn unpack_from_le<R: Read + ?Sized>(src: &mut R) -> IoResult<Self> {
        Self::unpack_from::<LittleEndian, _>(src)
    }

    /// Unpack multiple values from `src`.
    fn unpack_multiple_into<E: ByteOrder, R: Read + ?Sized>(
        src: &mut R,
        dst: &mut [Self],
    ) -> IoResult<()> {
        for i in dst {
            *i = Self::unpack_from::<E, _>(src)?;
        }
        Ok(())
    }
}

impl<T: UnpackFrom + Default + Copy, const N: usize> UnpackFrom for [T; N] {
    fn unpack_from<E: ByteOrder, R: Read + ?Sized>(src: &mut R) -> IoResult<Self> {
        let mut dst = [T::default(); N];
        T::unpack_multiple_into::<E, _>(src, &mut dst)?;
        Ok(dst)
    }
}

macro_rules! impl_tuple {
    ($($n:ident),+) => {
        impl<$($n: UnpackFrom),+> UnpackFrom for ($($n,)+)
        {
            #[inline]
            fn unpack_from<E: ByteOrder, R: Read + ?Sized>(src: &mut R) -> IoResult<Self> {
                Ok(
                    (
                        $($n::unpack_from::<E, _>(src)?,)+
                    )
                )
            }
        }
    };
}
impl_tuple!(T1);
impl_tuple!(T1, T2);
impl_tuple!(T1, T2, T3);
impl_tuple!(T1, T2, T3, T4);
impl_tuple!(T1, T2, T3, T4, T5);
impl_tuple!(T1, T2, T3, T4, T5, T6);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

macro_rules! impl_primitive {
    ($($name:ident + $name2:ident => $ty:ty),+) => {
        $(
            impl UnpackFrom for $ty {
                fn unpack_from<E: ByteOrder, R: Read + ?Sized>(src: &mut R) -> IoResult<Self> {
                    src.$name::<E>()
                }

                fn unpack_multiple_into<E: ByteOrder, R: Read + ?Sized>(
                    src: &mut R,
                    dst: &mut [Self],
                ) -> IoResult<()> {
                    src.$name2::<E>(dst)
                }
            }
        )+
    };
}

impl_primitive!(
    read_u16 + read_u16_into => u16, read_u32 + read_u32_into => u32, read_u64 + read_u64_into => u64, read_u128 + read_u128_into => u128,
    read_i16 + read_i16_into => i16, read_i32 + read_i32_into => i32, read_i64 + read_i64_into => i64, read_i128 + read_i128_into => i128,
    read_f32 + read_f32_into => f32, read_f64 + read_f64_into => f64
);

impl UnpackFrom for u8 {
    fn unpack_from<E: ByteOrder, R: Read + ?Sized>(src: &mut R) -> IoResult<Self> {
        src.read_u8()
    }

    fn unpack_multiple_into<E: ByteOrder, R: Read + ?Sized>(
        src: &mut R,
        dst: &mut [Self],
    ) -> IoResult<()> {
        src.read_exact(dst)
    }
}

impl UnpackFrom for i8 {
    fn unpack_from<E: ByteOrder, R: Read + ?Sized>(src: &mut R) -> IoResult<Self> {
        src.read_i8()
    }

    fn unpack_multiple_into<E: ByteOrder, R: Read + ?Sized>(
        src: &mut R,
        dst: &mut [Self],
    ) -> IoResult<()> {
        src.read_i8_into(dst)
    }
}
