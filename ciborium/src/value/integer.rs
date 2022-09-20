// SPDX-License-Identifier: Apache-2.0
use core::{cmp::Ordering, hash::Hash, fmt::Debug};

macro_rules! implfrom {
    ($( $(#[$($attr:meta)+])? $t:ident)+) => {
        $(
            $(#[$($attr)+])?
            impl From<$t> for Integer {
                #[inline]
                fn from(value: $t) -> Self {
                    Self{
                        number:N::from(value)
                    }
                }
            }

            impl TryFrom<Integer> for $t {
                type Error = ConvertError;

                #[inline]
                fn try_from(value: Integer) -> Result<Self, Self::Error> {
                    $t::try_from(value.number)
                }
            }
        )+
    };
}

/// An abstract integer value
///
/// This opaque type represents an integer value which can be encoded in CBOR
/// without resulting to big integer encoding. Larger values may be encoded
/// using the big integer encoding as described in the CBOR RFC. See the
/// implementations for 128-bit integer conversions on `Value` for more
/// details.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer {
    number: N,
}



#[cfg(not(feature = "arbitrary_precision"))]
#[derive(Copy, Clone, Debug)]
enum N {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),

    USIZE(usize),
    ISIZE(isize),
}

impl PartialEq for N {
    fn eq(&self, other: &Self) -> bool {
        let v1 = u128::try_from(self.clone());
        if let Ok(x) = v1{
            let v2 = u128::try_from(other.clone());

            if let Ok(y) = v2  {
               return  x == y;
            }

        }

        let x = i128::try_from(self.clone()).unwrap();
        
        let y = i128::try_from(other.clone()).unwrap();

        return  x == y;
        }
    
}
impl Eq for N {}

impl Ord for N {
    fn cmp(&self, other: &Self) -> Ordering {
        let v1 = u128::try_from(self.clone());
        if let Ok(x) = v1{
            let v2 = u128::try_from(other.clone());

            if let Ok(y) = v2  {
               return  x.cmp(&y);
            }

        }

        let x = i128::try_from(self.clone()).unwrap();
        
        let v2 = i128::try_from(other.clone()).unwrap();

        return  x.cmp(&v2);
        
    }
}

impl PartialOrd for N {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
       let v1 = u128::try_from(self.clone());
        if let Ok(x) = v1{
            let v2 = u128::try_from(other.clone());

            if let Ok(y) = v2  {
               return  x.partial_cmp(&y);
            }

        }

        let v1 = i128::try_from(self.clone());
        
         if let Ok(x) = v1{
            let v2 = i128::try_from(other.clone());

            if let Ok(y) = v2  {
               return  x.partial_cmp(&y);
            }

        }

        None
    }
}

impl Hash for N {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            N::U8(x) => x.hash(state),
            N::U16(x) => x.hash(state),
            N::U32(x) => x.hash(state),
            N::U64(x) => x.hash(state),
            N::I8(x) => x.hash(state),
            N::I16(x) => x.hash(state),
            N::I32(x) => x.hash(state),
            N::I64(x) => x.hash(state),
            N::I128(x) => x.hash(state),
            N::USIZE(x) => x.hash(state),
            N::ISIZE(x) => x.hash(state),
        }
    }
}

#[derive(Debug)]
pub struct ConvertError {
    message: String,
}


impl From<core::num::TryFromIntError> for ConvertError {
    #[inline]
    fn from(value: core::num::TryFromIntError) -> Self {
        ConvertError {
            message: format!("Unable to convert {:?}", value),
        }
    }
}

impl From<std::convert::Infallible> for ConvertError {
    #[inline]
    fn from(value: std::convert::Infallible) -> Self {
        ConvertError {
            message: format!("Unable to convert {:?}", value),
        }
    }
}

