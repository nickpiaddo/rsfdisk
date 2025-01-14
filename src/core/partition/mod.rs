// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Module for working with partitions in a partition table.

// From dependency library

// From standard library

// From this library
pub use bit_flag_enum::BitFlag;
pub use code_enum::Code;
pub use dos_flag_enum::DOSFlag;
pub use gpt_flag_enum::GPTFlag;
pub use guid_enum::Guid;
pub(crate) use partition_builder_struct::PartBuilder;
pub use partition_builder_struct::PartitionBuilder;
pub use partition_iter_mut_struct::PartitionIterMut;
pub use partition_iter_struct::PartitionIter;
pub(crate) use partition_kind_builder_struct::PartTypeBuilder;
pub use partition_kind_builder_struct::PartitionKindBuilder;
pub use partition_kind_struct::PartitionKind;
pub use partition_list_struct::PartitionList;
pub use partition_struct::Partition;
pub use sgi_flag_enum::SGIFlag;

mod bit_flag_enum;
mod code_enum;
mod dos_flag_enum;
mod gpt_flag_enum;
mod guid_enum;
mod partition_builder_struct;
mod partition_iter_mut_struct;
mod partition_iter_struct;
mod partition_kind_builder_struct;
mod partition_kind_struct;
mod partition_list_struct;
mod partition_struct;
mod sgi_flag_enum;
