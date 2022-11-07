// Copyright 2022 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! This file contains a dummy backend whose only purpose is to let the decoder
//! run so we can test it in isolation.

use crate::decoders::MappableHandle;
use crate::decoders::Result;

pub struct BackendHandle;

impl MappableHandle for BackendHandle {
    fn read(&mut self, _: &mut [u8]) -> Result<()> {
        Ok(())
    }

    fn image_size(&mut self) -> Result<usize> {
        Ok(1)
    }
}
