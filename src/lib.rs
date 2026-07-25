//! Official engram capability `uuid`. Provides the guest with fresh random
//! UUIDs. Exports the `engram:cap-uuid/provider` interface; imports are the
//! allowlisted WASI interfaces declared in `wit/world.wit`.

#[allow(warnings)]
mod bindings;

use bindings::exports::engram::cap_uuid::provider::Guest;
use bindings::wasi::random::random;

struct Component;

impl Guest for Component {
    fn v4() -> String {
        // RFC 9562 section 5.4: 122 random bits, with the version nibble and
        // the two variant bits overwritten. `get-random-bytes` is the CSPRNG
        // WASI guarantees; formatting it by hand keeps this component
        // dependency-free.
        let mut bytes = <[u8; 16]>::try_from(random::get_random_bytes(16).as_slice())
            .expect("wasi:random returns the requested length");
        bytes[6] = (bytes[6] & 0x0f) | 0x40; // version 4
        bytes[8] = (bytes[8] & 0x3f) | 0x80; // variant 10xx

        let mut out = String::with_capacity(36);
        for (i, byte) in bytes.iter().enumerate() {
            if matches!(i, 4 | 6 | 8 | 10) {
                out.push('-');
            }
            out.push(HEX[(byte >> 4) as usize]);
            out.push(HEX[(byte & 0x0f) as usize]);
        }
        out
    }
}

const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

bindings::export!(Component with_types_in bindings);
