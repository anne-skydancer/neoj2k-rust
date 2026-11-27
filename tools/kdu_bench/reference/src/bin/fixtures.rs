// SPDX-License-Identifier: BSD-2-Clause
#![forbid(unsafe_code)]
use neo_j2k::{EncodeParams, Encoder, ImageSamples, ImageView};
fn main() {
    let directory = std::path::PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    std::fs::create_dir_all(&directory).unwrap();
    for side in [32usize, 128, 512, 1024] {
        let pixels: Vec<u8> = (0..side * side * 3)
            .map(|i| {
                let (x, y, c) = ((i / 3) % side, (i / 3) / side, i % 3);
                (3 * x + 5 * y + 13 * (x ^ y) + 23 * c) as u8
            })
            .collect();
        let view = ImageView {
            width: side as u32,
            height: side as u32,
            flip_vertical: false,
            samples: ImageSamples::Interleaved {
                samples: &pixels,
                components: 3,
                row_stride: side * 3,
            },
        };
        let mut params = EncodeParams::default();
        params.reversible = true;
        params.mct = true;
        params.resolutions = Some(6);
        let data = Encoder::new().encode(&view, &params).unwrap().to_vec();
        let name = format!("rgb_{side}_53");
        std::fs::write(directory.join(format!("{name}.j2k")), data).unwrap();
        std::fs::write(directory.join(format!("{name}.raw")), pixels).unwrap();
    }
}
