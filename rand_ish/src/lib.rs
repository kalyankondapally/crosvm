// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fs::File;
use std::io::{self, Read};

/// A simple prng based on a Linear congruential generator
/// https://en.wikipedia.org/wiki/Linear_congruential_generator
pub struct SimpleRng {
    seed: u64,
}

impl SimpleRng {
    /// Create a new SimpleRng
    pub fn new(seed: u64) -> SimpleRng {
        SimpleRng { seed }
    }

    /// Generate random u64
    pub fn rng(&mut self) -> u64 {
        // a simple Linear congruential generator
        let a: u64 = 6364136223846793005;
        let c: u64 = 1442695040888963407;
        self.seed = a.wrapping_mul(self.seed).wrapping_add(c);
        self.seed
    }

    /// Generate a random alphanumeric string.
    pub fn str(&mut self, len: usize) -> String {
        self.filter_map(|v| uniform_sample_ascii_alphanumeric(v as u8))
            .take(len)
            .collect()
    }
}

impl Iterator for SimpleRng {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng())
    }
}

// Uniformly samples the ASCII alphanumeric characters given a random variable. If an `Err` is
// passed in, the error is returned as `Some(Err(...))`. If the the random variable can not be used
// to uniformly sample, `None` is returned.
fn uniform_sample_ascii_alphanumeric(b: u8) -> Option<char> {
    const ASCII_CHARS: &[u8] = b"\
          ABCDEFGHIJKLMNOPQRSTUVWXYZ\
          abcdefghijklmnopqrstuvwxyz\
          0123456789";
    let char_index = b as usize;
    // Throw away numbers that would cause sampling bias.
    if char_index >= ASCII_CHARS.len() * 4 {
        None
    } else {
        Some(ASCII_CHARS[char_index % ASCII_CHARS.len()] as char)
    }
}

/// Samples `/dev/urandom` and generates a random ASCII string of length `len`
pub fn urandom_str(len: usize) -> io::Result<String> {
    File::open("/dev/urandom")?
        .bytes()
        .filter_map(|b| b.map(uniform_sample_ascii_alphanumeric).transpose())
        .take(len)
        .collect::<io::Result<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_100() {
        for i in 0..100 {
            let s = urandom_str(i).unwrap();
            assert_eq!(s.len(), i);
            assert!(s.is_ascii());
            assert!(!s.contains(' '));
            assert!(!s.contains(|c: char| !c.is_ascii_alphanumeric()));
        }
    }
}
