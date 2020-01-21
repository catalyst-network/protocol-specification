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
//! Generated file from `DfsMarketplace.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_0;

#[derive(PartialEq,Clone,Default)]
pub struct BlockChallengeRequest {
    // message fields
    pub challenge_salt: ::std::string::String,
    pub main_file_cid: ::std::string::String,
    pub block_idx_random_guid: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BlockChallengeRequest {
    fn default() -> &'a BlockChallengeRequest {
        <BlockChallengeRequest as ::protobuf::Message>::default_instance()
    }
}

impl BlockChallengeRequest {
    pub fn new() -> BlockChallengeRequest {
        ::std::default::Default::default()
    }

    // string challenge_salt = 1;


    pub fn get_challenge_salt(&self) -> &str {
        &self.challenge_salt
    }
    pub fn clear_challenge_salt(&mut self) {
        self.challenge_salt.clear();
    }

    // Param is passed by value, moved
    pub fn set_challenge_salt(&mut self, v: ::std::string::String) {
        self.challenge_salt = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenge_salt(&mut self) -> &mut ::std::string::String {
        &mut self.challenge_salt
    }

    // Take field
    pub fn take_challenge_salt(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.challenge_salt, ::std::string::String::new())
    }

    // string main_file_cid = 2;


    pub fn get_main_file_cid(&self) -> &str {
        &self.main_file_cid
    }
    pub fn clear_main_file_cid(&mut self) {
        self.main_file_cid.clear();
    }

    // Param is passed by value, moved
    pub fn set_main_file_cid(&mut self, v: ::std::string::String) {
        self.main_file_cid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_main_file_cid(&mut self) -> &mut ::std::string::String {
        &mut self.main_file_cid
    }

    // Take field
    pub fn take_main_file_cid(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.main_file_cid, ::std::string::String::new())
    }

    // bytes block_idx_random_guid = 3;


    pub fn get_block_idx_random_guid(&self) -> &[u8] {
        &self.block_idx_random_guid
    }
    pub fn clear_block_idx_random_guid(&mut self) {
        self.block_idx_random_guid.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_idx_random_guid(&mut self, v: ::std::vec::Vec<u8>) {
        self.block_idx_random_guid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_idx_random_guid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.block_idx_random_guid
    }

    // Take field
    pub fn take_block_idx_random_guid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.block_idx_random_guid, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for BlockChallengeRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.challenge_salt)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.main_file_cid)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.block_idx_random_guid)?;
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
        if !self.challenge_salt.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.challenge_salt);
        }
        if !self.main_file_cid.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.main_file_cid);
        }
        if !self.block_idx_random_guid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.block_idx_random_guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.challenge_salt.is_empty() {
            os.write_string(1, &self.challenge_salt)?;
        }
        if !self.main_file_cid.is_empty() {
            os.write_string(2, &self.main_file_cid)?;
        }
        if !self.block_idx_random_guid.is_empty() {
            os.write_bytes(3, &self.block_idx_random_guid)?;
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

    fn new() -> BlockChallengeRequest {
        BlockChallengeRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "challenge_salt",
                    |m: &BlockChallengeRequest| { &m.challenge_salt },
                    |m: &mut BlockChallengeRequest| { &mut m.challenge_salt },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "main_file_cid",
                    |m: &BlockChallengeRequest| { &m.main_file_cid },
                    |m: &mut BlockChallengeRequest| { &mut m.main_file_cid },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "block_idx_random_guid",
                    |m: &BlockChallengeRequest| { &m.block_idx_random_guid },
                    |m: &mut BlockChallengeRequest| { &mut m.block_idx_random_guid },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockChallengeRequest>(
                    "BlockChallengeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static BlockChallengeRequest {
        static mut instance: ::protobuf::lazy::Lazy<BlockChallengeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockChallengeRequest,
        };
        unsafe {
            instance.get(BlockChallengeRequest::new)
        }
    }
}

