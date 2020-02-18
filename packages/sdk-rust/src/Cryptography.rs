// This file is generated by rust-protobuf 2.10.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `Cryptography.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

#[derive(PartialEq,Clone,Default)]
pub struct Signature {
    // message fields
    pub signing_context: ::protobuf::SingularPtrField<SigningContext>,
    pub raw_bytes: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Signature {
    fn default() -> &'a Signature {
        <Signature as ::protobuf::Message>::default_instance()
    }
}

impl Signature {
    pub fn new() -> Signature {
        ::std::default::Default::default()
    }

    // .Catalyst.Protocol.Cryptography.SigningContext signing_context = 1;


    pub fn get_signing_context(&self) -> &SigningContext {
        self.signing_context.as_ref().unwrap_or_else(|| SigningContext::default_instance())
    }
    pub fn clear_signing_context(&mut self) {
        self.signing_context.clear();
    }

    pub fn has_signing_context(&self) -> bool {
        self.signing_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signing_context(&mut self, v: SigningContext) {
        self.signing_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signing_context(&mut self) -> &mut SigningContext {
        if self.signing_context.is_none() {
            self.signing_context.set_default();
        }
        self.signing_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_signing_context(&mut self) -> SigningContext {
        self.signing_context.take().unwrap_or_else(|| SigningContext::new())
    }

    // bytes raw_bytes = 2;


    pub fn get_raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }
    pub fn clear_raw_bytes(&mut self) {
        self.raw_bytes.clear();
    }

    // Param is passed by value, moved
    pub fn set_raw_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.raw_bytes = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.raw_bytes
    }

    // Take field
    pub fn take_raw_bytes(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.raw_bytes, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Signature {
    fn is_initialized(&self) -> bool {
        for v in &self.signing_context {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.signing_context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.raw_bytes)?;
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
        if let Some(ref v) = self.signing_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.raw_bytes.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.raw_bytes);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.signing_context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.raw_bytes.is_empty() {
            os.write_bytes(2, &self.raw_bytes)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Signature {
        Signature::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SigningContext>>(
                    "signing_context",
                    |m: &Signature| { &m.signing_context },
                    |m: &mut Signature| { &mut m.signing_context },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "raw_bytes",
                    |m: &Signature| { &m.raw_bytes },
                    |m: &mut Signature| { &mut m.raw_bytes },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Signature>(
                    "Signature",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Signature {
        static mut instance: ::protobuf::lazy::Lazy<Signature> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Signature,
        };
        unsafe {
            instance.get(Signature::new)
        }
    }
}

impl ::protobuf::Clear for Signature {
    fn clear(&mut self) {
        self.signing_context.clear();
        self.raw_bytes.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Signature {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SigningContext {
    // message fields
    pub network_type: super::Network::NetworkType,
    pub signature_type: SignatureType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SigningContext {
    fn default() -> &'a SigningContext {
        <SigningContext as ::protobuf::Message>::default_instance()
    }
}

impl SigningContext {
    pub fn new() -> SigningContext {
        ::std::default::Default::default()
    }

    // .Catalyst.Protocol.Network.NetworkType network_type = 1;


    pub fn get_network_type(&self) -> super::Network::NetworkType {
        self.network_type
    }
    pub fn clear_network_type(&mut self) {
        self.network_type = super::Network::NetworkType::NETWORK_TYPE_UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_network_type(&mut self, v: super::Network::NetworkType) {
        self.network_type = v;
    }

    // .Catalyst.Protocol.Cryptography.SignatureType signature_type = 2;


    pub fn get_signature_type(&self) -> SignatureType {
        self.signature_type
    }
    pub fn clear_signature_type(&mut self) {
        self.signature_type = SignatureType::SIGNATURE_TYPE_UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_signature_type(&mut self, v: SignatureType) {
        self.signature_type = v;
    }
}

impl ::protobuf::Message for SigningContext {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.network_type, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.signature_type, 2, &mut self.unknown_fields)?
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
        if self.network_type != super::Network::NetworkType::NETWORK_TYPE_UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.network_type);
        }
        if self.signature_type != SignatureType::SIGNATURE_TYPE_UNKNOWN {
            my_size += ::protobuf::rt::enum_size(2, self.signature_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.network_type != super::Network::NetworkType::NETWORK_TYPE_UNKNOWN {
            os.write_enum(1, self.network_type.value())?;
        }
        if self.signature_type != SignatureType::SIGNATURE_TYPE_UNKNOWN {
            os.write_enum(2, self.signature_type.value())?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SigningContext {
        SigningContext::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::Network::NetworkType>>(
                    "network_type",
                    |m: &SigningContext| { &m.network_type },
                    |m: &mut SigningContext| { &mut m.network_type },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<SignatureType>>(
                    "signature_type",
                    |m: &SigningContext| { &m.signature_type },
                    |m: &mut SigningContext| { &mut m.signature_type },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SigningContext>(
                    "SigningContext",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SigningContext {
        static mut instance: ::protobuf::lazy::Lazy<SigningContext> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SigningContext,
        };
        unsafe {
            instance.get(SigningContext::new)
        }
    }
}

impl ::protobuf::Clear for SigningContext {
    fn clear(&mut self) {
        self.network_type = super::Network::NetworkType::NETWORK_TYPE_UNKNOWN;
        self.signature_type = SignatureType::SIGNATURE_TYPE_UNKNOWN;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SigningContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SigningContext {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignatureBatch {
    // message fields
    pub signatures: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub public_keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub messages: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub context: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SignatureBatch {
    fn default() -> &'a SignatureBatch {
        <SignatureBatch as ::protobuf::Message>::default_instance()
    }
}

impl SignatureBatch {
    pub fn new() -> SignatureBatch {
        ::std::default::Default::default()
    }

    // repeated bytes signatures = 1;


    pub fn get_signatures(&self) -> &[::std::vec::Vec<u8>] {
        &self.signatures
    }
    pub fn clear_signatures(&mut self) {
        self.signatures.clear();
    }

    // Param is passed by value, moved
    pub fn set_signatures(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.signatures = v;
    }

    // Mutable pointer to the field.
    pub fn mut_signatures(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.signatures
    }

    // Take field
    pub fn take_signatures(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.signatures, ::protobuf::RepeatedField::new())
    }

    // repeated bytes public_keys = 2;


    pub fn get_public_keys(&self) -> &[::std::vec::Vec<u8>] {
        &self.public_keys
    }
    pub fn clear_public_keys(&mut self) {
        self.public_keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.public_keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_public_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.public_keys
    }

    // Take field
    pub fn take_public_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.public_keys, ::protobuf::RepeatedField::new())
    }

    // repeated bytes messages = 3;


    pub fn get_messages(&self) -> &[::std::vec::Vec<u8>] {
        &self.messages
    }
    pub fn clear_messages(&mut self) {
        self.messages.clear();
    }

    // Param is passed by value, moved
    pub fn set_messages(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.messages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_messages(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.messages
    }

    // Take field
    pub fn take_messages(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.messages, ::protobuf::RepeatedField::new())
    }

    // bytes context = 4;


    pub fn get_context(&self) -> &[u8] {
        &self.context
    }
    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: ::std::vec::Vec<u8>) {
        self.context = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.context
    }

    // Take field
    pub fn take_context(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.context, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for SignatureBatch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.signatures)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.public_keys)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.messages)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.context)?;
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
        for value in &self.signatures {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.public_keys {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in &self.messages {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        if !self.context.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.context);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.signatures {
            os.write_bytes(1, &v)?;
        };
        for v in &self.public_keys {
            os.write_bytes(2, &v)?;
        };
        for v in &self.messages {
            os.write_bytes(3, &v)?;
        };
        if !self.context.is_empty() {
            os.write_bytes(4, &self.context)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SignatureBatch {
        SignatureBatch::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signatures",
                    |m: &SignatureBatch| { &m.signatures },
                    |m: &mut SignatureBatch| { &mut m.signatures },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_keys",
                    |m: &SignatureBatch| { &m.public_keys },
                    |m: &mut SignatureBatch| { &mut m.public_keys },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "messages",
                    |m: &SignatureBatch| { &m.messages },
                    |m: &mut SignatureBatch| { &mut m.messages },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "context",
                    |m: &SignatureBatch| { &m.context },
                    |m: &mut SignatureBatch| { &mut m.context },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignatureBatch>(
                    "SignatureBatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SignatureBatch {
        static mut instance: ::protobuf::lazy::Lazy<SignatureBatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignatureBatch,
        };
        unsafe {
            instance.get(SignatureBatch::new)
        }
    }
}

impl ::protobuf::Clear for SignatureBatch {
    fn clear(&mut self) {
        self.signatures.clear();
        self.public_keys.clear();
        self.messages.clear();
        self.context.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignatureBatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignatureBatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SignatureType {
    SIGNATURE_TYPE_UNKNOWN = 0,
    TRANSACTION_PUBLIC = 1,
    TRANSACTION_CONFIDENTIAL = 2,
    PROTOCOL_RPC = 3,
    PROTOCOL_PEER = 4,
    WEB3_MESSAGE = 5,
}

impl ::protobuf::ProtobufEnum for SignatureType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SignatureType> {
        match value {
            0 => ::std::option::Option::Some(SignatureType::SIGNATURE_TYPE_UNKNOWN),
            1 => ::std::option::Option::Some(SignatureType::TRANSACTION_PUBLIC),
            2 => ::std::option::Option::Some(SignatureType::TRANSACTION_CONFIDENTIAL),
            3 => ::std::option::Option::Some(SignatureType::PROTOCOL_RPC),
            4 => ::std::option::Option::Some(SignatureType::PROTOCOL_PEER),
            5 => ::std::option::Option::Some(SignatureType::WEB3_MESSAGE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SignatureType] = &[
            SignatureType::SIGNATURE_TYPE_UNKNOWN,
            SignatureType::TRANSACTION_PUBLIC,
            SignatureType::TRANSACTION_CONFIDENTIAL,
            SignatureType::PROTOCOL_RPC,
            SignatureType::PROTOCOL_PEER,
            SignatureType::WEB3_MESSAGE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SignatureType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SignatureType {
}

impl ::std::default::Default for SignatureType {
    fn default() -> Self {
        SignatureType::SIGNATURE_TYPE_UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for SignatureType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ErrorCode {
    ERROR_CODE_UNKNOWN = 0,
    INVALID_SIGNATURE = 1,
    INVALID_PUBLIC_KEY = 2,
    INVALID_PRIVATE_KEY = 3,
    SIGNATURE_VERIFICATION_FAILURE = 4,
    INVALID_CONTEXT_LENGTH = 5,
    INVALID_BATCH_MESSAGE = 6,
    ARRAYS_NOT_EQUAL_LENGTH = 7,
    BATCH_VERIFICATION_FAILURE = 8,
    NO_ERROR = 418,
}

impl ::protobuf::ProtobufEnum for ErrorCode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ErrorCode> {
        match value {
            0 => ::std::option::Option::Some(ErrorCode::ERROR_CODE_UNKNOWN),
            1 => ::std::option::Option::Some(ErrorCode::INVALID_SIGNATURE),
            2 => ::std::option::Option::Some(ErrorCode::INVALID_PUBLIC_KEY),
            3 => ::std::option::Option::Some(ErrorCode::INVALID_PRIVATE_KEY),
            4 => ::std::option::Option::Some(ErrorCode::SIGNATURE_VERIFICATION_FAILURE),
            5 => ::std::option::Option::Some(ErrorCode::INVALID_CONTEXT_LENGTH),
            6 => ::std::option::Option::Some(ErrorCode::INVALID_BATCH_MESSAGE),
            7 => ::std::option::Option::Some(ErrorCode::ARRAYS_NOT_EQUAL_LENGTH),
            8 => ::std::option::Option::Some(ErrorCode::BATCH_VERIFICATION_FAILURE),
            418 => ::std::option::Option::Some(ErrorCode::NO_ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ErrorCode] = &[
            ErrorCode::ERROR_CODE_UNKNOWN,
            ErrorCode::INVALID_SIGNATURE,
            ErrorCode::INVALID_PUBLIC_KEY,
            ErrorCode::INVALID_PRIVATE_KEY,
            ErrorCode::SIGNATURE_VERIFICATION_FAILURE,
            ErrorCode::INVALID_CONTEXT_LENGTH,
            ErrorCode::INVALID_BATCH_MESSAGE,
            ErrorCode::ARRAYS_NOT_EQUAL_LENGTH,
            ErrorCode::BATCH_VERIFICATION_FAILURE,
            ErrorCode::NO_ERROR,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ErrorCode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ErrorCode {
}

impl ::std::default::Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::ERROR_CODE_UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorCode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12Cryptography.proto\x12\x1eCatalyst.Protocol.Cryptography\x1a\rNetw\
    ork.proto\"\x81\x01\n\tSignature\x12W\n\x0fsigning_context\x18\x01\x20\
    \x01(\x0b2..Catalyst.Protocol.Cryptography.SigningContextR\x0esigningCon\
    text\x12\x1b\n\traw_bytes\x18\x02\x20\x01(\x0cR\x08rawBytes\"\xb1\x01\n\
    \x0eSigningContext\x12I\n\x0cnetwork_type\x18\x01\x20\x01(\x0e2&.Catalys\
    t.Protocol.Network.NetworkTypeR\x0bnetworkType\x12T\n\x0esignature_type\
    \x18\x02\x20\x01(\x0e2-.Catalyst.Protocol.Cryptography.SignatureTypeR\rs\
    ignatureType\"\x87\x01\n\x0eSignatureBatch\x12\x1e\n\nsignatures\x18\x01\
    \x20\x03(\x0cR\nsignatures\x12\x1f\n\x0bpublic_keys\x18\x02\x20\x03(\x0c\
    R\npublicKeys\x12\x1a\n\x08messages\x18\x03\x20\x03(\x0cR\x08messages\
    \x12\x18\n\x07context\x18\x04\x20\x01(\x0cR\x07context*\x98\x01\n\rSigna\
    tureType\x12\x1a\n\x16SIGNATURE_TYPE_UNKNOWN\x10\0\x12\x16\n\x12TRANSACT\
    ION_PUBLIC\x10\x01\x12\x1c\n\x18TRANSACTION_CONFIDENTIAL\x10\x02\x12\x10\
    \n\x0cPROTOCOL_RPC\x10\x03\x12\x11\n\rPROTOCOL_PEER\x10\x04\x12\x10\n\
    \x0cWEB3_MESSAGE\x10\x05*\x92\x02\n\tErrorCode\x12\x16\n\x12ERROR_CODE_U\
    NKNOWN\x10\0\x12\x15\n\x11INVALID_SIGNATURE\x10\x01\x12\x16\n\x12INVALID\
    _PUBLIC_KEY\x10\x02\x12\x17\n\x13INVALID_PRIVATE_KEY\x10\x03\x12\"\n\x1e\
    SIGNATURE_VERIFICATION_FAILURE\x10\x04\x12\x1a\n\x16INVALID_CONTEXT_LENG\
    TH\x10\x05\x12\x19\n\x15INVALID_BATCH_MESSAGE\x10\x06\x12\x1b\n\x17ARRAY\
    S_NOT_EQUAL_LENGTH\x10\x07\x12\x1e\n\x1aBATCH_VERIFICATION_FAILURE\x10\
    \x08\x12\r\n\x08NO_ERROR\x10\xa2\x03B\x02P\x01J\x8e\x19\n\x06\x12\x04\
    \x13\0B\x01\n\xdf\x06\n\x01\x0c\x12\x03\x13\0\x122\xd4\x06*\n\x20Copyrig\
    ht\x20(c)\x202019\x20Catalyst\x20Network\n\n\x20This\x20file\x20is\x20pa\
    rt\x20of\x20Catalyst.Network.Protocol.Protobuffs\x20<https://github.com/\
    catalyst-network/protocol-protobuffs>\n\n\x20Catalyst.Network.Protocol.P\
    rotobuffs\x20is\x20free\x20software:\x20you\x20can\x20redistribute\x20it\
    \x20and/or\x20modify\n\x20it\x20under\x20the\x20terms\x20of\x20the\x20GN\
    U\x20General\x20Public\x20License\x20as\x20published\x20by\n\x20the\x20F\
    ree\x20Software\x20Foundation,\x20either\x20version\x202\x20of\x20the\
    \x20License,\x20or\n\x20(at\x20your\x20option)\x20any\x20later\x20versio\
    n.\n\x20\n\x20Catalyst.Network.Protocol.Protobuffs\x20is\x20distributed\
    \x20in\x20the\x20hope\x20that\x20it\x20will\x20be\x20useful,\n\x20but\
    \x20WITHOUT\x20ANY\x20WARRANTY;\x20without\x20even\x20the\x20implied\x20\
    warranty\x20of\n\x20MERCHANTABILITY\x20or\x20FITNESS\x20FOR\x20A\x20PART\
    ICULAR\x20PURPOSE.\x20See\x20the\n\x20GNU\x20General\x20Public\x20Licens\
    e\x20for\x20more\x20details.\n\x20\n\x20You\x20should\x20have\x20receive\
    d\x20a\x20copy\x20of\x20the\x20GNU\x20General\x20Public\x20License\n\x20\
    along\x20with\x20Catalyst.Network.Protocol.Protobuffs\x20If\x20not,\x20s\
    ee\x20<https://www.gnu.org/licenses/>.\n\n\x08\n\x01\x08\x12\x03\x15\0\"\
    \n\t\n\x02\x08\n\x12\x03\x15\0\"\n\t\n\x02\x03\0\x12\x03\x17\0\x17\n\x08\
    \n\x01\x02\x12\x03\x19\0'\n\n\n\x02\x04\0\x12\x04\x1b\0\x1e\x01\n\n\n\
    \x03\x04\0\x01\x12\x03\x1b\x08\x11\n%\n\x04\x04\0\x02\0\x12\x03\x1c\x08+\
    \"\x18\x20system\x20domain\x20context.\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\x1c\x08\x16\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1c\x17&\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\x1c)*\n\x20\n\x04\x04\0\x02\x01\x12\x03\x1d\
    \x08\x1c\"\x13\x20signature\x20digest.\n\n\x0c\n\x05\x04\0\x02\x01\x05\
    \x12\x03\x1d\x08\r\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x1d\x0e\x17\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x1d\x1a\x1b\n+\n\x02\x05\0\x12\x04!\
    \0(\x01\x1a\x1f\x20Represents\x20domains\x20of\x20a\x20node.\n\n\n\n\x03\
    \x05\0\x01\x12\x03!\x05\x12\n!\n\x04\x05\0\x02\0\x12\x03\"\x08#\"\x14\
    \x20Unknown\x20signature.\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\"\x08\
    \x1e\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\"!\"\n2\n\x04\x05\0\x02\x01\x12\
    \x03#\x08\x1f\"%\x20Signatures\x20for\x20public\x20transactions.\n\n\x0c\
    \n\x05\x05\0\x02\x01\x01\x12\x03#\x08\x1a\n\x0c\n\x05\x05\0\x02\x01\x02\
    \x12\x03#\x1d\x1e\n8\n\x04\x05\0\x02\x02\x12\x03$\x08%\"+\x20Signatures\
    \x20for\x20confidential\x20transactions.\n\n\x0c\n\x05\x05\0\x02\x02\x01\
    \x12\x03$\x08\x20\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03$#$\n+\n\x04\x05\
    \0\x02\x03\x12\x03%\x08\x19\"\x1e\x20Signatures\x20for\x20rpc\x20message\
    s.\n\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03%\x08\x14\n\x0c\n\x05\x05\0\
    \x02\x03\x02\x12\x03%\x17\x18\n5\n\x04\x05\0\x02\x04\x12\x03&\x08\x1a\"(\
    \x20Signatures\x20for\x20peer\x20protocol\x20messages.\n\n\x0c\n\x05\x05\
    \0\x02\x04\x01\x12\x03&\x08\x15\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03&\
    \x18\x19\n<\n\x04\x05\0\x02\x05\x12\x03'\x08\x19\"/\x20Signatures\x20for\
    \x20messages\x20in\x20the\x20wen3\x20provider.\n\n\x0c\n\x05\x05\0\x02\
    \x05\x01\x12\x03'\x08\x14\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03'\x17\x18\
    \n\n\n\x02\x04\x01\x12\x04*\0-\x01\n\n\n\x03\x04\x01\x01\x12\x03*\x08\
    \x16\n9\n\x04\x04\x01\x02\0\x12\x03+\x08-\",\x20is\x20the\x20network\x20\
    enum\x20(mainet\x20/\x20devnet\x20etc).\n\n\x0c\n\x05\x04\x01\x02\0\x06\
    \x12\x03+\x08\x1b\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03+\x1c(\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03++,\na\n\x04\x04\x01\x02\x01\x12\x03,\x08)\
    \"T\x20contains\x20info\x20on\x20whether\x20the\x20signature\x20is\x20fo\
    r\x20a\x20protocol\x20message\x20or\x20a\x20transaction.\n\n\x0c\n\x05\
    \x04\x01\x02\x01\x06\x12\x03,\x08\x15\n\x0c\n\x05\x04\x01\x02\x01\x01\
    \x12\x03,\x16$\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03,'(\n\n\n\x02\x05\
    \x01\x12\x04/\0:\x01\n\n\n\x03\x05\x01\x01\x12\x03/\x05\x0e\n\x1d\n\x04\
    \x05\x01\x02\0\x12\x030\x08\x1f\"\x10\x20Unknown\x20error.\n\n\x0c\n\x05\
    \x05\x01\x02\0\x01\x12\x030\x08\x1a\n\x0c\n\x05\x05\x01\x02\0\x02\x12\
    \x030\x1d\x1e\nX\n\x04\x05\x01\x02\x01\x12\x031\x08\x1e\"K\x20Signature\
    \x20is\x20of\x20incorrect\x20length\x20or\x20does\x20not\x20correspond\
    \x20to\x20a\x20valid\x20point.\n\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03\
    1\x08\x19\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x031\x1c\x1d\n\\\n\x04\x05\
    \x01\x02\x02\x12\x032\x08\x1f\"O\x20Public\x20key\x20is\x20of\x20incorre\
    ct\x20length\x20or\x20cannot\x20be\x20decompressed\x20to\x20a\x20valid\
    \x20point.\n\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x032\x08\x1a\n\x0c\n\
    \x05\x05\x01\x02\x02\x02\x12\x032\x1d\x1e\n2\n\x04\x05\x01\x02\x03\x12\
    \x033\x08\x20\"%\x20Private\x20key\x20is\x20of\x20incorrect\x20length.\n\
    \n\x0c\n\x05\x05\x01\x02\x03\x01\x12\x033\x08\x1b\n\x0c\n\x05\x05\x01\
    \x02\x03\x02\x12\x033\x1e\x1f\nM\n\x04\x05\x01\x02\x04\x12\x034\x02%\"@\
    \x20Signature\x20cannot\x20be\x20verified\x20against\x20the\x20provided\
    \x20information.\n\n\x0c\n\x05\x05\x01\x02\x04\x01\x12\x034\x02\x20\n\
    \x0c\n\x05\x05\x01\x02\x04\x02\x12\x034#$\n9\n\x04\x05\x01\x02\x05\x12\
    \x035\x02\x1d\",\x20Context\x20exceed\x20the\x20maximum\x20allowed\x20le\
    ngth.\n\n\x0c\n\x05\x05\x01\x02\x05\x01\x12\x035\x02\x18\n\x0c\n\x05\x05\
    \x01\x02\x05\x02\x12\x035\x1b\x1c\n8\n\x04\x05\x01\x02\x06\x12\x036\x08\
    \"\"+\x20Unable\x20to\x20convert\x20message\x20to\x20valid\x20data.\x20\
    \n\n\x0c\n\x05\x05\x01\x02\x06\x01\x12\x036\x08\x1d\n\x0c\n\x05\x05\x01\
    \x02\x06\x02\x12\x036\x20!\np\n\x04\x05\x01\x02\x07\x12\x037\x08$\"c\x20\
    Cannot\x20perform\x20batch\x20verification\x20as\x20an\x20unequal\x20num\
    ber\x20of\x20messages/signatures/keys\x20were\x20provided.\n\n\x0c\n\x05\
    \x05\x01\x02\x07\x01\x12\x037\x08\x1f\n\x0c\n\x05\x05\x01\x02\x07\x02\
    \x12\x037\"#\n9\n\x04\x05\x01\x02\x08\x12\x038\x08'\",\x20One\x20or\x20m\
    ore\x20signatures\x20cannot\x20be\x20verified.\n\n\x0c\n\x05\x05\x01\x02\
    \x08\x01\x12\x038\x08\"\n\x0c\n\x05\x05\x01\x02\x08\x02\x12\x038%&\n(\n\
    \x04\x05\x01\x02\t\x12\x039\x08\x17\"\x1b\x20No\x20error\x20(just\x20a\
    \x20teapot).\n\n\x0c\n\x05\x05\x01\x02\t\x01\x12\x039\x08\x10\n\x0c\n\
    \x05\x05\x01\x02\t\x02\x12\x039\x13\x16\n=\n\x02\x04\x02\x12\x04=\0B\x01\
    \x1a1Holds\x20data\x20for\x20batch\x20verification\x20of\x20signatures.\
    \n\n\n\n\x03\x04\x02\x01\x12\x03=\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\
    \x03>\x08&\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03>\x08\x10\n\x0c\n\x05\
    \x04\x02\x02\0\x05\x12\x03>\x11\x16\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03>\x17!\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03>$%\n\x0b\n\x04\x04\x02\
    \x02\x01\x12\x03?\x08'\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03?\x08\x10\
    \n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03?\x11\x16\n\x0c\n\x05\x04\x02\
    \x02\x01\x01\x12\x03?\x17\"\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03?%&\n\
    \x0b\n\x04\x04\x02\x02\x02\x12\x03@\x08$\n\x0c\n\x05\x04\x02\x02\x02\x04\
    \x12\x03@\x08\x10\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03@\x11\x16\n\x0c\
    \n\x05\x04\x02\x02\x02\x01\x12\x03@\x17\x1f\n\x0c\n\x05\x04\x02\x02\x02\
    \x03\x12\x03@\"#\n\x0b\n\x04\x04\x02\x02\x03\x12\x03A\x08\x1a\n\x0c\n\
    \x05\x04\x02\x02\x03\x05\x12\x03A\x08\r\n\x0c\n\x05\x04\x02\x02\x03\x01\
    \x12\x03A\x0e\x15\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03A\x18\x19b\x06p\
    roto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
