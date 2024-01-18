use proto_macros::bolt_struct_derive;
use proto_common::marker::{MARKER_TINY_STRUCT_BASE, SIGNATURE_NODE};

#[bolt_struct_derive]
struct Demo {
    pb: i64
}

impl Demo {
    fn hell(&self) -> i64 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::demo::Demo;

    #[test]
    fn test_1() {
        let a = Demo{pb: 1};
        println!("{:?}", a.get_hell())
    }
}