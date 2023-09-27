use std::io::{Result as IoResult, Write};

use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};

/// Write a value into a [`Write`].
pub trait PackTo: Sized {
    /// Pack binary data into `dst`.
    /// # Example
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder::BigEndian;
    /// use byteorder_pack::PackTo;
    ///
    /// let mut cursor = Cursor::new(vec![]);
    ///
    /// (1u8, 2u8, 3u16, 4u16).pack_to::<BigEndian, _>(&mut cursor).unwrap();
    ///
    /// assert_eq!(cursor.into_inner(), vec![0x01, 0x02, 0x00, 0x03, 0x00, 0x04]);
    /// ```
    fn pack_to<E: ByteOrder, W: Write + ?Sized>(&self, dst: &mut W) -> IoResult<()>;

    /// Pack binary data into `dst` from a tuple, in [`BigEndian`] order.
    /// # Example
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder_pack::PackTo;
    ///
    /// let mut cursor = Cursor::new(vec![]);
    ///
    /// (1u8, 2u8, 3u16, 4u16).pack_to_be(&mut cursor).unwrap();
    ///
    /// assert_eq!(cursor.into_inner(), vec![0x01, 0x02, 0x00, 0x03, 0x00, 0x04]);
    /// ```
    fn pack_to_be<W: Write + ?Sized>(&self, dst: &mut W) -> IoResult<()> {
        self.pack_to::<BigEndian, _>(dst)
    }
    /// Pack binary data into `dst` from a tuple, in [`LittleEndian`] order.
    /// # Example
    /// ```rust
    /// use std::io::Cursor;
    /// use byteorder_pack::PackTo;
    ///
    /// let mut cursor = Cursor::new(vec![]);
    ///
    /// (1u8, 2u8, 3u16, 4u16).pack_to_le(&mut cursor).unwrap();
    ///
    /// assert_eq!(cursor.into_inner(), vec![0x01, 0x02, 0x03, 0x00, 0x04, 0x00]);
    /// ```
    fn pack_to_le<W: Write + ?Sized>(&self, dst: &mut W) -> IoResult<()> {
        self.pack_to::<LittleEndian, _>(dst)
    }

    /// Pack multiple values into `dest`.
    fn pack_multiple_to<E: ByteOrder, W: Write + ?Sized>(
        buf: &[Self],
        dst: &mut W,
    ) -> IoResult<()> {
        for i in buf {
            i.pack_to::<E, _>(dst)?;
        }
        Ok(())
    }
}

impl<T: PackTo> PackTo for &'_ T {
    #[inline]
    fn pack_to<E: ByteOrder, W: Write + ?Sized>(&self, dst: &mut W) -> IoResult<()> {
        (*self).pack_to::<E, _>(dst)
    }
}

impl PackTo for () {
    #[inline]
    fn pack_to<E: ByteOrder, W: Write + ?Sized>(&self, _dst: &mut W) -> IoResult<()> {
        Ok(())
    }
}

macro_rules! impl_tuple {
    ($($n:tt => $t:ident),+) => {
        impl<$($t: PackTo),+> PackTo for ($($t,)+)
        {
            #[inline]
            fn pack_to<E: ByteOrder, W: Write + ?Sized>(&self, dst: &mut W) -> IoResult<()> {
                $(self.$n.pack_to::<E, _>(dst)?;)+
                Ok(())
            }
        }
    };
}
impl_tuple!(0 => T1);
impl_tuple!(0 => T1, 1 => T2);
impl_tuple!(0 => T1, 1 => T2, 2 => T3);
impl_tuple!(0 => T1, 1 => T2, 2 => T3, 3 => T4);
impl_tuple!(0 => T1, 1 => T2, 2 => T3, 3 => T4, 4 => T5);
impl_tuple!(0 => T1, 1 => T2, 2 => T3, 3 => T4, 4 => T5, 5 => T6);
impl_tuple!(0 => T1, 1 => T2, 2 => T3, 3 => T4, 4 => T5, 5 => T6, 6 => T7);
impl_tuple!(0 => T1, 1 => T2, 2 => T3, 3 => T4, 4 => T5, 5 => T6, 6 => T7, 7 => T8);
impl_tuple!(
    0 => T1, 1 => T2, 2 => T3, 3 => T4, 4 => T5, 5 => T6,
    6 => T7, 7 => T8, 8 => T9
);
impl_tuple!(
    0 => T1, 1 => T2, 2 => T3, 3 => T4, 4 => T5, 5 => T6,
    6 => T7, 7 => T8, 8 => T9, 9 => T10
);
impl_tuple!(
    0 => T1, 1 => T2, 2 => T3, 3 => T4, 4 => T5, 5 => T6,
    6 => T7, 7 => T8, 8 => T9, 9 => T10, 10 => T11
);
impl_tuple!(
    0 => T1, 1 => T2, 2 => T3, 3 => T4, 4 => T5, 5 => T6,
    6 => T7, 7 => T8, 8 => T9, 9 => T10, 10 => T11, 11 => T12
);

macro_rules! impl_primitive {
    ($($name:ident => $ty:ty),+) => {
        $(
            impl PackTo for $ty {
                fn pack_to<E: ByteOrder, W: Write + ?Sized>(&self, src: &mut W) -> IoResult<()> {
                    src.$name::<E>(*self)
                }
            }
        )+
    };
}

impl_primitive!(
    write_u16 => u16, write_u32 => u32, write_u64 => u64, write_u128 => u128,
    write_i16 => i16, write_i32 => i32, write_i64 => i64, write_i128 => i128,
    write_f32 => f32, write_f64 => f64
);

impl PackTo for u8 {
    fn pack_to<E: ByteOrder, W: Write + ?Sized>(&self, dst: &mut W) -> IoResult<()> {
        dst.write_u8(*self)
    }

    fn pack_multiple_to<E: ByteOrder, W: Write + ?Sized>(
        buf: &[Self],
        dst: &mut W,
    ) -> IoResult<()> {
        dst.write_all(buf)?;
        Ok(())
    }
}

impl PackTo for i8 {
    fn pack_to<E: ByteOrder, W: Write + ?Sized>(&self, dst: &mut W) -> IoResult<()> {
        dst.write_i8(*self)
    }
}

impl<T: PackTo + Copy, const N: usize> PackTo for [T; N] {
    fn pack_to<E: ByteOrder, W: Write + ?Sized>(&self, dst: &mut W) -> IoResult<()> {
        T::pack_multiple_to::<E, _>(&self[..], dst)?;
        Ok(())
    }
}

impl<T: PackTo + Copy> PackTo for &[T] {
    fn pack_to<E: ByteOrder, W: Write + ?Sized>(&self, dst: &mut W) -> IoResult<()> {
        T::pack_multiple_to::<E, _>(self, dst)?;
        Ok(())
    }
}
