use std::io;

use super::wire_type::{self, WireType, WIRE_TYPE_SIZED};

pub trait Serialize: WireType {
    fn compute_size(&self) -> u32;
    fn serialize_with_cached_size(&self, writer: &mut impl io::Write) -> io::Result<()>;

    #[inline]
    fn cached_size(&self) -> u32 {
        self.compute_size()
    }

    #[inline]
    fn serialize(&self, writer: &mut impl io::Write) -> io::Result<()> {
        self.compute_size();
        self.serialize_with_cached_size(writer)
    }

    #[inline]
    fn key(tag: u16) -> u32 {
        wire_type::key(tag, Self::WIRE_TYPE)
    }

    #[inline]
    fn is_default_nested_with_cached_size(&self) -> bool {
        self.cached_size() == 0
    }

    #[inline]
    fn compute_size_nested_omittable(&self, tag: impl Into<Option<u16>>, omittable: bool) -> u32 {
        let tag = tag.into();
        let mut size = self.compute_size();

        if tag.is_some() && omittable && self.is_default_nested_with_cached_size() {
            return 0;
        }

        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            size += size.compute_size();
        }

        if let Some(tag) = tag.into() {
            size += Self::key(tag).compute_size();
        }

        size
    }

    #[inline]
    fn compute_size_nested(&self, tag: impl Into<Option<u16>>) -> u32 {
        self.compute_size_nested_omittable(tag, true)
    }

    #[inline]
    fn serialize_nested_omittable_with_cached_size(
        &self,
        tag: impl Into<Option<u16>>,
        omittable: bool,
        writer: &mut impl io::Write,
    ) -> io::Result<()> {
        let tag = tag.into();

        if tag.is_some() && omittable && self.is_default_nested_with_cached_size() {
            return Ok(());
        }

        if let Some(tag) = tag {
            Self::key(tag).serialize_with_cached_size(writer)?;
        }

        if Self::WIRE_TYPE == WIRE_TYPE_SIZED {
            self.cached_size().serialize_with_cached_size(writer)?;
        }

        self.serialize_with_cached_size(writer)
    }

    #[inline]
    fn serialize_nested_with_cached_size(
        &self,
        tag: impl Into<Option<u16>>,
        writer: &mut impl io::Write,
    ) -> io::Result<()> {
        self.serialize_nested_omittable_with_cached_size(tag, true, writer)
    }
}
