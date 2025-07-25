// Some parts copyright Serde developers under the MIT / Apache-2.0 licenses at your option.
// See `serde` version `v1.0.169` for the parts where MIT / Apache-2.0 applies.

mod impls;
#[cfg(feature = "serde")]
pub mod serde;

use crate::de::DeserializeSeed;
use crate::{algebraic_value::ser::ValueSerializer, bsatn, buffer::BufWriter};
use crate::{AlgebraicValue, WithTypespace};
use core::marker::PhantomData;
use core::{convert::Infallible, fmt};
use ethnum::{i256, u256};
pub use spacetimedb_bindings_macro::Serialize;

/// A data format that can deserialize any data structure supported by SATs.
///
/// The `Serializer` trait in SATS performs the same function as `serde::Serializer` in [`serde`].
/// See the documentation of `serde::Serializer` for more information on the data model.
///
/// [`serde`]: https://crates.io/crates/serde
pub trait Serializer: Sized {
    /// The output type produced by this `Serializer` during successful serialization.
    ///
    /// Most serializers that produce text or binary output should set `Ok = ()`
    /// and serialize into an [`io::Write`] or buffer contained within the `Serializer` instance.
    /// Serializers that build in-memory data structures may be simplified by using `Ok` to propagate
    /// the data structure around.
    ///
    /// [`io::Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
    type Ok;

    /// The error type when some error occurs during serialization.
    type Error: Error;

    /// Type returned from [`serialize_array`](Serializer::serialize_array)
    /// for serializing the contents of the array.
    type SerializeArray: SerializeArray<Ok = Self::Ok, Error = Self::Error>;

    /// Type returned from [`serialize_seq_product`](Serializer::serialize_seq_product)
    /// for serializing the contents of the *unnamed* product.
    type SerializeSeqProduct: SerializeSeqProduct<Ok = Self::Ok, Error = Self::Error>;

    /// Type returned from [`serialize_named_product`](Serializer::serialize_named_product)
    /// for serializing the contents of the *named* product.
    type SerializeNamedProduct: SerializeNamedProduct<Ok = Self::Ok, Error = Self::Error>;

    /// Serialize a `bool` value.
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u8` value.
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u16` value.
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u32` value.
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u64` value.
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u128` value.
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `u256` value.
    fn serialize_u256(self, v: u256) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i8` value.
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i16` value.
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i32` value.
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i64` value.
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i128` value.
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `i256` value.
    fn serialize_i256(self, v: i256) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `f32` value.
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;

    /// Serialize an `f64` value.
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `&str` string slice.
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;

    /// Serialize a `&[u8]` byte slice.
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;

    /// Begin to serialize a variably sized array.
    /// This call must be followed by zero or more calls to [`SerializeArray::serialize_element`],
    /// then a call to [`SerializeArray::end`].
    ///
    /// The argument is the number of elements in the sequence.
    fn serialize_array(self, len: usize) -> Result<Self::SerializeArray, Self::Error>;

    /// Begin to serialize a product with unnamed fields.
    /// This call must be followed by zero or more calls to [`SerializeSeqProduct::serialize_element`],
    /// then a call to [`SerializeSeqProduct::end`].
    ///
    /// The argument is the number of fields in the product.
    fn serialize_seq_product(self, len: usize) -> Result<Self::SerializeSeqProduct, Self::Error>;

    /// Begin to serialize a product with named fields.
    /// This call must be followed by zero or more calls to [`SerializeNamedProduct::serialize_element`],
    /// then a call to [`SerializeNamedProduct::end`].
    ///
    /// The argument is the number of fields in the product.
    fn serialize_named_product(self, len: usize) -> Result<Self::SerializeNamedProduct, Self::Error>;

