// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use crate::common::*;
use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod stardust_xr {

  use crate::common::*;
  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum PointerOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Pointer<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Pointer<'a> {
  type Inner = Pointer<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Pointer<'a> {
  pub const VT_ORIGIN: flatbuffers::VOffsetT = 4;
  pub const VT_ORIENTATION: flatbuffers::VOffsetT = 6;
  pub const VT_DEEPEST_POINT: flatbuffers::VOffsetT = 8;

  pub const fn get_fully_qualified_name() -> &'static str {
    "StardustXR.Pointer"
  }

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Pointer { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PointerArgs<'args>
  ) -> flatbuffers::WIPOffset<Pointer<'bldr>> {
    let mut builder = PointerBuilder::new(_fbb);
    if let Some(x) = args.deepest_point { builder.add_deepest_point(x); }
    if let Some(x) = args.orientation { builder.add_orientation(x); }
    if let Some(x) = args.origin { builder.add_origin(x); }
    builder.finish()
  }

  pub fn unpack(&self) -> PointerT {
    let origin = {
      let x = self.origin();
      x.unpack()
    };
    let orientation = {
      let x = self.orientation();
      x.unpack()
    };
    let deepest_point = {
      let x = self.deepest_point();
      x.unpack()
    };
    PointerT {
      origin,
      orientation,
      deepest_point,
    }
  }

  #[inline]
  pub fn origin(&self) -> &'a Vec3 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Vec3>(Pointer::VT_ORIGIN, None).unwrap()}
  }
  #[inline]
  pub fn orientation(&self) -> &'a Quat {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Quat>(Pointer::VT_ORIENTATION, None).unwrap()}
  }
  #[inline]
  pub fn deepest_point(&self) -> &'a Vec3 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Vec3>(Pointer::VT_DEEPEST_POINT, None).unwrap()}
  }
}

impl flatbuffers::Verifiable for Pointer<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Vec3>("origin", Self::VT_ORIGIN, true)?
     .visit_field::<Quat>("orientation", Self::VT_ORIENTATION, true)?
     .visit_field::<Vec3>("deepest_point", Self::VT_DEEPEST_POINT, true)?
     .finish();
    Ok(())
  }
}
pub struct PointerArgs<'a> {
    pub origin: Option<&'a Vec3>,
    pub orientation: Option<&'a Quat>,
    pub deepest_point: Option<&'a Vec3>,
}
impl<'a> Default for PointerArgs<'a> {
  #[inline]
  fn default() -> Self {
    PointerArgs {
      origin: None, // required field
      orientation: None, // required field
      deepest_point: None, // required field
    }
  }
}

pub struct PointerBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PointerBuilder<'a, 'b> {
  #[inline]
  pub fn add_origin(&mut self, origin: &Vec3) {
    self.fbb_.push_slot_always::<&Vec3>(Pointer::VT_ORIGIN, origin);
  }
  #[inline]
  pub fn add_orientation(&mut self, orientation: &Quat) {
    self.fbb_.push_slot_always::<&Quat>(Pointer::VT_ORIENTATION, orientation);
  }
  #[inline]
  pub fn add_deepest_point(&mut self, deepest_point: &Vec3) {
    self.fbb_.push_slot_always::<&Vec3>(Pointer::VT_DEEPEST_POINT, deepest_point);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PointerBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PointerBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Pointer<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Pointer::VT_ORIGIN,"origin");
    self.fbb_.required(o, Pointer::VT_ORIENTATION,"orientation");
    self.fbb_.required(o, Pointer::VT_DEEPEST_POINT,"deepest_point");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Pointer<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Pointer");
      ds.field("origin", &self.origin());
      ds.field("orientation", &self.orientation());
      ds.field("deepest_point", &self.deepest_point());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct PointerT {
  pub origin: Vec3T,
  pub orientation: QuatT,
  pub deepest_point: Vec3T,
}
impl Default for PointerT {
  fn default() -> Self {
    Self {
      origin: Default::default(),
      orientation: Default::default(),
      deepest_point: Default::default(),
    }
  }
}
impl PointerT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Pointer<'b>> {
    let origin_tmp = Some(self.origin.pack());
    let origin = origin_tmp.as_ref();
    let orientation_tmp = Some(self.orientation.pack());
    let orientation = orientation_tmp.as_ref();
    let deepest_point_tmp = Some(self.deepest_point.pack());
    let deepest_point = deepest_point_tmp.as_ref();
    Pointer::create(_fbb, &PointerArgs{
      origin,
      orientation,
      deepest_point,
    })
  }
}
}  // pub mod StardustXR