impl ::protobuf::Clear for BlockChallengeRequest {
    fn clear(&mut self) {
        self.challenge_salt.clear();
        self.main_file_cid.clear();
        self.block_idx_random_guid.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockChallengeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockChallengeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockChallengeResponse {
    // message fields
    pub answer: ::std::string::String,
    pub block_challenge_request_hash: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BlockChallengeResponse {
    fn default() -> &'a BlockChallengeResponse {
        <BlockChallengeResponse as ::protobuf::Message>::default_instance()
    }
}

impl BlockChallengeResponse {
    pub fn new() -> BlockChallengeResponse {
        ::std::default::Default::default()
    }

    // string answer = 1;


    pub fn get_answer(&self) -> &str {
        &self.answer
    }
    pub fn clear_answer(&mut self) {
        self.answer.clear();
    }

    // Param is passed by value, moved
    pub fn set_answer(&mut self, v: ::std::string::String) {
        self.answer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_answer(&mut self) -> &mut ::std::string::String {
        &mut self.answer
    }

    // Take field
    pub fn take_answer(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.answer, ::std::string::String::new())
    }

    // string block_challenge_request_hash = 2;


    pub fn get_block_challenge_request_hash(&self) -> &str {
        &self.block_challenge_request_hash
    }
    pub fn clear_block_challenge_request_hash(&mut self) {
        self.block_challenge_request_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_block_challenge_request_hash(&mut self, v: ::std::string::String) {
        self.block_challenge_request_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block_challenge_request_hash(&mut self) -> &mut ::std::string::String {
        &mut self.block_challenge_request_hash
    }

    // Take field
    pub fn take_block_challenge_request_hash(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.block_challenge_request_hash, ::std::string::String::new())
    }
}

impl ::protobuf::Message for BlockChallengeResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.answer)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.block_challenge_request_hash)?;
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
        if !self.answer.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.answer);
        }
        if !self.block_challenge_request_hash.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.block_challenge_request_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.answer.is_empty() {
            os.write_string(1, &self.answer)?;
        }
        if !self.block_challenge_request_hash.is_empty() {
            os.write_string(2, &self.block_challenge_request_hash)?;
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

    fn new() -> BlockChallengeResponse {
        BlockChallengeResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "answer",
                    |m: &BlockChallengeResponse| { &m.answer },
                    |m: &mut BlockChallengeResponse| { &mut m.answer },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "block_challenge_request_hash",
                    |m: &BlockChallengeResponse| { &m.block_challenge_request_hash },
                    |m: &mut BlockChallengeResponse| { &mut m.block_challenge_request_hash },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockChallengeResponse>(
                    "BlockChallengeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static BlockChallengeResponse {
        static mut instance: ::protobuf::lazy::Lazy<BlockChallengeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockChallengeResponse,
        };
        unsafe {
            instance.get(BlockChallengeResponse::new)
        }
    }
}

