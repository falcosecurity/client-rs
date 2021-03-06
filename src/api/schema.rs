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
//! Generated file from `schema.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_1;

// Note: you cannot use pattern matching for enums with allow_alias option
#[derive(Clone,Eq,Debug)]
pub enum priority {
    EMERGENCY, // 0
    emergency, // 0
    Emergency, // 0
    ALERT, // 1
    alert, // 1
    Alert, // 1
    CRITICAL, // 2
    critical, // 2
    Critical, // 2
    ERROR, // 3
    error, // 3
    Error, // 3
    WARNING, // 4
    warning, // 4
    Warning, // 4
    NOTICE, // 5
    notice, // 5
    Notice, // 5
    INFORMATIONAL, // 6
    informational, // 6
    Informational, // 6
    DEBUG, // 7
    debug, // 7
    Debug, // 7
}

impl ::std::cmp::PartialEq for priority {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl ::std::hash::Hash for priority {
    fn hash<H : ::std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.value())
    }
}

impl ::protobuf::ProtobufEnum for priority {
    fn value(&self) -> i32 {
        match *self {
            priority::EMERGENCY => 0,
            priority::emergency => 0,
            priority::Emergency => 0,
            priority::ALERT => 1,
            priority::alert => 1,
            priority::Alert => 1,
            priority::CRITICAL => 2,
            priority::critical => 2,
            priority::Critical => 2,
            priority::ERROR => 3,
            priority::error => 3,
            priority::Error => 3,
            priority::WARNING => 4,
            priority::warning => 4,
            priority::Warning => 4,
            priority::NOTICE => 5,
            priority::notice => 5,
            priority::Notice => 5,
            priority::INFORMATIONAL => 6,
            priority::informational => 6,
            priority::Informational => 6,
            priority::DEBUG => 7,
            priority::debug => 7,
            priority::Debug => 7,
        }
    }

