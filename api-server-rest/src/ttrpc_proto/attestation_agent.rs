// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by pure
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `attestation_agent.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:attestation_agent.GetEvidenceRequest)
pub struct GetEvidenceRequest {
    // message fields
    // @@protoc_insertion_point(field:attestation_agent.GetEvidenceRequest.RuntimeData)
    pub RuntimeData: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:attestation_agent.GetEvidenceRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetEvidenceRequest {
    fn default() -> &'a GetEvidenceRequest {
        <GetEvidenceRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetEvidenceRequest {
    pub fn new() -> GetEvidenceRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "RuntimeData",
            |m: &GetEvidenceRequest| { &m.RuntimeData },
            |m: &mut GetEvidenceRequest| { &mut m.RuntimeData },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetEvidenceRequest>(
            "GetEvidenceRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetEvidenceRequest {
    const NAME: &'static str = "GetEvidenceRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.RuntimeData = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.RuntimeData.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.RuntimeData);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.RuntimeData.is_empty() {
            os.write_bytes(1, &self.RuntimeData)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetEvidenceRequest {
        GetEvidenceRequest::new()
    }

    fn clear(&mut self) {
        self.RuntimeData.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetEvidenceRequest {
        static instance: GetEvidenceRequest = GetEvidenceRequest {
            RuntimeData: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetEvidenceRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetEvidenceRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetEvidenceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEvidenceRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:attestation_agent.GetEvidenceResponse)
pub struct GetEvidenceResponse {
    // message fields
    // @@protoc_insertion_point(field:attestation_agent.GetEvidenceResponse.Evidence)
    pub Evidence: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:attestation_agent.GetEvidenceResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetEvidenceResponse {
    fn default() -> &'a GetEvidenceResponse {
        <GetEvidenceResponse as ::protobuf::Message>::default_instance()
    }
}

impl GetEvidenceResponse {
    pub fn new() -> GetEvidenceResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Evidence",
            |m: &GetEvidenceResponse| { &m.Evidence },
            |m: &mut GetEvidenceResponse| { &mut m.Evidence },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetEvidenceResponse>(
            "GetEvidenceResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetEvidenceResponse {
    const NAME: &'static str = "GetEvidenceResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.Evidence = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.Evidence.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.Evidence);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.Evidence.is_empty() {
            os.write_bytes(1, &self.Evidence)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetEvidenceResponse {
        GetEvidenceResponse::new()
    }

    fn clear(&mut self) {
        self.Evidence.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetEvidenceResponse {
        static instance: GetEvidenceResponse = GetEvidenceResponse {
            Evidence: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetEvidenceResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetEvidenceResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetEvidenceResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetEvidenceResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:attestation_agent.GetTokenRequest)
pub struct GetTokenRequest {
    // message fields
    // @@protoc_insertion_point(field:attestation_agent.GetTokenRequest.TokenType)
    pub TokenType: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:attestation_agent.GetTokenRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetTokenRequest {
    fn default() -> &'a GetTokenRequest {
        <GetTokenRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetTokenRequest {
    pub fn new() -> GetTokenRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "TokenType",
            |m: &GetTokenRequest| { &m.TokenType },
            |m: &mut GetTokenRequest| { &mut m.TokenType },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetTokenRequest>(
            "GetTokenRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetTokenRequest {
    const NAME: &'static str = "GetTokenRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.TokenType = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.TokenType.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.TokenType);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.TokenType.is_empty() {
            os.write_string(1, &self.TokenType)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetTokenRequest {
        GetTokenRequest::new()
    }

    fn clear(&mut self) {
        self.TokenType.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetTokenRequest {
        static instance: GetTokenRequest = GetTokenRequest {
            TokenType: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetTokenRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetTokenRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetTokenRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTokenRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:attestation_agent.GetTokenResponse)
pub struct GetTokenResponse {
    // message fields
    // @@protoc_insertion_point(field:attestation_agent.GetTokenResponse.Token)
    pub Token: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:attestation_agent.GetTokenResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetTokenResponse {
    fn default() -> &'a GetTokenResponse {
        <GetTokenResponse as ::protobuf::Message>::default_instance()
    }
}

impl GetTokenResponse {
    pub fn new() -> GetTokenResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Token",
            |m: &GetTokenResponse| { &m.Token },
            |m: &mut GetTokenResponse| { &mut m.Token },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetTokenResponse>(
            "GetTokenResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetTokenResponse {
    const NAME: &'static str = "GetTokenResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.Token = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.Token.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.Token);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.Token.is_empty() {
            os.write_bytes(1, &self.Token)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetTokenResponse {
        GetTokenResponse::new()
    }

    fn clear(&mut self) {
        self.Token.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetTokenResponse {
        static instance: GetTokenResponse = GetTokenResponse {
            Token: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetTokenResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetTokenResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetTokenResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetTokenResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17attestation_agent.proto\x12\x11attestation_agent\"6\n\x12GetEviden\
    ceRequest\x12\x20\n\x0bRuntimeData\x18\x01\x20\x01(\x0cR\x0bRuntimeData\
    \"1\n\x13GetEvidenceResponse\x12\x1a\n\x08Evidence\x18\x01\x20\x01(\x0cR\
    \x08Evidence\"/\n\x0fGetTokenRequest\x12\x1c\n\tTokenType\x18\x01\x20\
    \x01(\tR\tTokenType\"(\n\x10GetTokenResponse\x12\x14\n\x05Token\x18\x01\
    \x20\x01(\x0cR\x05Token2\xcc\x01\n\x17AttestationAgentService\x12\\\n\
    \x0bGetEvidence\x12%.attestation_agent.GetEvidenceRequest\x1a&.attestati\
    on_agent.GetEvidenceResponse\x12S\n\x08GetToken\x12\".attestation_agent.\
    GetTokenRequest\x1a#.attestation_agent.GetTokenResponseb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(GetEvidenceRequest::generated_message_descriptor_data());
            messages.push(GetEvidenceResponse::generated_message_descriptor_data());
            messages.push(GetTokenRequest::generated_message_descriptor_data());
            messages.push(GetTokenResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}