    /// Serialize a sum value provided the chosen `tag`, `name`, and `value`.
    fn serialize_variant<T: Serialize + ?Sized>(
        self,
        tag: u8,
        name: Option<&str>,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>;

    /// Serialize the given `bsatn` encoded data of type `ty`.
    ///
    /// This is a concession to performance,
    /// allowing some implementations to write the buffer directly.
    ///
    /// # Safety
    ///
    /// - `decode(ty, &mut bsatn).is_ok()`.
    ///   That is, `bsatn` encodes a valid element of `ty`.
    ///   It's up to the caller to arrange `Ty` such that this holds.
    unsafe fn serialize_bsatn<Ty>(self, ty: &Ty, bsatn: &[u8]) -> Result<Self::Ok, Self::Error>
    where
        for<'a, 'de> WithTypespace<'a, Ty>: DeserializeSeed<'de, Output: Into<AlgebraicValue>>,
    {
        // TODO(Centril): Consider instead deserializing the `bsatn` through a
        // deserializer that serializes into `self` directly.

        // First convert the BSATN to an `AlgebraicValue`.
        // SAFETY: Forward caller requirements of this method to that we are calling.
        let res = unsafe { ValueSerializer.serialize_bsatn(ty, bsatn) };
        let value = res.unwrap_or_else(|x| match x {});

        // Then serialize that.
        value.serialize(self)
    }

    /// Serialize the given `bsatn` encoded data of type `ty`.
    ///
    /// The data is provided as an iterator of chunks, at arbitrary boundaries,
    /// with a total concatenated length of `total_bsatn_len` which callers can assume.
    ///
    /// An implementation of this method is semantically the same as:
    /// ```rust,ignore
    /// let mut buf = Vec::new();
    /// for chunk in bsatn {
    ///     buf.extend(chunk);
    /// }
    /// ser.serialize_bsatn(&buf);
    /// ```
    ///
    /// This method is a concession to performance,
    /// allowing some implementations to write the buffer directly.
    ///
    /// The parameter `I` is required to be `Clone` only for `debug_assert!` purposes.
    ///
    /// # Safety
    ///
    /// - `total_bsatn_len == bsatn.map(|c| c.len()).sum() <= isize::MAX`
    /// - Let `buf` be defined as above, i.e., the bytes of `bsatn` concatenated.
    ///   Then `decode(ty, &mut buf).is_ok()`.
    ///   That is, `buf` encodes a valid element of `ty`.
    ///   It's up to the caller to arrange `Ty` such that this holds.
    unsafe fn serialize_bsatn_in_chunks<'a, Ty, I: Clone + Iterator<Item = &'a [u8]>>(
        self,
        ty: &Ty,
        total_bsatn_len: usize,
        bsatn: I,
    ) -> Result<Self::Ok, Self::Error>
    where
        for<'b, 'de> WithTypespace<'b, Ty>: DeserializeSeed<'de, Output: Into<AlgebraicValue>>,
    {
        // TODO(Centril): Unlike above, in this case we must at minimum concatenate `bsatn`
        // before we can do the piping mentioned above, but that's better than
        // serializing to `AlgebraicValue` first, so consider that.

        // First convert the BSATN to an `AlgebraicValue`.
        // SAFETY: Forward caller requirements of this method to that we are calling.
        let res = unsafe { ValueSerializer.serialize_bsatn_in_chunks(ty, total_bsatn_len, bsatn) };
        let value = res.unwrap_or_else(|x| match x {});

        // Then serialize that.
        value.serialize(self)
    }

    /// Serialize the given `string`.
    ///
    /// The string is provided as an iterator of chunks, at arbitrary boundaries,
    /// with a total concatenated length of `total_len` which callers can trust.
    ///
    /// An implementation of this method is semantically the same as:
    /// ```rust,ignore
    /// let mut buf = Vec::new();
    /// for chunk in string {
    ///     buf.extend(chunk);
    /// }
    /// let str = unsafe { core::str::from_utf8_unchecked(&buf) };
    /// ser.serialize_str(str);
    /// ```
    ///
    /// This method is a concession to performance,
    /// allowing some implementations to write the buffer directly.
    ///
    /// The parameter `I` is required to be `Clone` only for `debug_assert!` purposes.
    ///
    /// # Safety
    ///
    /// - `total_len == string.map(|c| c.len()).sum() <= isize::MAX`
    /// - Let `buf` be the bytes of `string` concatenated.
    ///   Then `core::str::from_utf8(&buf).is_ok()`.
    ///   That is, `buf` is valid UTF-8.
    ///   Note however that individual chunks need not be valid UTF-8,
    ///   as multi-byte characters may be split across chunk boundaries.
    unsafe fn serialize_str_in_chunks<'a, I: Clone + Iterator<Item = &'a [u8]>>(
        self,
        total_len: usize,
        string: I,
    ) -> Result<Self::Ok, Self::Error> {
        // First convert the `string` to an `AlgebraicValue`.
        // SAFETY: Forward caller requirements of this method to that we are calling.
        let res = unsafe { ValueSerializer.serialize_str_in_chunks(total_len, string) };
        let value = res.unwrap_or_else(|x| match x {});

        // Then serialize that.
        // This incurs a very minor cost of branching on `AlgebraicValue::String`.
        value.serialize(self)
    }
}

/// A **data structure** that can be serialized into any data format supported by
/// the SpacetimeDB Algebraic Type System.
///
/// In most cases, implementations of `Serialize` may be `#[derive(Serialize)]`d.
///
/// The `Serialize` trait in SATS performs the same function as `serde::Serialize` in [`serde`].
/// See the documentation of `serde::Serialize` for more information of the data model.
///
/// Do not manually implement this trait unless you know what you are doing.
/// Implementations must be consistent with `Deerialize<'de> for T`, `SpacetimeType for T` and `Serialize, Deserialize for AlgebraicValue`.
/// Implementations that are inconsistent across these traits may result in data loss.
///
/// [`serde`]: https://crates.io/crates/serde
pub trait Serialize {
    /// Serialize `self` in the data format of `S` using the provided `serializer`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;

    #[doc(hidden)]
    /// Serialize `self` in the data format BSATN using the provided BSATN `serializer`.
    fn serialize_into_bsatn<W: BufWriter>(
        &self,
        serializer: bsatn::Serializer<'_, W>,
    ) -> Result<(), bsatn::EncodeError> {
        self.serialize(serializer)
    }