macro_rules! number_from {
    ($($v:ident($t:ty)),+ $(,)?) => {
        $(
            impl From<$t> for N {
                #[inline]
                fn from(value: $t) -> Self {
                    Self::$v(value)
                }
            }


            impl TryFrom<N> for $t {
                type Error = ConvertError;

                #[inline]
                fn try_from(value: N) -> Result<$t, Self::Error> {

                  match value {
                        N::U8(x) =>    {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::U16(x) =>    {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::U32(x) =>    {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::U64(x) =>   {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::I8(x) =>    {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::I16(x) =>    {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::I32(x) =>   {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::I64(x) =>   {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::I128(x) =>   {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::USIZE(x) =>  {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                        N::ISIZE(x) =>  {
                            match <$t>::try_from(x){
                                Ok(x) => Ok(x),
                                Err(e) => Err(ConvertError::from(e))
                            }
                        },
                    }


                }
            }

        )+
    };
}

number_from! {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    USIZE(usize),
    ISIZE(isize),
}

impl Integer {
    /// Returns the canonical length this integer will have when serialized to bytes.
    /// This is called `canonical` as it is only used for canonically comparing two
    /// values. It shouldn't be used in any other context.
    fn canonical_len(&self) -> usize {
        let x = self.number;

        if let Ok(x) = u8::try_from(x) {
            if x < 24 {
                1
            } else {
                2
            }
        } else if let Ok(x) = i8::try_from(x) {
            if x >= -24i8 {
                1
            } else {
                2
            }
        } else if u16::try_from(x).is_ok() || i16::try_from(x).is_ok() {
            3
        } else if u32::try_from(x).is_ok() || i32::try_from(x).is_ok() {
            5
        } else if u64::try_from(x).is_ok() || i64::try_from(x).is_ok() {
            9
        } else  {
            // Ciborium serializes u128/i128 as BigPos if they don't fit in 64 bits.
            // In this special case we have to calculate the length.
            // The Tag itself will always be 1 byte.
            let y =  i128::try_from(x).unwrap();
            y.to_be_bytes().len() + 1
        }
    }

    fn is_negative(&self) -> bool {
        match self.number {
            N::U8(_) | N::U16(_) | N::U32(_) | N::U64(_) | N::USIZE(_) => false,

            N::I8(x) => x.is_negative(),
            N::I16(x) => x.is_negative(),
            N::I32(x) => x.is_negative(),
            N::I64(x) => x.is_negative(),
            N::I128(x) => x.is_negative(),
            N::ISIZE(x) => x.is_negative(),
        }
    }

    fn as_i128(&self) -> i128 {
        match self.number {
           

            N::I8(x) => x as i128,
            N::I16(x) => x as i128,
            N::I32(x) => x as i128,
            N::I64(x) => x as i128,
            N::I128(x) => x as i128,
            N::ISIZE(x) => x as i128,

            N::U8(x) => x as i128,
            N::U16(x) => x as i128,
            N::U32(x) => x as i128,
            N::U64(x) => x as i128,
            N::USIZE(x) => x as i128,
        }
    }

    fn as_u64(&self) -> u64 {
        match self.number {
            
            N::U8(x) => x as u64,
            N::U16(x) => x as u64,
            N::U32(x) => x as u64,
            N::U64(x) => x as u64,
            N::USIZE(x) => x as u64,
            N::I8(x) => x as u64,
            N::I16(x) => x as u64,
            N::I32(x) => x as u64,
            N::I64(x) => x as u64,
            N::I128(x) => x as u64,
            N::ISIZE(x) => x as u64,

            
        }
    }

    /// Compare two integers as if we were to serialize them, but more efficiently.
    pub fn canonical_cmp(&self, other: &Self) -> Ordering {
        match self.canonical_len().cmp(&other.canonical_len()) {
            Ordering::Equal => {
                // Negative numbers are higher in byte-order than positive numbers.
                match (self.is_negative(), other.is_negative()) {
                    (false, true) => Ordering::Less,
                    (true, false) => Ordering::Greater,
                    (true, true) => {
                        // For negative numbers the byte order puts numbers closer to 0 which
                        // are lexically higher, lower. So -1 < -2 when sorting by be_bytes().
                        let v1 = self.as_i128();
                        let v2 = other.as_i128();
                        match v1.cmp(&v2) {
                            Ordering::Less => Ordering::Greater,
                            Ordering::Equal => Ordering::Equal,
                            Ordering::Greater => Ordering::Less,
                        }
                    }
                    (false, false) => {
                        
                        let v1 = self.as_u64();
                        let v2 = other.as_u64();
                        v1.cmp(&v2)
                    } //self.0.cmp(&other.0),
                }
            }
            x => x,
        }
    }
}

implfrom! {
    u8 u16 u32 u64
    i8 i16 i32 i64

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    usize

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    isize
}

impl TryFrom<i128> for Integer {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        u64::try_from(match value.is_negative() {
            false => value,
            true => value ^ !0,
        })?;

        Ok(Integer {
            number: N::I128(value),
        })
    }
}

impl TryFrom<u128> for Integer {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Ok(Self {
            number: u64::try_from(value)?.into(),
        })
    }
}

impl From<Integer> for i128 {
    #[inline]
    fn from(value: Integer) -> Self {
        value.as_i128()
    }
}

impl TryFrom<Integer> for u128 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        value.number.try_into()
    }
}

impl TryFrom<N> for u128 {
    type Error = core::num::TryFromIntError;

    #[inline]
    fn try_from(value: N) -> Result<Self, Self::Error> {
        
        match  value {
            N::U8(x) => Ok(x as u128),
            N::U16(x) => Ok(x as u128),
            N::U32(x) => Ok(x as u128),
            N::U64(x) => Ok(x as u128),
            N::I8(x) => u128::try_from(x),
            N::I16(x) => u128::try_from(x),
            N::I32(x) => u128::try_from(x),
            N::I64(x) => u128::try_from(x),
            N::I128(x) => u128::try_from(x),
            N::USIZE(x) => u128::try_from(x),
            N::ISIZE(x) => u128::try_from(x),
        }
    }
}

