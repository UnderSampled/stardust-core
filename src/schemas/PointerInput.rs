// automatically generated by the FlatBuffers compiler, do not modify



use crate::common::*;
use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod stardust_xr {

  use crate::common::*;
  use std::mem;
  use std::cmp::Ordering;

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
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> Pointer<'a> {
  pub const VT_ORIGIN: flatbuffers::VOffsetT = 4;
  pub const VT_DIRECTION: flatbuffers::VOffsetT = 6;
  pub const VT_TILT: flatbuffers::VOffsetT = 8;
  pub const VT_DEEPEST_POINT: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Pointer { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PointerArgs<'args>
  ) -> flatbuffers::WIPOffset<Pointer<'bldr>> {
    let mut builder = PointerBuilder::new(_fbb);
    if let Some(x) = args.deepest_point { builder.add_deepest_point(x); }
    builder.add_tilt(args.tilt);
    if let Some(x) = args.direction { builder.add_direction(x); }
    if let Some(x) = args.origin { builder.add_origin(x); }
    builder.finish()
  }


  #[inline]
  pub fn origin(&self) -> &'a vec3 {
    self._tab.get::<vec3>(Pointer::VT_ORIGIN, None).unwrap()
  }
  #[inline]
  pub fn direction(&self) -> &'a vec3 {
    self._tab.get::<vec3>(Pointer::VT_DIRECTION, None).unwrap()
  }
  #[inline]
  pub fn tilt(&self) -> f32 {
    self._tab.get::<f32>(Pointer::VT_TILT, Some(0.0)).unwrap()
  }
  #[inline]
  pub fn deepest_point(&self) -> &'a vec3 {
    self._tab.get::<vec3>(Pointer::VT_DEEPEST_POINT, None).unwrap()
  }
}

impl flatbuffers::Verifiable for Pointer<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<vec3>("origin", Self::VT_ORIGIN, true)?
     .visit_field::<vec3>("direction", Self::VT_DIRECTION, true)?
     .visit_field::<f32>("tilt", Self::VT_TILT, false)?
     .visit_field::<vec3>("deepest_point", Self::VT_DEEPEST_POINT, true)?
     .finish();
    Ok(())
  }
}
pub struct PointerArgs<'a> {
    pub origin: Option<&'a vec3>,
    pub direction: Option<&'a vec3>,
    pub tilt: f32,
    pub deepest_point: Option<&'a vec3>,
}
impl<'a> Default for PointerArgs<'a> {
  #[inline]
  fn default() -> Self {
    PointerArgs {
      origin: None, // required field
      direction: None, // required field
      tilt: 0.0,
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
  pub fn add_origin(&mut self, origin: &vec3) {
    self.fbb_.push_slot_always::<&vec3>(Pointer::VT_ORIGIN, origin);
  }
  #[inline]
  pub fn add_direction(&mut self, direction: &vec3) {
    self.fbb_.push_slot_always::<&vec3>(Pointer::VT_DIRECTION, direction);
  }
  #[inline]
  pub fn add_tilt(&mut self, tilt: f32) {
    self.fbb_.push_slot::<f32>(Pointer::VT_TILT, tilt, 0.0);
  }
  #[inline]
  pub fn add_deepest_point(&mut self, deepest_point: &vec3) {
    self.fbb_.push_slot_always::<&vec3>(Pointer::VT_DEEPEST_POINT, deepest_point);
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
    self.fbb_.required(o, Pointer::VT_DIRECTION,"direction");
    self.fbb_.required(o, Pointer::VT_DEEPEST_POINT,"deepest_point");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for Pointer<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("Pointer");
      ds.field("origin", &self.origin());
      ds.field("direction", &self.direction());
      ds.field("tilt", &self.tilt());
      ds.field("deepest_point", &self.deepest_point());
      ds.finish()
  }
}
}  // pub mod StardustXR

