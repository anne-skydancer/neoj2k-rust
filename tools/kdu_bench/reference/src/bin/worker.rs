// SPDX-License-Identifier: BSD-2-Clause
#![forbid(unsafe_code)]
use neo_j2k::{DecodeParams, DecodeTarget, Decoder, DecoderConfig, OutputPlane, SampleType};
use std::io::{BufRead, Write};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 7);
    let input = std::fs::read(&args[1]).unwrap();
    let expected = std::fs::read(&args[2]).unwrap();
    let width: usize = args[4].parse().unwrap();
    let height: usize = args[5].parse().unwrap();
    let channels: usize = args[6].parse().unwrap();
    assert_eq!(expected.len(), width * height * channels);
    let mut storage = vec![0xcd; expected.len() + 128];
    let planes: Vec<_> = (0..channels)
        .map(|c| OutputPlane {
            component: c as u32,
            offset: 64 + c,
            row_stride: (width * channels) as isize,
            sample_stride: channels,
        })
        .collect();
    let decode = |storage: &mut [u8]| {
        let mut codec = Decoder::new(DecoderConfig::default());
        let header = codec.read_header(&input).unwrap();
        assert_eq!(header.components.len(), channels);
        for c in &header.components {
            assert_eq!(
                (c.width as usize, c.height as usize, c.precision, c.signed),
                (width, height, 8, false)
            );
        }
        assert!(
            !codec
                .decode_into(
                    std::hint::black_box(&input),
                    DecodeParams::default(),
                    SampleType::U8,
                    &mut [DecodeTarget {
                        bytes: storage,
                        planes: &planes
                    }]
                )
                .unwrap()
        );
    };
    let verify = |storage: &mut [u8]| {
        storage.fill(0xcd);
        decode(storage);
        assert_eq!(&storage[64..64 + expected.len()], expected);
        assert!(storage[..64].iter().all(|&b| b == 0xcd));
        assert!(storage[64 + expected.len()..].iter().all(|&b| b == 0xcd));
    };
    verify(&mut storage);
    std::fs::write(&args[3], &storage[64..64 + expected.len()]).unwrap();
    println!("{{\"ready\":true}}");
    std::io::stdout().flush().unwrap();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let batch: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        assert_eq!(batch.len(), 2);
        let (iterations, warm) = (batch[0], batch[1]);
        assert!((1..=100000).contains(&iterations) && warm <= 1000);
        verify(&mut storage);
        for _ in 0..warm {
            decode(&mut storage);
        }
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            decode(&mut storage);
        }
        let us = start.elapsed().as_secs_f64() * 1e6 / iterations as f64;
        std::hint::black_box(&storage);
        verify(&mut storage);
        println!("{{\"us\":{us:.9},\"validated\":true}}");
        std::io::stdout().flush().unwrap();
    }
}
