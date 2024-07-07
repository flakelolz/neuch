use crate::prelude::*;

pub fn normals(buffer: &InputBuffer) -> bool {
    if cr_light_punch(buffer) {
        return true;
    }
    if st_light_punch(buffer) {
        return true;
    }

    false
}

pub fn chains(buffer: &InputBuffer) -> bool {
    if cr_light_punch(buffer) {
        return true;
    }
    if st_light_punch(buffer) {
        return true;
    }
    false
}

pub fn neutral(buffer: &InputBuffer) -> bool {
    !buffer.input().up
        && !buffer.input().down
        && !buffer.input().forward
        && !buffer.input().backward
}

pub fn down(buffer: &InputBuffer) -> bool {
    buffer.input().down
}

pub fn st_light_punch(buffer: &InputBuffer) -> bool {
    buffer.buffered(&Inputs::LightPunch, buffer.attack)
}

pub fn cr_light_punch(buffer: &InputBuffer) -> bool {
    buffer.buffered(&Inputs::LightPunch, buffer.attack) && buffer.input().down
}
