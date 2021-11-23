// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `api.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct AuthRequest {
    // message fields
    pub password: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a AuthRequest {
    fn default() -> &'a AuthRequest {
        <AuthRequest as ::protobuf::Message>::default_instance()
    }
}

impl AuthRequest {
    pub fn new() -> AuthRequest {
        ::std::default::Default::default()
    }

    // string password = 1;


    pub fn get_password(&self) -> &str {
        &self.password
    }
    pub fn clear_password(&mut self) {
        self.password.clear();
    }

    // Param is passed by value, moved
    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_password(&mut self) -> &mut ::std::string::String {
        &mut self.password
    }

    // Take field
    pub fn take_password(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.password, ::std::string::String::new())
    }
}

impl ::protobuf::Message for AuthRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.password)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.password);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.password.is_empty() {
            os.write_string(1, &self.password)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> AuthRequest {
        AuthRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "password",
                |m: &AuthRequest| { &m.password },
                |m: &mut AuthRequest| { &mut m.password },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<AuthRequest>(
                "AuthRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static AuthRequest {
        static instance: ::protobuf::rt::LazyV2<AuthRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(AuthRequest::new)
    }
}

impl ::protobuf::Clear for AuthRequest {
    fn clear(&mut self) {
        self.password.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AuthRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AuthRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Empty {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Empty {
    fn default() -> &'a Empty {
        <Empty as ::protobuf::Message>::default_instance()
    }
}

impl Empty {
    pub fn new() -> Empty {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for Empty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Empty {
        Empty::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let fields = ::std::vec::Vec::new();
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Empty>(
                "Empty",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Empty {
        static instance: ::protobuf::rt::LazyV2<Empty> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Empty::new)
    }
}

impl ::protobuf::Clear for Empty {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Empty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Empty {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DataSourceResponse {
    // message fields
    pub data_source_name: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DataSourceResponse {
    fn default() -> &'a DataSourceResponse {
        <DataSourceResponse as ::protobuf::Message>::default_instance()
    }
}

impl DataSourceResponse {
    pub fn new() -> DataSourceResponse {
        ::std::default::Default::default()
    }

    // string data_source_name = 1;


    pub fn get_data_source_name(&self) -> &str {
        &self.data_source_name
    }
    pub fn clear_data_source_name(&mut self) {
        self.data_source_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_source_name(&mut self, v: ::std::string::String) {
        self.data_source_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_source_name(&mut self) -> &mut ::std::string::String {
        &mut self.data_source_name
    }

    // Take field
    pub fn take_data_source_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.data_source_name, ::std::string::String::new())
    }
}

impl ::protobuf::Message for DataSourceResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.data_source_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.data_source_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.data_source_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.data_source_name.is_empty() {
            os.write_string(1, &self.data_source_name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> DataSourceResponse {
        DataSourceResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "data_source_name",
                |m: &DataSourceResponse| { &m.data_source_name },
                |m: &mut DataSourceResponse| { &mut m.data_source_name },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<DataSourceResponse>(
                "DataSourceResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static DataSourceResponse {
        static instance: ::protobuf::rt::LazyV2<DataSourceResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(DataSourceResponse::new)
    }
}

impl ::protobuf::Clear for DataSourceResponse {
    fn clear(&mut self) {
        self.data_source_name.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DataSourceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DataSourceResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InspektorPolicy {
    // message fields
    pub wasm_byte_code: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a InspektorPolicy {
    fn default() -> &'a InspektorPolicy {
        <InspektorPolicy as ::protobuf::Message>::default_instance()
    }
}

impl InspektorPolicy {
    pub fn new() -> InspektorPolicy {
        ::std::default::Default::default()
    }

    // bytes wasm_byte_code = 1;


    pub fn get_wasm_byte_code(&self) -> &[u8] {
        &self.wasm_byte_code
    }
    pub fn clear_wasm_byte_code(&mut self) {
        self.wasm_byte_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_wasm_byte_code(&mut self, v: ::std::vec::Vec<u8>) {
        self.wasm_byte_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_wasm_byte_code(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.wasm_byte_code
    }

    // Take field
    pub fn take_wasm_byte_code(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.wasm_byte_code, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for InspektorPolicy {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.wasm_byte_code)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.wasm_byte_code.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.wasm_byte_code);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.wasm_byte_code.is_empty() {
            os.write_bytes(1, &self.wasm_byte_code)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> InspektorPolicy {
        InspektorPolicy::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "wasm_byte_code",
                |m: &InspektorPolicy| { &m.wasm_byte_code },
                |m: &mut InspektorPolicy| { &mut m.wasm_byte_code },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<InspektorPolicy>(
                "InspektorPolicy",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static InspektorPolicy {
        static instance: ::protobuf::rt::LazyV2<InspektorPolicy> = ::protobuf::rt::LazyV2::INIT;
        instance.get(InspektorPolicy::new)
    }
}

impl ::protobuf::Clear for InspektorPolicy {
    fn clear(&mut self) {
        self.wasm_byte_code.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InspektorPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InspektorPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tapi.proto\x12\x03api\")\n\x0bAuthRequest\x12\x1a\n\x08password\x18\
    \x01\x20\x01(\tR\x08password\"\x07\n\x05Empty\">\n\x12DataSourceResponse\
    \x12(\n\x10data_source_name\x18\x01\x20\x01(\tR\x0edataSourceName\"7\n\
    \x0fInspektorPolicy\x12$\n\x0ewasm_byte_code\x18\x01\x20\x01(\x0cR\x0cwa\
    smByteCode2\x9b\x01\n\tInspektor\x12&\n\x04Auth\x12\x10.api.AuthRequest\
    \x1a\n.api.Empty\"\0\x12.\n\x06Policy\x12\n.api.Empty\x1a\x14.api.Inspek\
    torPolicy\"\00\x01\x126\n\rGetDataSource\x12\n.api.Empty\x1a\x17.api.Dat\
    aSourceResponse\"\0B\x17Z\x15controlplane/apiprotob\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
