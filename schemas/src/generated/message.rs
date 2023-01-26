// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod stardust_xr {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum MessageOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Message<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Message<'a> {
  type Inner = Message<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Message<'a> {
  pub const VT_TYPE_: flatbuffers::VOffsetT = 4;
  pub const VT_ID: flatbuffers::VOffsetT = 6;
  pub const VT_OBJECT: flatbuffers::VOffsetT = 8;
  pub const VT_METHOD: flatbuffers::VOffsetT = 10;
  pub const VT_ERROR: flatbuffers::VOffsetT = 12;
  pub const VT_DATA: flatbuffers::VOffsetT = 14;

  pub const fn get_fully_qualified_name() -> &'static str {
    "StardustXR.Message"
  }

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Message { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args MessageArgs<'args>
  ) -> flatbuffers::WIPOffset<Message<'bldr>> {
    let mut builder = MessageBuilder::new(_fbb);
    builder.add_id(args.id);
    if let Some(x) = args.data { builder.add_data(x); }
    if let Some(x) = args.error { builder.add_error(x); }
    if let Some(x) = args.method { builder.add_method(x); }
    if let Some(x) = args.object { builder.add_object(x); }
    builder.add_type_(args.type_);
    builder.finish()
  }

  pub fn unpack(&self) -> MessageT {
    let type_ = self.type_();
    let id = self.id();
    let object = self.object().map(|x| {
      x.to_string()
    });
    let method = self.method().map(|x| {
      x.to_string()
    });
    let error = self.error().map(|x| {
      x.to_string()
    });
    let data = self.data().map(|x| {
      x.into_iter().collect()
    });
    MessageT {
      type_,
      id,
      object,
      method,
      error,
      data,
    }
  }

  #[inline]
  pub fn type_(&self) -> u8 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u8>(Message::VT_TYPE_, Some(0)).unwrap()}
  }
  #[inline]
  pub fn id(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(Message::VT_ID, Some(0)).unwrap()}
  }
  #[inline]
  pub fn object(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Message::VT_OBJECT, None)}
  }
  #[inline]
  pub fn method(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Message::VT_METHOD, None)}
  }
  #[inline]
  pub fn error(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Message::VT_ERROR, None)}
  }
  #[inline]
  pub fn data(&self) -> Option<flatbuffers::Vector<'a, u8>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Message::VT_DATA, None)}
  }
}

impl flatbuffers::Verifiable for Message<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u8>("type_", Self::VT_TYPE_, false)?
     .visit_field::<u64>("id", Self::VT_ID, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("object", Self::VT_OBJECT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("method", Self::VT_METHOD, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("error", Self::VT_ERROR, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, u8>>>("data", Self::VT_DATA, false)?
     .finish();
    Ok(())
  }
}
pub struct MessageArgs<'a> {
    pub type_: u8,
    pub id: u64,
    pub object: Option<flatbuffers::WIPOffset<&'a str>>,
    pub method: Option<flatbuffers::WIPOffset<&'a str>>,
    pub error: Option<flatbuffers::WIPOffset<&'a str>>,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, u8>>>,
}
impl<'a> Default for MessageArgs<'a> {
  #[inline]
  fn default() -> Self {
    MessageArgs {
      type_: 0,
      id: 0,
      object: None,
      method: None,
      error: None,
      data: None,
    }
  }
}

pub struct MessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> MessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_type_(&mut self, type_: u8) {
    self.fbb_.push_slot::<u8>(Message::VT_TYPE_, type_, 0);
  }
  #[inline]
  pub fn add_id(&mut self, id: u64) {
    self.fbb_.push_slot::<u64>(Message::VT_ID, id, 0);
  }
  #[inline]
  pub fn add_object(&mut self, object: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_OBJECT, object);
  }
  #[inline]
  pub fn add_method(&mut self, method: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_METHOD, method);
  }
  #[inline]
  pub fn add_error(&mut self, error: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_ERROR, error);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Message::VT_DATA, data);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> MessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    MessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Message<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Message<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Message");
      ds.field("type_", &self.type_());
      ds.field("id", &self.id());
      ds.field("object", &self.object());
      ds.field("method", &self.method());
      ds.field("error", &self.error());
      ds.field("data", &self.data());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct MessageT {
  pub type_: u8,
  pub id: u64,
  pub object: Option<String>,
  pub method: Option<String>,
  pub error: Option<String>,
  pub data: Option<Vec<u8>>,
}
impl Default for MessageT {
  fn default() -> Self {
    Self {
      type_: 0,
      id: 0,
      object: None,
      method: None,
      error: None,
      data: None,
    }
  }
}
impl MessageT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<Message<'b>> {
    let type_ = self.type_;
    let id = self.id;
    let object = self.object.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let method = self.method.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let error = self.error.as_ref().map(|x|{
      _fbb.create_string(x)
    });
    let data = self.data.as_ref().map(|x|{
      _fbb.create_vector(x)
    });
    Message::create(_fbb, &MessageArgs{
      type_,
      id,
      object,
      method,
      error,
      data,
    })
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Message`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_message_unchecked`.
pub fn root_as_message(buf: &[u8]) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Message>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Message` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_message_unchecked`.
pub fn size_prefixed_root_as_message(buf: &[u8]) -> Result<Message, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Message>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Message` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_message_unchecked`.
pub fn root_as_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Message<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Message` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_message_unchecked`.
pub fn size_prefixed_root_as_message_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Message<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Message<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Message and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Message`.
pub unsafe fn root_as_message_unchecked(buf: &[u8]) -> Message {
  flatbuffers::root_unchecked::<Message>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Message and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Message`.
pub unsafe fn size_prefixed_root_as_message_unchecked(buf: &[u8]) -> Message {
  flatbuffers::size_prefixed_root_unchecked::<Message>(buf)
}
#[inline]
pub fn finish_message_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_message_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Message<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod StardustXR

