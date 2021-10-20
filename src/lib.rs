#[macro_use]
extern crate num_derive;

include!(concat!(env!("OUT_DIR"), "/proto.rs"));
include!(concat!("..", "/gen", "/packet_id.rs"));