    fn from_i32(value: i32) -> ::std::option::Option<priority> {
        match value {
            0 => ::std::option::Option::Some(priority::EMERGENCY),
            1 => ::std::option::Option::Some(priority::ALERT),
            2 => ::std::option::Option::Some(priority::CRITICAL),
            3 => ::std::option::Option::Some(priority::ERROR),
            4 => ::std::option::Option::Some(priority::WARNING),
            5 => ::std::option::Option::Some(priority::NOTICE),
            6 => ::std::option::Option::Some(priority::INFORMATIONAL),
            7 => ::std::option::Option::Some(priority::DEBUG),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [priority] = &[
            priority::EMERGENCY,
            priority::emergency,
            priority::Emergency,
            priority::ALERT,
            priority::alert,
            priority::Alert,
            priority::CRITICAL,
            priority::critical,
            priority::Critical,
            priority::ERROR,
            priority::error,
            priority::Error,
            priority::WARNING,
            priority::warning,
            priority::Warning,
            priority::NOTICE,
            priority::notice,
            priority::Notice,
            priority::INFORMATIONAL,
            priority::informational,
            priority::Informational,
            priority::DEBUG,
            priority::debug,
            priority::Debug,
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
                ::protobuf::reflect::EnumDescriptor::new("priority", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for priority {
}

impl ::std::default::Default for priority {
    fn default() -> Self {
        priority::EMERGENCY
    }
}

impl ::protobuf::reflect::ProtobufValue for priority {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

// Note: you cannot use pattern matching for enums with allow_alias option
#[derive(Clone,Eq,Debug)]
pub enum source {
    SYSCALL, // 0
    syscall, // 0
    Syscall, // 0
    K8S_AUDIT, // 1
    k8s_audit, // 1
    K8s_audit, // 1
    K8S_audit, // 1
}

impl ::std::cmp::PartialEq for source {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl ::std::hash::Hash for source {
    fn hash<H : ::std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.value())
    }
}

impl ::protobuf::ProtobufEnum for source {
    fn value(&self) -> i32 {
        match *self {
            source::SYSCALL => 0,
            source::syscall => 0,
            source::Syscall => 0,
            source::K8S_AUDIT => 1,
            source::k8s_audit => 1,
            source::K8s_audit => 1,
            source::K8S_audit => 1,
        }
    }

    fn from_i32(value: i32) -> ::std::option::Option<source> {
        match value {
            0 => ::std::option::Option::Some(source::SYSCALL),
            1 => ::std::option::Option::Some(source::K8S_AUDIT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [source] = &[
            source::SYSCALL,
            source::syscall,
            source::Syscall,
            source::K8S_AUDIT,
            source::k8s_audit,
            source::K8s_audit,
            source::K8S_audit,
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
                ::protobuf::reflect::EnumDescriptor::new("source", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for source {
}

impl ::std::default::Default for source {
    fn default() -> Self {
        source::SYSCALL
    }
}

impl ::protobuf::reflect::ProtobufValue for source {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cschema.proto\x12\x0cfalco.schema*\xcc\x02\n\x08priority\x12\r\n\tE\
    MERGENCY\x10\0\x12\r\n\temergency\x10\0\x12\r\n\tEmergency\x10\0\x12\t\n\
    \x05ALERT\x10\x01\x12\t\n\x05alert\x10\x01\x12\t\n\x05Alert\x10\x01\x12\
    \x0c\n\x08CRITICAL\x10\x02\x12\x0c\n\x08critical\x10\x02\x12\x0c\n\x08Cr\
    itical\x10\x02\x12\t\n\x05ERROR\x10\x03\x12\t\n\x05error\x10\x03\x12\t\n\
    \x05Error\x10\x03\x12\x0b\n\x07WARNING\x10\x04\x12\x0b\n\x07warning\x10\
    \x04\x12\x0b\n\x07Warning\x10\x04\x12\n\n\x06NOTICE\x10\x05\x12\n\n\x06n\
    otice\x10\x05\x12\n\n\x06Notice\x10\x05\x12\x11\n\rINFORMATIONAL\x10\x06\
    \x12\x11\n\rinformational\x10\x06\x12\x11\n\rInformational\x10\x06\x12\t\
    \n\x05DEBUG\x10\x07\x12\t\n\x05debug\x10\x07\x12\t\n\x05Debug\x10\x07\
    \x1a\x02\x10\x01*o\n\x06source\x12\x0b\n\x07SYSCALL\x10\0\x12\x0b\n\x07s\
    yscall\x10\0\x12\x0b\n\x07Syscall\x10\0\x12\r\n\tK8S_AUDIT\x10\x01\x12\r\
    \n\tk8s_audit\x10\x01\x12\r\n\tK8s_audit\x10\x01\x12\r\n\tK8S_audit\x10\
    \x01\x1a\x02\x10\x01J\xf5\n\n\x06\x12\x04\0\0*\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\0\x15\n\n\n\x02\x05\0\x12\x04\
    \x05\0\x1f\x01\n\n\n\x03\x05\0\x01\x12\x03\x05\x05\r\n\n\n\x03\x05\0\x03\
    \x12\x03\x06\x02\x1c\n\x0b\n\x04\x05\0\x03\x02\x12\x03\x06\x02\x1c\n\x0b\
    \n\x04\x05\0\x02\0\x12\x03\x07\x02\x10\n\x0c\n\x05\x05\0\x02\0\x01\x12\
    \x03\x07\x02\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x07\x0e\x0f\n\x0b\n\
    \x04\x05\0\x02\x01\x12\x03\x08\x02\x10\n\x0c\n\x05\x05\0\x02\x01\x01\x12\
    \x03\x08\x02\x0b\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x08\x0e\x0f\n\x0b\
    \n\x04\x05\0\x02\x02\x12\x03\t\x02\x10\n\x0c\n\x05\x05\0\x02\x02\x01\x12\
    \x03\t\x02\x0b\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\t\x0e\x0f\n\x0b\n\
    \x04\x05\0\x02\x03\x12\x03\n\x02\x0c\n\x0c\n\x05\x05\0\x02\x03\x01\x12\
    \x03\n\x02\x07\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\n\n\x0b\n\x0b\n\x04\
    \x05\0\x02\x04\x12\x03\x0b\x02\x0c\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\
    \x0b\x02\x07\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x0b\n\x0b\n\x0b\n\x04\
    \x05\0\x02\x05\x12\x03\x0c\x02\x0c\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03\
    \x0c\x02\x07\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x0c\n\x0b\n\x0b\n\x04\
    \x05\0\x02\x06\x12\x03\r\x02\x0f\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\r\
    \x02\n\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\r\r\x0e\n\x0b\n\x04\x05\0\
    \x02\x07\x12\x03\x0e\x02\x0f\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x0e\
    \x02\n\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\x0e\r\x0e\n\x0b\n\x04\x05\0\
    \x02\x08\x12\x03\x0f\x02\x0f\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x0f\
    \x02\n\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03\x0f\r\x0e\n\x0b\n\x04\x05\0\
    \x02\t\x12\x03\x10\x02\x0c\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\x10\x02\
    \x07\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\x10\n\x0b\n\x0b\n\x04\x05\0\x02\
    \n\x12\x03\x11\x02\x0c\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\x11\x02\x07\n\
    \x0c\n\x05\x05\0\x02\n\x02\x12\x03\x11\n\x0b\n\x0b\n\x04\x05\0\x02\x0b\
    \x12\x03\x12\x02\x0c\n\x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x12\x02\x07\n\
    \x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\x12\n\x0b\n\x0b\n\x04\x05\0\x02\x0c\
    \x12\x03\x13\x02\x0e\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03\x13\x02\t\n\
    \x0c\n\x05\x05\0\x02\x0c\x02\x12\x03\x13\x0c\r\n\x0b\n\x04\x05\0\x02\r\
    \x12\x03\x14\x02\x0e\n\x0c\n\x05\x05\0\x02\r\x01\x12\x03\x14\x02\t\n\x0c\
    \n\x05\x05\0\x02\r\x02\x12\x03\x14\x0c\r\n\x0b\n\x04\x05\0\x02\x0e\x12\
    \x03\x15\x02\x0e\n\x0c\n\x05\x05\0\x02\x0e\x01\x12\x03\x15\x02\t\n\x0c\n\
    \x05\x05\0\x02\x0e\x02\x12\x03\x15\x0c\r\n\x0b\n\x04\x05\0\x02\x0f\x12\
    \x03\x16\x02\r\n\x0c\n\x05\x05\0\x02\x0f\x01\x12\x03\x16\x02\x08\n\x0c\n\
    \x05\x05\0\x02\x0f\x02\x12\x03\x16\x0b\x0c\n\x0b\n\x04\x05\0\x02\x10\x12\
    \x03\x17\x02\r\n\x0c\n\x05\x05\0\x02\x10\x01\x12\x03\x17\x02\x08\n\x0c\n\
    \x05\x05\0\x02\x10\x02\x12\x03\x17\x0b\x0c\n\x0b\n\x04\x05\0\x02\x11\x12\
    \x03\x18\x02\r\n\x0c\n\x05\x05\0\x02\x11\x01\x12\x03\x18\x02\x08\n\x0c\n\
    \x05\x05\0\x02\x11\x02\x12\x03\x18\x0b\x0c\n\x0b\n\x04\x05\0\x02\x12\x12\
    \x03\x19\x02\x14\n\x0c\n\x05\x05\0\x02\x12\x01\x12\x03\x19\x02\x0f\n\x0c\
    \n\x05\x05\0\x02\x12\x02\x12\x03\x19\x12\x13\n\x0b\n\x04\x05\0\x02\x13\
    \x12\x03\x1a\x02\x14\n\x0c\n\x05\x05\0\x02\x13\x01\x12\x03\x1a\x02\x0f\n\
    \x0c\n\x05\x05\0\x02\x13\x02\x12\x03\x1a\x12\x13\n\x0b\n\x04\x05\0\x02\
    \x14\x12\x03\x1b\x02\x14\n\x0c\n\x05\x05\0\x02\x14\x01\x12\x03\x1b\x02\
    \x0f\n\x0c\n\x05\x05\0\x02\x14\x02\x12\x03\x1b\x12\x13\n\x0b\n\x04\x05\0\
    \x02\x15\x12\x03\x1c\x02\x0c\n\x0c\n\x05\x05\0\x02\x15\x01\x12\x03\x1c\
    \x02\x07\n\x0c\n\x05\x05\0\x02\x15\x02\x12\x03\x1c\n\x0b\n\x0b\n\x04\x05\
    \0\x02\x16\x12\x03\x1d\x02\x0c\n\x0c\n\x05\x05\0\x02\x16\x01\x12\x03\x1d\
    \x02\x07\n\x0c\n\x05\x05\0\x02\x16\x02\x12\x03\x1d\n\x0b\n\x0b\n\x04\x05\
    \0\x02\x17\x12\x03\x1e\x02\x0c\n\x0c\n\x05\x05\0\x02\x17\x01\x12\x03\x1e\
    \x02\x07\n\x0c\n\x05\x05\0\x02\x17\x02\x12\x03\x1e\n\x0b\n\n\n\x02\x05\
    \x01\x12\x04!\0*\x01\n\n\n\x03\x05\x01\x01\x12\x03!\x05\x0b\n\n\n\x03\
    \x05\x01\x03\x12\x03\"\x02\x1c\n\x0b\n\x04\x05\x01\x03\x02\x12\x03\"\x02\
    \x1c\n\x0b\n\x04\x05\x01\x02\0\x12\x03#\x02\x0e\n\x0c\n\x05\x05\x01\x02\
    \0\x01\x12\x03#\x02\t\n\x0c\n\x05\x05\x01\x02\0\x02\x12\x03#\x0c\r\n\x0b\
    \n\x04\x05\x01\x02\x01\x12\x03$\x02\x0e\n\x0c\n\x05\x05\x01\x02\x01\x01\
    \x12\x03$\x02\t\n\x0c\n\x05\x05\x01\x02\x01\x02\x12\x03$\x0c\r\n\x0b\n\
    \x04\x05\x01\x02\x02\x12\x03%\x02\x0e\n\x0c\n\x05\x05\x01\x02\x02\x01\
    \x12\x03%\x02\t\n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03%\x0c\r\n\x0b\n\
    \x04\x05\x01\x02\x03\x12\x03&\x02\x10\n\x0c\n\x05\x05\x01\x02\x03\x01\
    \x12\x03&\x02\x0b\n\x0c\n\x05\x05\x01\x02\x03\x02\x12\x03&\x0e\x0f\n\x0b\
    \n\x04\x05\x01\x02\x04\x12\x03'\x02\x10\n\x0c\n\x05\x05\x01\x02\x04\x01\
    \x12\x03'\x02\x0b\n\x0c\n\x05\x05\x01\x02\x04\x02\x12\x03'\x0e\x0f\n\x0b\
    \n\x04\x05\x01\x02\x05\x12\x03(\x02\x10\n\x0c\n\x05\x05\x01\x02\x05\x01\
    \x12\x03(\x02\x0b\n\x0c\n\x05\x05\x01\x02\x05\x02\x12\x03(\x0e\x0f\n\x0b\
    \n\x04\x05\x01\x02\x06\x12\x03)\x02\x10\n\x0c\n\x05\x05\x01\x02\x06\x01\
    \x12\x03)\x02\x0b\n\x0c\n\x05\x05\x01\x02\x06\x02\x12\x03)\x0e\x0fb\x06p\
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