impl ::protobuf::Clear for BlockChallengeResponse {
    fn clear(&mut self) {
        self.answer.clear();
        self.block_challenge_request_hash.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockChallengeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockChallengeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockChallengeBroadcast {
    // message fields
    pub original_challenge: ::protobuf::SingularPtrField<BlockChallengeRequest>,
    pub answer: ::std::string::String,
    pub challenged_peer: ::protobuf::SingularPtrField<super::Peer::PeerId>,
    pub challenged_by: ::protobuf::SingularPtrField<super::Peer::PeerId>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a BlockChallengeBroadcast {
    fn default() -> &'a BlockChallengeBroadcast {
        <BlockChallengeBroadcast as ::protobuf::Message>::default_instance()
    }
}

impl BlockChallengeBroadcast {
    pub fn new() -> BlockChallengeBroadcast {
        ::std::default::Default::default()
    }

    // .Catalyst.Protocol.DfsMarketplace.BlockChallengeRequest original_challenge = 1;


    pub fn get_original_challenge(&self) -> &BlockChallengeRequest {
        self.original_challenge.as_ref().unwrap_or_else(|| BlockChallengeRequest::default_instance())
    }
    pub fn clear_original_challenge(&mut self) {
        self.original_challenge.clear();
    }

    pub fn has_original_challenge(&self) -> bool {
        self.original_challenge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_original_challenge(&mut self, v: BlockChallengeRequest) {
        self.original_challenge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_original_challenge(&mut self) -> &mut BlockChallengeRequest {
        if self.original_challenge.is_none() {
            self.original_challenge.set_default();
        }
        self.original_challenge.as_mut().unwrap()
    }

    // Take field
    pub fn take_original_challenge(&mut self) -> BlockChallengeRequest {
        self.original_challenge.take().unwrap_or_else(|| BlockChallengeRequest::new())
    }

    // string answer = 2;


    pub fn get_answer(&self) -> &str {
        &self.answer
    }
    pub fn clear_answer(&mut self) {
        self.answer.clear();
    }

    // Param is passed by value, moved
    pub fn set_answer(&mut self, v: ::std::string::String) {
        self.answer = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_answer(&mut self) -> &mut ::std::string::String {
        &mut self.answer
    }

    // Take field
    pub fn take_answer(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.answer, ::std::string::String::new())
    }

    // .Catalyst.Protocol.Peer.PeerId challenged_peer = 3;


    pub fn get_challenged_peer(&self) -> &super::Peer::PeerId {
        self.challenged_peer.as_ref().unwrap_or_else(|| super::Peer::PeerId::default_instance())
    }
    pub fn clear_challenged_peer(&mut self) {
        self.challenged_peer.clear();
    }

    pub fn has_challenged_peer(&self) -> bool {
        self.challenged_peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenged_peer(&mut self, v: super::Peer::PeerId) {
        self.challenged_peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenged_peer(&mut self) -> &mut super::Peer::PeerId {
        if self.challenged_peer.is_none() {
            self.challenged_peer.set_default();
        }
        self.challenged_peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenged_peer(&mut self) -> super::Peer::PeerId {
        self.challenged_peer.take().unwrap_or_else(|| super::Peer::PeerId::new())
    }

    // .Catalyst.Protocol.Peer.PeerId challenged_by = 4;


    pub fn get_challenged_by(&self) -> &super::Peer::PeerId {
        self.challenged_by.as_ref().unwrap_or_else(|| super::Peer::PeerId::default_instance())
    }
    pub fn clear_challenged_by(&mut self) {
        self.challenged_by.clear();
    }

    pub fn has_challenged_by(&self) -> bool {
        self.challenged_by.is_some()
    }

    // Param is passed by value, moved
    pub fn set_challenged_by(&mut self, v: super::Peer::PeerId) {
        self.challenged_by = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_challenged_by(&mut self) -> &mut super::Peer::PeerId {
        if self.challenged_by.is_none() {
            self.challenged_by.set_default();
        }
        self.challenged_by.as_mut().unwrap()
    }

    // Take field
    pub fn take_challenged_by(&mut self) -> super::Peer::PeerId {
        self.challenged_by.take().unwrap_or_else(|| super::Peer::PeerId::new())
    }
}

impl ::protobuf::Message for BlockChallengeBroadcast {
    fn is_initialized(&self) -> bool {
        for v in &self.original_challenge {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.challenged_peer {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.challenged_by {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.original_challenge)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.answer)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.challenged_peer)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.challenged_by)?;
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
        if let Some(ref v) = self.original_challenge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.answer.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.answer);
        }
        if let Some(ref v) = self.challenged_peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.challenged_by.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.original_challenge.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.answer.is_empty() {
            os.write_string(2, &self.answer)?;
        }
        if let Some(ref v) = self.challenged_peer.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.challenged_by.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

    fn new() -> BlockChallengeBroadcast {
        BlockChallengeBroadcast::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockChallengeRequest>>(
                    "original_challenge",
                    |m: &BlockChallengeBroadcast| { &m.original_challenge },
                    |m: &mut BlockChallengeBroadcast| { &mut m.original_challenge },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "answer",
                    |m: &BlockChallengeBroadcast| { &m.answer },
                    |m: &mut BlockChallengeBroadcast| { &mut m.answer },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Peer::PeerId>>(
                    "challenged_peer",
                    |m: &BlockChallengeBroadcast| { &m.challenged_peer },
                    |m: &mut BlockChallengeBroadcast| { &mut m.challenged_peer },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::Peer::PeerId>>(
                    "challenged_by",
                    |m: &BlockChallengeBroadcast| { &m.challenged_by },
                    |m: &mut BlockChallengeBroadcast| { &mut m.challenged_by },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockChallengeBroadcast>(
                    "BlockChallengeBroadcast",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static BlockChallengeBroadcast {
        static mut instance: ::protobuf::lazy::Lazy<BlockChallengeBroadcast> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockChallengeBroadcast,
        };
        unsafe {
            instance.get(BlockChallengeBroadcast::new)
        }
    }
}

impl ::protobuf::Clear for BlockChallengeBroadcast {
    fn clear(&mut self) {
        self.original_challenge.clear();
        self.answer.clear();
        self.challenged_peer.clear();
        self.challenged_by.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockChallengeBroadcast {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockChallengeBroadcast {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14DfsMarketplace.proto\x12\x20Catalyst.Protocol.DfsMarketplace\x1a\n\
    Peer.proto\"\x95\x01\n\x15BlockChallengeRequest\x12%\n\x0echallenge_salt\
    \x18\x01\x20\x01(\tR\rchallengeSalt\x12\"\n\rmain_file_cid\x18\x02\x20\
    \x01(\tR\x0bmainFileCid\x121\n\x15block_idx_random_guid\x18\x03\x20\x01(\
    \x0cR\x12blockIdxRandomGuid\"q\n\x16BlockChallengeResponse\x12\x16\n\x06\
    answer\x18\x01\x20\x01(\tR\x06answer\x12?\n\x1cblock_challenge_request_h\
    ash\x18\x02\x20\x01(\tR\x19blockChallengeRequestHash\"\xa7\x02\n\x17Bloc\
    kChallengeBroadcast\x12f\n\x12original_challenge\x18\x01\x20\x01(\x0b27.\
    Catalyst.Protocol.DfsMarketplace.BlockChallengeRequestR\x11originalChall\
    enge\x12\x16\n\x06answer\x18\x02\x20\x01(\tR\x06answer\x12G\n\x0fchallen\
    ged_peer\x18\x03\x20\x01(\x0b2\x1e.Catalyst.Protocol.Peer.PeerIdR\x0echa\
    llengedPeer\x12C\n\rchallenged_by\x18\x04\x20\x01(\x0b2\x1e.Catalyst.Pro\
    tocol.Peer.PeerIdR\x0cchallengedByB\x02P\x01J\x8f\r\n\x06\x12\x04\x13\0+\
    \x01\n\xdf\x06\n\x01\x0c\x12\x03\x13\0\x122\xd4\x06*\n\x20Copyright\x20(\
    c)\x202019\x20Catalyst\x20Network\n\n\x20This\x20file\x20is\x20part\x20o\
    f\x20Catalyst.Network.Protocol.Protobuffs\x20<https://github.com/catalys\
    t-network/protocol-protobuffs>\n\n\x20Catalyst.Network.Protocol.Protobuf\
    fs\x20is\x20free\x20software:\x20you\x20can\x20redistribute\x20it\x20and\
    /or\x20modify\n\x20it\x20under\x20the\x20terms\x20of\x20the\x20GNU\x20Ge\
    neral\x20Public\x20License\x20as\x20published\x20by\n\x20the\x20Free\x20\
    Software\x20Foundation,\x20either\x20version\x202\x20of\x20the\x20Licens\
    e,\x20or\n\x20(at\x20your\x20option)\x20any\x20later\x20version.\n\x20\n\
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
    \xe7\x07\0\x03\x12\x03\x15\x1d!\n\x08\n\x01\x02\x12\x03\x17\x08(\n\t\n\
    \x02\x03\0\x12\x03\x19\x07\x13\n\n\n\x02\x04\0\x12\x04\x1b\0\x1f\x01\n\n\
    \n\x03\x04\0\x01\x12\x03\x1b\x08\x1d\n\x0b\n\x04\x04\0\x02\0\x12\x03\x1c\
    \x08\"\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x1c\x08\x1b\x1f\n\x0c\n\x05\x04\
    \0\x02\0\x05\x12\x03\x1c\x08\x0e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1c\
    \x0f\x1d\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x1c\x20!\n\x0b\n\x04\x04\0\
    \x02\x01\x12\x03\x1d\x08!\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x1d\x08\
    \x1c\"\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x1d\x08\x0e\n\x0c\n\x05\x04\
    \0\x02\x01\x01\x12\x03\x1d\x0f\x1c\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\
    \x1d\x1f\x20\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x1e\x08(\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x04\x1e\x08\x1d!\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\
    \x1e\x08\r\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1e\x0e#\n\x0c\n\x05\
    \x04\0\x02\x02\x03\x12\x03\x1e&'\n\n\n\x02\x04\x01\x12\x04!\0$\x01\n\n\n\
    \x03\x04\x01\x01\x12\x03!\x08\x1e\n\x0b\n\x04\x04\x01\x02\0\x12\x03\"\
    \x08\x1a\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\"\x08!\x20\n\x0c\n\x05\x04\
    \x01\x02\0\x05\x12\x03\"\x08\x0e\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\"\
    \x0f\x15\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\"\x18\x19\n\x0b\n\x04\x04\
    \x01\x02\x01\x12\x03#\x080\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04#\x08\"\
    \x1a\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03#\x08\x0e\n\x0c\n\x05\x04\
    \x01\x02\x01\x01\x12\x03#\x0f+\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03#.\
    /\n\n\n\x02\x04\x02\x12\x04&\0+\x01\n\n\n\x03\x04\x02\x01\x12\x03&\x08\
    \x1f\n\x0b\n\x04\x04\x02\x02\0\x12\x03'\x085\n\r\n\x05\x04\x02\x02\0\x04\
    \x12\x04'\x08&!\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03'\x08\x1d\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03'\x1e0\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03'34\n\x0b\n\x04\x04\x02\x02\x01\x12\x03(\x08\x1a\n\r\n\x05\x04\x02\
    \x02\x01\x04\x12\x04(\x08'5\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03(\x08\
    \x0e\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03(\x0f\x15\n\x0c\n\x05\x04\
    \x02\x02\x01\x03\x12\x03(\x18\x19\n\x0b\n\x04\x04\x02\x02\x02\x12\x03)\
    \x08(\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04)\x08(\x1a\n\x0c\n\x05\x04\
    \x02\x02\x02\x06\x12\x03)\x08\x13\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\
    \x03)\x14#\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03)&'\n\x0b\n\x04\x04\
    \x02\x02\x03\x12\x03*\x08&\n\r\n\x05\x04\x02\x02\x03\x04\x12\x04*\x08)(\
    \n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03*\x08\x13\n\x0c\n\x05\x04\x02\
    \x02\x03\x01\x12\x03*\x14!\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\x03*$%b\
    \x06proto3\
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
