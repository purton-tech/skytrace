pub mod google {
    pub mod protobuf {
        #![allow(clippy::derive_partial_eq_without_eq)]
        tonic::include_proto!("google.protobuf");
    }
}

pub mod ccsds {
    pub mod schema {
        #![allow(clippy::derive_partial_eq_without_eq)]
        tonic::include_proto!("ccsds.schema");
    }
}

pub mod validate {
    #![allow(clippy::derive_partial_eq_without_eq)]
    #![allow(clippy::len_without_is_empty)]
    tonic::include_proto!("validate");
}

pub mod trace {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("trace");
}
