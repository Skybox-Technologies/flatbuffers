// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod my_game {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod example {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_TEST_ENUM: i8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_TEST_ENUM: i8 = 2;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_TEST_ENUM: [TestEnum; 3] = [
  TestEnum::A,
  TestEnum::B,
  TestEnum::C,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, num_derive::FromPrimitive, serde::Serialize, serde::Deserialize)]
#[repr(transparent)]
pub struct TestEnum(pub i8);
#[allow(non_upper_case_globals)]
impl TestEnum {
  pub const A: Self = Self(0);
  pub const B: Self = Self(1);
  pub const C: Self = Self(2);

  pub const ENUM_MIN: i8 = 0;
  pub const ENUM_MAX: i8 = 2;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::A,
    Self::B,
    Self::C,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::A => Some("A"),
      Self::B => Some("B"),
      Self::C => Some("C"),
      _ => None,
    }
  }
}
impl std::fmt::Debug for TestEnum {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for TestEnum {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = unsafe {
      flatbuffers::read_scalar_at::<i8>(buf, loc)
    };
    Self(b)
  }
}

impl flatbuffers::Push for TestEnum {
    type Output = TestEnum;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe { flatbuffers::emplace_scalar::<i8>(dst, self.0); }
    }
}

impl flatbuffers::EndianScalar for TestEnum {
  #[inline]
  fn to_little_endian(self) -> Self {
    let b = i8::to_le(self.0);
    Self(b)
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(self) -> Self {
    let b = i8::from_le(self.0);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for TestEnum {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for TestEnum {}
// struct NestedStruct, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct NestedStruct(pub [u8; 32]);
impl Default for NestedStruct { 
  fn default() -> Self { 
    Self([0; 32])
  }
}
impl std::fmt::Debug for NestedStruct {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("NestedStruct")
      .field("a", &self.a())
      .field("b", &self.b())
      .field("c", &self.c())
      .field("d", &self.d())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for NestedStruct {}
impl flatbuffers::SafeSliceAccess for NestedStruct {}
impl<'a> flatbuffers::Follow<'a> for NestedStruct {
  type Inner = &'a NestedStruct;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a NestedStruct>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a NestedStruct {
  type Inner = &'a NestedStruct;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<NestedStruct>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for NestedStruct {
    type Output = NestedStruct;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const NestedStruct as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b NestedStruct {
    type Output = NestedStruct;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const NestedStruct as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for NestedStruct {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl<'a> NestedStruct {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    a: &[i32; 2],
    b: TestEnum,
    c: &[TestEnum; 2],
    d: &[i64; 2],
  ) -> Self {
    let mut s = Self([0; 32]);
    s.set_a(&a);
    s.set_b(b);
    s.set_c(&c);
    s.set_d(&d);
    s
  }

  pub fn a(&'a self) -> flatbuffers::Array<'a, i32, 2> {
    flatbuffers::Array::follow(&self.0, 0)
  }

  pub fn set_a(&mut self, items: &[i32; 2]) {
    flatbuffers::emplace_scalar_array(&mut self.0, 0, items);
  }

  pub fn b(&self) -> TestEnum {
    let mut mem = core::mem::MaybeUninit::<TestEnum>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[8..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<TestEnum>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_b(&mut self, x: TestEnum) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const TestEnum as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<TestEnum>(),
      );
    }
  }

  pub fn c(&'a self) -> flatbuffers::Array<'a, TestEnum, 2> {
    flatbuffers::Array::follow(&self.0, 9)
  }

  pub fn set_c(&mut self, x: &[TestEnum; 2]) {
    unsafe {
      std::ptr::copy(
        x.as_ptr() as *const u8,
        self.0.as_mut_ptr().add(9),
        2,
      );
    }
  }

  pub fn d(&'a self) -> flatbuffers::Array<'a, i64, 2> {
    flatbuffers::Array::follow(&self.0, 16)
  }

  pub fn set_d(&mut self, items: &[i64; 2]) {
    flatbuffers::emplace_scalar_array(&mut self.0, 16, items);
  }

  pub fn unpack(&self) -> NestedStructT {
    NestedStructT {
      a: self.a().into(),
      b: self.b(),
      c: self.c().into(),
      d: self.d().into(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct NestedStructT {
  pub a: [i32; 2],
  pub b: TestEnum,
  pub c: [TestEnum; 2],
  pub d: [i64; 2],
}
impl NestedStructT {
  pub fn pack(&self) -> NestedStruct {
    NestedStruct::new(
      &self.a,
      self.b,
      &self.c,
      &self.d,
    )
  }
}

// struct ArrayStruct, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct ArrayStruct(pub [u8; 160]);
impl Default for ArrayStruct { 
  fn default() -> Self { 
    Self([0; 160])
  }
}
impl std::fmt::Debug for ArrayStruct {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.debug_struct("ArrayStruct")
      .field("a", &self.a())
      .field("b", &self.b())
      .field("c", &self.c())
      .field("d", &self.d())
      .field("e", &self.e())
      .field("f", &self.f())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ArrayStruct {}
impl flatbuffers::SafeSliceAccess for ArrayStruct {}
impl<'a> flatbuffers::Follow<'a> for ArrayStruct {
  type Inner = &'a ArrayStruct;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a ArrayStruct>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a ArrayStruct {
  type Inner = &'a ArrayStruct;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<ArrayStruct>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for ArrayStruct {
    type Output = ArrayStruct;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(self as *const ArrayStruct as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b ArrayStruct {
    type Output = ArrayStruct;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::std::slice::from_raw_parts(*self as *const ArrayStruct as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for ArrayStruct {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}
impl<'a> ArrayStruct {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    a: f32,
    b: &[i32; 15],
    c: i8,
    d: &[NestedStruct; 2],
    e: i32,
    f: &[i64; 2],
  ) -> Self {
    let mut s = Self([0; 160]);
    s.set_a(a);
    s.set_b(&b);
    s.set_c(c);
    s.set_d(&d);
    s.set_e(e);
    s.set_f(&f);
    s
  }

  pub fn a(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<f32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_a(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f32 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<f32>(),
      );
    }
  }

  pub fn b(&'a self) -> flatbuffers::Array<'a, i32, 15> {
    flatbuffers::Array::follow(&self.0, 4)
  }

  pub fn set_b(&mut self, items: &[i32; 15]) {
    flatbuffers::emplace_scalar_array(&mut self.0, 4, items);
  }

  pub fn c(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[64..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_c(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[64..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

  pub fn d(&'a self) -> flatbuffers::Array<'a, NestedStruct, 2> {
    flatbuffers::Array::follow(&self.0, 72)
  }

  pub fn set_d(&mut self, x: &[NestedStruct; 2]) {
    unsafe {
      std::ptr::copy(
        x.as_ptr() as *const u8,
        self.0.as_mut_ptr().add(72),
        64,
      );
    }
  }

  pub fn e(&self) -> i32 {
    let mut mem = core::mem::MaybeUninit::<i32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[136..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_e(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i32 as *const u8,
        self.0[136..].as_mut_ptr(),
        core::mem::size_of::<i32>(),
      );
    }
  }

  pub fn f(&'a self) -> flatbuffers::Array<'a, i64, 2> {
    flatbuffers::Array::follow(&self.0, 144)
  }

  pub fn set_f(&mut self, items: &[i64; 2]) {
    flatbuffers::emplace_scalar_array(&mut self.0, 144, items);
  }

  pub fn unpack(&self) -> ArrayStructT {
    ArrayStructT {
      a: self.a(),
      b: self.b().into(),
      c: self.c(),
      d: { let d = self.d(); flatbuffers::array_init(|i| d.get(i).unpack()) },
      e: self.e(),
      f: self.f().into(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ArrayStructT {
  pub a: f32,
  pub b: [i32; 15],
  pub c: i8,
  pub d: [NestedStructT; 2],
  pub e: i32,
  pub f: [i64; 2],
}
impl ArrayStructT {
  pub fn pack(&self) -> ArrayStruct {
    ArrayStruct::new(
      self.a,
      &self.b,
      self.c,
      &flatbuffers::array_init(|i| self.d[i].pack()),
      self.e,
      &self.f,
    )
  }
}

pub enum ArrayTableOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct ArrayTable<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ArrayTable<'a> {
    type Inner = ArrayTable<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self { _tab: flatbuffers::Table { buf, loc } }
    }
}

impl<'a> ArrayTable<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ArrayTable { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ArrayTableArgs<'args>) -> flatbuffers::WIPOffset<ArrayTable<'bldr>> {
      let mut builder = ArrayTableBuilder::new(_fbb);
      if let Some(x) = args.a { builder.add_a(x); }
      builder.finish()
    }

    pub fn unpack(&self) -> ArrayTableT {
      let a = self.a().map(|x| {
        x.unpack()
      });
      ArrayTableT {
        a,
      }
    }
    pub const VT_A: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn a(&self) -> Option<&'a ArrayStruct> {
    self._tab.get::<ArrayStruct>(ArrayTable::VT_A, None)
  }
}

impl flatbuffers::Verifiable for ArrayTable<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<ArrayStruct>(&"a", Self::VT_A, false)?
     .finish();
    Ok(())
  }
}
pub struct ArrayTableArgs<'a> {
    pub a: Option<&'a ArrayStruct>,
}
impl<'a> Default for ArrayTableArgs<'a> {
    #[inline]
    fn default() -> Self {
        ArrayTableArgs {
            a: None,
        }
    }
}
pub struct ArrayTableBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ArrayTableBuilder<'a, 'b> {
  #[inline]
  pub fn add_a(&mut self, a: &ArrayStruct) {
    self.fbb_.push_slot_always::<&ArrayStruct>(ArrayTable::VT_A, a);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ArrayTableBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ArrayTableBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ArrayTable<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for ArrayTable<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("ArrayTable");
      ds.field("a", &self.a());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayTableT {
  pub a: Option<ArrayStructT>,
}
impl Default for ArrayTableT {
  fn default() -> Self {
    Self {
      a: None,
    }
  }
}
impl ArrayTableT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<ArrayTable<'b>> {
    let a_tmp = self.a.as_ref().map(|x| x.pack());
    let a = a_tmp.as_ref();
    ArrayTable::create(_fbb, &ArrayTableArgs{
      a,
    })
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_array_table<'a>(buf: &'a [u8]) -> ArrayTable<'a> {
  unsafe { flatbuffers::root_unchecked::<ArrayTable<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_array_table<'a>(buf: &'a [u8]) -> ArrayTable<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<ArrayTable<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `ArrayTable`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_array_table_unchecked`.
pub fn root_as_array_table(buf: &[u8]) -> Result<ArrayTable, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<ArrayTable>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `ArrayTable` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_array_table_unchecked`.
pub fn size_prefixed_root_as_array_table(buf: &[u8]) -> Result<ArrayTable, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<ArrayTable>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `ArrayTable` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_array_table_unchecked`.
pub fn root_as_array_table_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ArrayTable<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<ArrayTable<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `ArrayTable` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_array_table_unchecked`.
pub fn size_prefixed_root_as_array_table_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<ArrayTable<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<ArrayTable<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a ArrayTable and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `ArrayTable`.
pub unsafe fn root_as_array_table_unchecked(buf: &[u8]) -> ArrayTable {
  flatbuffers::root_unchecked::<ArrayTable>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed ArrayTable and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `ArrayTable`.
pub unsafe fn size_prefixed_root_as_array_table_unchecked(buf: &[u8]) -> ArrayTable {
  flatbuffers::size_prefixed_root_unchecked::<ArrayTable>(buf)
}
pub const ARRAY_TABLE_IDENTIFIER: &str = "ARRT";

#[inline]
pub fn array_table_buffer_has_identifier(buf: &[u8]) -> bool {
  flatbuffers::buffer_has_identifier(buf, ARRAY_TABLE_IDENTIFIER, false)
}

#[inline]
pub fn array_table_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
  flatbuffers::buffer_has_identifier(buf, ARRAY_TABLE_IDENTIFIER, true)
}

pub const ARRAY_TABLE_EXTENSION: &str = "mon";

#[inline]
pub fn finish_array_table_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<ArrayTable<'a>>) {
  fbb.finish(root, Some(ARRAY_TABLE_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_array_table_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<ArrayTable<'a>>) {
  fbb.finish_size_prefixed(root, Some(ARRAY_TABLE_IDENTIFIER));
}
}  // pub mod Example
}  // pub mod MyGame

