// Copyright 2019 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::filesystem::{DirEntry, DirectoryIterator, FileSystem, ZeroCopyReader, ZeroCopyWriter};
use crate::server::{Reader, Server, Writer};

// Use a file system that does nothing since we are fuzzing the server implementation.
struct NullFs;
impl FileSystem for NullFs {
    type Inode = u64;
    type Handle = u64;
    type DirIter = NullIter;
}

struct NullIter;
impl DirectoryIterator for NullIter {
    fn next(&mut self) -> Option<DirEntry> {
        None
    }
}

/// Fuzz the server implementation.
pub fn fuzz_server<R: Reader + ZeroCopyReader, W: Writer + ZeroCopyWriter>(r: R, w: W) {
    let server = Server::new(NullFs);

    let _ = server.handle_message(r, w);
}