    /// Used in the `Serialize for Vec<T>` implementation
    /// to allow a specialized serialization of `Vec<T>` as bytes.
    #[doc(hidden)]
    #[inline(always)]
    fn __serialize_array<S: Serializer>(this: &[Self], serializer: S) -> Result<S::Ok, S::Error>
    where
        Self: Sized,
    {
        let mut vec = serializer.serialize_array(this.len())?;
        for elem in this {
            vec.serialize_element(elem)?;
        }
        vec.end()
    }
}

/// The base trait serialization error types must implement.
pub trait Error {
    /// Returns an error derived from `msg: impl Display`.
    fn custom<T: fmt::Display>(msg: T) -> Self;
}

impl Error for String {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        msg.to_string()
    }
}

impl Error for std::convert::Infallible {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        panic!("error generated for Infallible serializer: {msg}")
    }
}

/// Returned from [`Serializer::serialize_array`].
///
/// This provides a continuation of sorts
/// where you can call [`serialize_element`](SerializeArray::serialize_element) however many times
/// and then finally the [`end`](SerializeArray::end) is reached.
pub trait SerializeArray {
    /// Must match the `Ok` type of any `Serializer` that uses this type.
    type Ok;

    /// Must match the `Error` type of any `Serializer` that uses this type.
    type Error: Error;

    /// Serialize an array `element`.
    fn serialize_element<T: Serialize + ?Sized>(&mut self, element: &T) -> Result<(), Self::Error>;

    /// Consumes and finalizes the array serializer returning the `Self::Ok` data.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Returned from [`Serializer::serialize_seq_product`].
///
/// This provides a continuation of sorts
/// where you can call [`serialize_element`](SerializeSeqProduct::serialize_element) however many times
/// and then finally the [`end`](SerializeSeqProduct::end) is reached.
pub trait SerializeSeqProduct {
    /// Must match the `Ok` type of any `Serializer` that uses this type.
    type Ok;

    /// Must match the `Error` type of any `Serializer` that uses this type.
    type Error: Error;

    /// Serialize an unnamed product `element`.
    fn serialize_element<T: Serialize + ?Sized>(&mut self, element: &T) -> Result<(), Self::Error>;

    /// Consumes and finalizes the product serializer returning the `Self::Ok` data.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Returned from [`Serializer::serialize_named_product`].
///
/// This provides a continuation of sorts
/// where you can call [`serialize_element`](SerializeNamedProduct::serialize_element) however many times
/// and then finally the [`end`](SerializeNamedProduct::end) is reached.
pub trait SerializeNamedProduct {
    /// Must match the `Ok` type of any `Serializer` that uses this type.
    type Ok;

    /// Must match the `Error` type of any `Serializer` that uses this type.
    type Error: Error;

    /// Serialize a named product `element` with `name`.
    fn serialize_element<T: Serialize + ?Sized>(&mut self, name: Option<&str>, elem: &T) -> Result<(), Self::Error>;

    /// Consumes and finalizes the product serializer returning the `Self::Ok` data.
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Forwards the implementation of a named product value
/// to the implementation of the unnamed kind,
/// thereby ignoring any field names.
pub struct ForwardNamedToSeqProduct<S> {
    /// The unnamed product serializer.
    tup: S,
}

impl<S> ForwardNamedToSeqProduct<S> {
    /// Returns a forwarder based on the provided unnamed product serializer.
    pub fn new(tup: S) -> Self {
        Self { tup }
    }

    /// Forwards the serialization of a named product of `len` fields
    /// to an unnamed serialization format.
    pub fn forward<Ser>(ser: Ser, len: usize) -> Result<Self, Ser::Error>
    where
        Ser: Serializer<SerializeSeqProduct = S>,
    {
        ser.serialize_seq_product(len).map(Self::new)
    }
}

impl<S: SerializeSeqProduct> SerializeNamedProduct for ForwardNamedToSeqProduct<S> {
    type Ok = S::Ok;
    type Error = S::Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, _name: Option<&str>, elem: &T) -> Result<(), Self::Error> {
        self.tup.serialize_element(elem)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.tup.end()
    }
}

/// A type usable in one of the associated types of [`Serializer`]
/// when the data format does not support the data.
pub struct Impossible<Ok, Error> {
    // They gave each other a pledge. Unheard of, absurd.
    absurd: Infallible,
    marker: PhantomData<(Ok, Error)>,
}

impl<Ok, Error: self::Error> SerializeArray for Impossible<Ok, Error> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, _: &T) -> Result<(), Self::Error> {
        match self.absurd {}
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.absurd {}
    }
}

impl<Ok, Error: self::Error> SerializeSeqProduct for Impossible<Ok, Error> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, _: &T) -> Result<(), Self::Error> {
        match self.absurd {}
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.absurd {}
    }
}

impl<Ok, Error: self::Error> SerializeNamedProduct for Impossible<Ok, Error> {
    type Ok = Ok;
    type Error = Error;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, _: Option<&str>, _: &T) -> Result<(), Self::Error> {
        match self.absurd {}
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.absurd {}
    }
}
