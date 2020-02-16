// Do not edit: This code was generated by flatdata's generator.
pub mod n {

pub mod schema {
pub mod structs {
pub const S: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
    @range( y_range )
    first_y : u32 : 14;
}
}

"#;}pub mod a {
pub const A: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
    @range( y_range )
    first_y : u32 : 14;
}
}

namespace n {
archive A
{
    data : vector< .n.S >;
}
}

"#;
pub mod resources {pub const DATA: &str = r#"namespace n {
struct S
{
    x : u64 : 64;
    @range( y_range )
    first_y : u32 : 14;
}
}

namespace n {
archive A
{
    data : vector< .n.S >;
}
}

"#;}
}
}
#[derive(Clone, Debug)]
pub struct S {}

#[derive(Clone, Copy)]
pub struct SRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for S
{
    const SCHEMA: &'static str = schema::structs::S;
    const SIZE_IN_BYTES: usize = 10;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;

    type Item = SRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item{ data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = SMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut{ data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}


impl<'a> SRef<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }    #[inline]
    pub fn first_y(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 64, 14);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }#[inline]
    pub fn y_range(&self) -> std::ops::Range<u32> {
        let start = flatdata_read_bytes!(u32, self.data, 64, 14);
        let end = flatdata_read_bytes!(u32, self.data, 64 + 10 * 8, 14);
        start..end
    }


    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for SRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("S")
            .field("x", &self.x())
            .field("first_y", &self.first_y())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for SRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() &&        self.first_y() == other.first_y()     }
}

impl<'a> flatdata::Ref for SRef<'a> {}

pub struct SMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> SMut<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn set_x(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 10)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }

    #[inline]
    pub fn first_y(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 64, 14);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[inline]
    pub fn set_first_y(&mut self, value: u32) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 10)
        };
        flatdata_write_bytes!(u32; value, buffer, 64, 14)
    }


    #[inline]
    pub fn fill_from(&mut self, other: &SRef) {
        self.set_x(other.x());
        self.set_first_y(other.first_y());
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.data
    }
}

impl<'a> std::fmt::Debug for SMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        SRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for SMut<'a> {}




#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    data: flatdata::MemoryDescriptor,
}

impl A {
    fn read_resource(
        storage: &dyn flatdata::ResourceStorage,
        name: &str,
        schema: &str,
    ) -> Result<flatdata::MemoryDescriptor, flatdata::ResourceStorageError>
    {
        storage.read(name, schema).map(|x| flatdata::MemoryDescriptor::new(&x))
    }

    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> flatdata::ArrayView<super::n::S>
    {
        flatdata::ArrayView::new(&unsafe {self.data.as_bytes()})
    }

}

impl ::std::fmt::Debug for A {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("A")
            .field("data", &self.data())
            .finish()
    }
}

impl flatdata::Archive for A {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let data = Self::read_resource(&*storage, "data", schema::a::resources::DATA)?;

        Ok(Self {
            _storage: storage,
            data,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ABuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl ABuilder {
    #[inline]
    pub fn set_data(&self, vector: &flatdata::ArrayView<super::n::S>) -> ::std::io::Result<()> {
        self.storage.write("data", schema::a::resources::DATA, vector.as_ref())
    }

    #[inline]
    pub fn start_data(&self) -> ::std::io::Result<flatdata::ExternalVector<super::n::S>> {
        flatdata::create_external_vector(&*self.storage, "data", schema::a::resources::DATA)
    }

}

impl flatdata::ArchiveBuilder for ABuilder {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}


}
