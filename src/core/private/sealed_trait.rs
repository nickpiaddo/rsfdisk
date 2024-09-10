// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library

// From standard library

// From this library
use crate::core::partition_table::PartitionTable;

pub trait Sealed {}

impl Sealed for PartitionTable {}
