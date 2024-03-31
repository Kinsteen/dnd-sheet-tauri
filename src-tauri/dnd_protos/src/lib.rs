pub mod protos {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));

    pub mod classes {
        include!(concat!(env!("OUT_DIR"), "/proto.classes.rs"));
    }
}

pub mod proto_helpers;
