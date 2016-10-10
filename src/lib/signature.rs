use lib::time::*;

pub struct GitSignature {
    name: String,
    email: String,
    when: GitTime,
}

