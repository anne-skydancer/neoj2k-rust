// SPDX-License-Identifier: BSD-2-Clause
#![forbid(unsafe_code)]
use neo_j2k::{DecodeParams, Decoder, DecoderConfig};

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    assert_eq!(args.len(), 3, "usage: kdu-pixel-reference INPUT.j2k OUTPUT.raw");
    let data = std::fs::read(&args[1]).expect("input");
    let mut decoder = Decoder::new(DecoderConfig::default());
    let image = decoder.decode(&data, DecodeParams::default()).expect("decode");
    assert!(!image.truncated);
    let first = image.components[0].info();
    for component in image.components {
        let info = component.info();
        assert_eq!((info.width, info.height), (first.width, first.height));
        assert_eq!((info.precision, info.signed), (8, false));
    }
    let mut pixels = Vec::new();
    for pixel in 0..(first.width as usize * first.height as usize) {
        for component in image.components {
            pixels.push(u8::try_from(component.samples()[pixel]).expect("8-bit sample"));
        }
    }
    std::fs::write(&args[2], pixels).expect("output");
    println!(
        "{{\"width\":{},\"height\":{},\"components\":{}}}",
        first.width,
        first.height,
        image.components.len()
    );
}
