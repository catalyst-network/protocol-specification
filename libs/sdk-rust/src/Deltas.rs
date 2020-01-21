// This file is generated by rust-protobuf 2.10.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
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
//! Generated file from `Deltas.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_0;

#[derive(PartialEq,Clone,Default)]
pub struct DeltaIndex {
    // message fields
    pub height: u32,
    pub cid: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a DeltaIndex {
    fn default() -> &'a DeltaIndex {
        <DeltaIndex as ::protobuf::Message>::default_instance()
    }
}

impl DeltaIndex {
    pub fn new() -> DeltaIndex {
        ::std::default::Default::default()
    }

    // uint32 height = 1;


    pub fn get_height(&self) -> u32 {
        self.height
    }
    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u32) {
        self.height = v;
    }

    // bytes cid = 2;


    pub fn get_cid(&self) -> &[u8] {
        &self.cid
    }
    pub fn clear_cid(&mut self) {
        self.cid.clear();
    }

    // Param is passed by value, moved
    pub fn set_cid(&mut self, v: ::std::vec::Vec<u8>) {
        self.cid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.cid
    }

    // Take field
    pub fn take_cid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.cid, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for DeltaIndex {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.height = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.cid)?;
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
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(1, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.cid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.cid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.height != 0 {
            os.write_uint32(1, self.height)?;
        }
        if !self.cid.is_empty() {
            os.write_bytes(2, &self.cid)?;
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

    fn new() -> DeltaIndex {
        DeltaIndex::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "height",
                    |m: &DeltaIndex| { &m.height },
                    |m: &mut DeltaIndex| { &mut m.height },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cid",
                    |m: &DeltaIndex| { &m.cid },
                    |m: &mut DeltaIndex| { &mut m.cid },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeltaIndex>(
                    "DeltaIndex",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static DeltaIndex {
        static mut instance: ::protobuf::lazy::Lazy<DeltaIndex> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeltaIndex,
        };
        unsafe {
            instance.get(DeltaIndex::new)
        }
    }
}

impl ::protobuf::Clear for DeltaIndex {
    fn clear(&mut self) {
        self.height = 0;
        self.cid.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeltaIndex {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeltaIndex {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Delta {
    // message fields
    pub previous_delta_dfs_hash: ::std::vec::Vec<u8>,
    pub merkle_root: ::std::vec::Vec<u8>,
    pub merkle_poda: ::std::vec::Vec<u8>,
    pub time_stamp: ::protobuf::SingularPtrField<::protobuf::well_known_types::Timestamp>,
    pub public_entries: ::protobuf::RepeatedField<super::Transaction::PublicEntry>,
    pub confidential_entries: ::protobuf::RepeatedField<super::Transaction::ConfidentialEntry>,
    pub coinbase_entries: ::protobuf::RepeatedField<super::Transaction::CoinbaseEntry>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Delta {
    fn default() -> &'a Delta {
        <Delta as ::protobuf::Message>::default_instance()
    }
}

impl Delta {
    pub fn new() -> Delta {
        ::std::default::Default::default()
    }

    // bytes previous_delta_dfs_hash = 1;


    pub fn get_previous_delta_dfs_hash(&self) -> &[u8] {
        &self.previous_delta_dfs_hash
    }
    pub fn clear_previous_delta_dfs_hash(&mut self) {
        self.previous_delta_dfs_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_previous_delta_dfs_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.previous_delta_dfs_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previous_delta_dfs_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.previous_delta_dfs_hash
    }

    // Take field
    pub fn take_previous_delta_dfs_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.previous_delta_dfs_hash, ::std::vec::Vec::new())
    }

    // bytes merkle_root = 2;


    pub fn get_merkle_root(&self) -> &[u8] {
        &self.merkle_root
    }
    pub fn clear_merkle_root(&mut self) {
        self.merkle_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_merkle_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.merkle_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merkle_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.merkle_root
    }

    // Take field
    pub fn take_merkle_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.merkle_root, ::std::vec::Vec::new())
    }

    // bytes merkle_poda = 3;


    pub fn get_merkle_poda(&self) -> &[u8] {
        &self.merkle_poda
    }
    pub fn clear_merkle_poda(&mut self) {
        self.merkle_poda.clear();
    }

    // Param is passed by value, moved
    pub fn set_merkle_poda(&mut self, v: ::std::vec::Vec<u8>) {
        self.merkle_poda = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merkle_poda(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.merkle_poda
    }

    // Take field
    pub fn take_merkle_poda(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.merkle_poda, ::std::vec::Vec::new())
    }

    // .google.protobuf.Timestamp time_stamp = 4;


    pub fn get_time_stamp(&self) -> &::protobuf::well_known_types::Timestamp {
        self.time_stamp.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::default_instance())
    }
    pub fn clear_time_stamp(&mut self) {
        self.time_stamp.clear();
    }

    pub fn has_time_stamp(&self) -> bool {
        self.time_stamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_stamp(&mut self, v: ::protobuf::well_known_types::Timestamp) {
        self.time_stamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_time_stamp(&mut self) -> &mut ::protobuf::well_known_types::Timestamp {
        if self.time_stamp.is_none() {
            self.time_stamp.set_default();
        }
        self.time_stamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_time_stamp(&mut self) -> ::protobuf::well_known_types::Timestamp {
        self.time_stamp.take().unwrap_or_else(|| ::protobuf::well_known_types::Timestamp::new())
    }

    // repeated .Catalyst.Protocol.Transaction.PublicEntry public_entries = 5;


    pub fn get_public_entries(&self) -> &[super::Transaction::PublicEntry] {
        &self.public_entries
    }
    pub fn clear_public_entries(&mut self) {
        self.public_entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_entries(&mut self, v: ::protobuf::RepeatedField<super::Transaction::PublicEntry>) {
        self.public_entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_public_entries(&mut self) -> &mut ::protobuf::RepeatedField<super::Transaction::PublicEntry> {
        &mut self.public_entries
    }

    // Take field
    pub fn take_public_entries(&mut self) -> ::protobuf::RepeatedField<super::Transaction::PublicEntry> {
        ::std::mem::replace(&mut self.public_entries, ::protobuf::RepeatedField::new())
    }

    // repeated .Catalyst.Protocol.Transaction.ConfidentialEntry confidential_entries = 6;


    pub fn get_confidential_entries(&self) -> &[super::Transaction::ConfidentialEntry] {
        &self.confidential_entries
    }
    pub fn clear_confidential_entries(&mut self) {
        self.confidential_entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_confidential_entries(&mut self, v: ::protobuf::RepeatedField<super::Transaction::ConfidentialEntry>) {
        self.confidential_entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_confidential_entries(&mut self) -> &mut ::protobuf::RepeatedField<super::Transaction::ConfidentialEntry> {
        &mut self.confidential_entries
    }

    // Take field
    pub fn take_confidential_entries(&mut self) -> ::protobuf::RepeatedField<super::Transaction::ConfidentialEntry> {
        ::std::mem::replace(&mut self.confidential_entries, ::protobuf::RepeatedField::new())
    }

    // repeated .Catalyst.Protocol.Transaction.CoinbaseEntry coinbase_entries = 7;


    pub fn get_coinbase_entries(&self) -> &[super::Transaction::CoinbaseEntry] {
        &self.coinbase_entries
    }
    pub fn clear_coinbase_entries(&mut self) {
        self.coinbase_entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_coinbase_entries(&mut self, v: ::protobuf::RepeatedField<super::Transaction::CoinbaseEntry>) {
        self.coinbase_entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_coinbase_entries(&mut self) -> &mut ::protobuf::RepeatedField<super::Transaction::CoinbaseEntry> {
        &mut self.coinbase_entries
    }

    // Take field
    pub fn take_coinbase_entries(&mut self) -> ::protobuf::RepeatedField<super::Transaction::CoinbaseEntry> {
        ::std::mem::replace(&mut self.coinbase_entries, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Delta {
    fn is_initialized(&self) -> bool {
        for v in &self.time_stamp {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.public_entries {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.confidential_entries {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.coinbase_entries {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.previous_delta_dfs_hash)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.merkle_root)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.merkle_poda)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.time_stamp)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.public_entries)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.confidential_entries)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.coinbase_entries)?;
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
        if !self.previous_delta_dfs_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.previous_delta_dfs_hash);
        }
        if !self.merkle_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.merkle_root);
        }
        if !self.merkle_poda.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.merkle_poda);
        }
        if let Some(ref v) = self.time_stamp.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.public_entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.confidential_entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.coinbase_entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.previous_delta_dfs_hash.is_empty() {
            os.write_bytes(1, &self.previous_delta_dfs_hash)?;
        }
        if !self.merkle_root.is_empty() {
            os.write_bytes(2, &self.merkle_root)?;
        }
        if !self.merkle_poda.is_empty() {
            os.write_bytes(3, &self.merkle_poda)?;
        }
        if let Some(ref v) = self.time_stamp.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.public_entries {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.confidential_entries {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.coinbase_entries {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> Delta {
        Delta::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "previous_delta_dfs_hash",
                    |m: &Delta| { &m.previous_delta_dfs_hash },
                    |m: &mut Delta| { &mut m.previous_delta_dfs_hash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "merkle_root",
                    |m: &Delta| { &m.merkle_root },
                    |m: &mut Delta| { &mut m.merkle_root },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "merkle_poda",
                    |m: &Delta| { &m.merkle_poda },
                    |m: &mut Delta| { &mut m.merkle_poda },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Timestamp>>(
                    "time_stamp",
                    |m: &Delta| { &m.time_stamp },
                    |m: &mut Delta| { &mut m.time_stamp },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Transaction::PublicEntry>>(
                    "public_entries",
                    |m: &Delta| { &m.public_entries },
                    |m: &mut Delta| { &mut m.public_entries },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Transaction::ConfidentialEntry>>(
                    "confidential_entries",
                    |m: &Delta| { &m.confidential_entries },
                    |m: &mut Delta| { &mut m.confidential_entries },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Transaction::CoinbaseEntry>>(
                    "coinbase_entries",
                    |m: &Delta| { &m.coinbase_entries },
                    |m: &mut Delta| { &mut m.coinbase_entries },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Delta>(
                    "Delta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static Delta {
        static mut instance: ::protobuf::lazy::Lazy<Delta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Delta,
        };
        unsafe {
            instance.get(Delta::new)
        }
    }
}

impl ::protobuf::Clear for Delta {
    fn clear(&mut self) {
        self.previous_delta_dfs_hash.clear();
        self.merkle_root.clear();
        self.merkle_poda.clear();
        self.time_stamp.clear();
        self.public_entries.clear();
        self.confidential_entries.clear();
        self.coinbase_entries.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Delta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Delta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cDeltas.proto\x12\x18Catalyst.Protocol.Deltas\x1a\x11Transaction.pr\
    oto\x1a\x1fgoogle/protobuf/timestamp.proto\"6\n\nDeltaIndex\x12\x16\n\
    \x06height\x18\x01\x20\x01(\rR\x06height\x12\x10\n\x03cid\x18\x02\x20\
    \x01(\x0cR\x03cid\"\xcc\x03\n\x05Delta\x125\n\x17previous_delta_dfs_hash\
    \x18\x01\x20\x01(\x0cR\x14previousDeltaDfsHash\x12\x1f\n\x0bmerkle_root\
    \x18\x02\x20\x01(\x0cR\nmerkleRoot\x12\x1f\n\x0bmerkle_poda\x18\x03\x20\
    \x01(\x0cR\nmerklePoda\x129\n\ntime_stamp\x18\x04\x20\x01(\x0b2\x1a.goog\
    le.protobuf.TimestampR\ttimeStamp\x12Q\n\x0epublic_entries\x18\x05\x20\
    \x03(\x0b2*.Catalyst.Protocol.Transaction.PublicEntryR\rpublicEntries\
    \x12c\n\x14confidential_entries\x18\x06\x20\x03(\x0b20.Catalyst.Protocol\
    .Transaction.ConfidentialEntryR\x13confidentialEntries\x12W\n\x10coinbas\
    e_entries\x18\x07\x20\x03(\x0b2,.Catalyst.Protocol.Transaction.CoinbaseE\
    ntryR\x0fcoinbaseEntriesB\x02P\x01J\x87\x0e\n\x06\x12\x04\x13\0)\x01\n\
    \xdf\x06\n\x01\x0c\x12\x03\x13\0\x122\xd4\x06*\n\x20Copyright\x20(c)\x20\
    2019\x20Catalyst\x20Network\n\n\x20This\x20file\x20is\x20part\x20of\x20C\
    atalyst.Network.Protocol.Protobuffs\x20<https://github.com/catalyst-netw\
    ork/protocol-protobuffs>\n\n\x20Catalyst.Network.Protocol.Protobuffs\x20\
    is\x20free\x20software:\x20you\x20can\x20redistribute\x20it\x20and/or\
    \x20modify\n\x20it\x20under\x20the\x20terms\x20of\x20the\x20GNU\x20Gener\
    al\x20Public\x20License\x20as\x20published\x20by\n\x20the\x20Free\x20Sof\
    tware\x20Foundation,\x20either\x20version\x202\x20of\x20the\x20License,\
    \x20or\n\x20(at\x20your\x20option)\x20any\x20later\x20version.\n\x20\n\
    \x20Catalyst.Network.Protocol.Protobuffs\x20is\x20distributed\x20in\x20t\
    he\x20hope\x20that\x20it\x20will\x20be\x20useful,\n\x20but\x20WITHOUT\
    \x20ANY\x20WARRANTY;\x20without\x20even\x20the\x20implied\x20warranty\
    \x20of\n\x20MERCHANTABILITY\x20or\x20FITNESS\x20FOR\x20A\x20PARTICULAR\
    \x20PURPOSE.\x20See\x20the\n\x20GNU\x20General\x20Public\x20License\x20f\
    or\x20more\x20details.\n\x20\n\x20You\x20should\x20have\x20received\x20a\
    \x20copy\x20of\x20the\x20GNU\x20General\x20Public\x20License\n\x20along\
    \x20with\x20Catalyst.Network.Protocol.Protobuffs\x20If\x20not,\x20see\
    \x20<https://www.gnu.org/licenses/>.\n\n\x08\n\x01\x08\x12\x03\x15\0\"\n\
    \x0b\n\x04\x08\xe7\x07\0\x12\x03\x15\0\"\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x15\x07\x1a\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x15\x07\x1a\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x15\x07\x1a\n\x0c\n\x05\x08\
    \xe7\x07\0\x03\x12\x03\x15\x1d!\n\t\n\x02\x03\0\x12\x03\x17\x07\x1a\n\t\
    \n\x02\x03\x01\x12\x03\x18\x07(\n\x08\n\x01\x02\x12\x03\x1a\x08\x20\n\n\
    \n\x02\x04\0\x12\x04\x1c\0\x1f\x01\n\n\n\x03\x04\0\x01\x12\x03\x1c\x08\
    \x12\n\x0b\n\x04\x04\0\x02\0\x12\x03\x1d\x08\x1a\n\r\n\x05\x04\0\x02\0\
    \x04\x12\x04\x1d\x08\x1c\x14\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x1d\x08\
    \x0e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1d\x0f\x15\n\x0c\n\x05\x04\0\
    \x02\0\x03\x12\x03\x1d\x18\x19\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x1e\x08\
    \x16\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x1e\x08\x1d\x1a\n\x0c\n\x05\x04\
    \0\x02\x01\x05\x12\x03\x1e\x08\r\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\
    \x1e\x0e\x11\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x1e\x14\x15\n\n\n\x02\
    \x04\x01\x12\x04!\0)\x01\n\n\n\x03\x04\x01\x01\x12\x03!\x08\r\nH\n\x04\
    \x04\x01\x02\0\x12\x03\"\x08*\";\x20\x20address\x20for\x20the\x20content\
    \x20of\x20the\x20previous\x20delta\x20on\x20the\x20DFS\n\n\r\n\x05\x04\
    \x01\x02\0\x04\x12\x04\"\x08!\x0f\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\
    \"\x08\r\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\"\x0e%\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\"()\n\x0b\n\x04\x04\x01\x02\x01\x12\x03#\x08\x1e\
    \n\r\n\x05\x04\x01\x02\x01\x04\x12\x04#\x08\"*\n\x0c\n\x05\x04\x01\x02\
    \x01\x05\x12\x03#\x08\r\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03#\x0e\x19\
    \n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03#\x1c\x1d\n=\n\x04\x04\x01\x02\
    \x02\x12\x03$\x08\x1e\"0\x20proof\x20of\x20delegated\x20authority\x20for\
    \x20active\x20wokers\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04$\x08#\x1e\n\
    \x0c\n\x05\x04\x01\x02\x02\x05\x12\x03$\x08\r\n\x0c\n\x05\x04\x01\x02\
    \x02\x01\x12\x03$\x0e\x19\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03$\x1c\
    \x1d\n\x0b\n\x04\x04\x01\x02\x03\x12\x03%\x081\n\r\n\x05\x04\x01\x02\x03\
    \x04\x12\x04%\x08$\x1e\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03%\x08!\n\
    \x0c\n\x05\x04\x01\x02\x03\x01\x12\x03%\",\n\x0c\n\x05\x04\x01\x02\x03\
    \x03\x12\x03%/0\n\x0b\n\x04\x04\x01\x02\x04\x12\x03&\x08<\n\x0c\n\x05\
    \x04\x01\x02\x04\x04\x12\x03&\x08\x10\n\x0c\n\x05\x04\x01\x02\x04\x06\
    \x12\x03&\x11(\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03&)7\n\x0c\n\x05\
    \x04\x01\x02\x04\x03\x12\x03&:;\n\x0b\n\x04\x04\x01\x02\x05\x12\x03'\x08\
    H\n\x0c\n\x05\x04\x01\x02\x05\x04\x12\x03'\x08\x10\n\x0c\n\x05\x04\x01\
    \x02\x05\x06\x12\x03'\x11.\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03'/C\n\
    \x0c\n\x05\x04\x01\x02\x05\x03\x12\x03'FG\n$\n\x04\x04\x01\x02\x06\x12\
    \x03(\x08@\"\x17\x20one\x20per\x20active\x20worker\n\n\x0c\n\x05\x04\x01\
    \x02\x06\x04\x12\x03(\x08\x10\n\x0c\n\x05\x04\x01\x02\x06\x06\x12\x03(\
    \x11*\n\x0c\n\x05\x04\x01\x02\x06\x01\x12\x03(+;\n\x0c\n\x05\x04\x01\x02\
    \x06\x03\x12\x03(>?b\x06proto3\
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
