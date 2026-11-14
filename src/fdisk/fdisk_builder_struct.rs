// Copyright (c) 2023 Nick Piaddo
// SPDX-License-Identifier: Apache-2.0 OR MIT

// From dependency library
use typed_builder::TypedBuilder;

// From standard library
use std::fs::File;
use std::path::{Path, PathBuf};

// From this library
use crate::fdisk::DeviceAddressing;
use crate::fdisk::Fdisk;
use crate::fdisk::FdiskBuilderError;
use crate::fdisk::SizeFormat;

#[derive(Debug, TypedBuilder)]
#[builder(
    builder_type(
        name = FdiskBuilder,
        vis = "pub",
        doc ="Configure and instantiate a [`Fdisk`].\n\nFor usage, see [`FdiskBuilder::build`]."),
    build_method(vis = "", name = __make))]
pub(crate) struct CtxBuilder<'a> {
    #[builder(
        default,
        setter(
        transform = |device_path: impl AsRef<Path>| Some(device_path.as_ref().to_path_buf()),
        doc = "A device assigned to an [`Fdisk`] is opened and scanned for partition tables,
partitions, file systems, and topology metadata.\n\n
Usually the first step of preparing a newly installed disk, before any file system is setup, is to
create one or more regions on a storage device that can be managed separately. These regions are
called partitions.\n\n
The disk stores the information about the sizes and locations of these partitions in
an area known as the partition table; area the operating system reads before any other part of the
disk. Each partition then appears to the operating system as a distinct *logical* disk that uses
part of the actual disk.\n\n
The partition table is critical to the operating system's correct execution, as such a back up copy
is kept on the disk to allow for recovering from any metadata corruption. When the option
[`FdiskBuilder::display_partition_details`] is set, the library will perform a full scan
looking for an already existing file system or partition table.\n\n
To ensure any old metadata present on the assigned device is effectively erased before writing a
new partition table, set the option [`FdiskBuilder::wipe_device_metadata`].\n\n
To check whether an already existing file system or partition table was detected while
instantiating an [`Fdisk`], use
[`Fdisk::device_has_collisions`](crate::core::Fdisk::device_has_collisions).\n\n
Call [`Fdisk::device_describe_collisions`](crate::core::Fdisk::device_describe_collisions) to get the
name of the already existing file system or partition table."))]
    assign_device: Option<PathBuf>,

    #[builder(
        default,
        setter(
        transform = |device_file: &'a File, device_path: impl AsRef<Path>| Some((device_file,
                device_path.as_ref().to_path_buf())),
        doc = "This method acts like [`FdiskBuilder::assign_device`], but the caller assumes
responsibility for manually opening/closing the assigned device.  The device **MUST** be
opened in read/write mode if you set [`FdiskBuilder::enable_read_write`] to `true`."))]
    assign_device_by_file: Option<(&'a File, PathBuf)>,

    #[builder(
        default,
        setter(
        transform = |cylinders: u32, heads: u32, sectors: u32| Some((cylinders, heads, sectors)),
        doc = "Override the assigned device's number of cylinders, heads, and sectors. Obviously
this function can not modify the physical properties of a disk, it just sets the internal
values used when creating a partition table.\n\n
Note that the maximum number of heads can not exceed `255`, and the number of sectors exceed `63`."))]
    device_geometry: Option<(u32, u32, u32)>,

    #[builder(
        default,
        setter(
            strip_option,
            doc = "Override the assigned device's preferred grain size. The
grain size is used to align partitions, and is by default equal to the optimal I/O size
or 1 MiB, whichever is the largest. If the value given is too small, [`Fdisk`] will
use the largest of the device's physical sector size or the minimum I/O
size."
        )
    )]
    device_grain_size: Option<u64>,

    #[builder(
        default,
        setter(
        transform = |physical_sector_size:u32, logical_sector_size: u32|
        Some((physical_sector_size, logical_sector_size)),
        doc = "Override the assigned device's preferred logical and physical sectors sizes (in bytes)."))]
    device_sector_sizes: Option<(u32, u32)>,

    #[builder(
        default,
        setter(
            strip_option,
            doc = "Set the unit for device addressing. Used by `SUN` partition tables which locate the
beginning of a partition by its cylinder value. (default [`DeviceAddressing::Sector`])"
        )
    )]
    device_addressing: Option<DeviceAddressing>,

    #[builder(setter(
        strip_bool,
        doc = "Enable the dialog-driven partitioning process (interactive mode). Disabled by default."
    ))]
    enable_interactive: bool,

    #[builder(setter(
        strip_bool,
        doc = "Show only a list of partitions when printing on the console (shows NO detailed metadata). (default)"
    ))]
    display_partition_list_only: bool,

    #[builder(setter(
        strip_bool,
        doc = "Show each partition's detailed metadata when printing on the console."
    ))]
    display_partition_details: bool,

    #[builder(setter(
        strip_bool,
        doc = "By default, an [`Fdisk`] keeps all data on the first sector of its assigned device
when it creates a new MBR or GPT partition table. Set this option if you want to erase the
data on the first sector."
    ))]
    erase_master_boot_record: bool,

    #[builder(
        default,
        setter(
            strip_option,
            doc = "Set the format in which to display partition sizes. (default [`SizeFormat::Bytes`])"
        )
    )]
    partition_size_format: Option<SizeFormat>,

    #[builder(setter(
        strip_bool,
        doc = "Set an [`Fdisk`] as read-write to allow it to persist changes to disk."
    ))]
    enable_read_write: bool,

    #[builder(setter(
        strip_bool,
        doc = "Remove all existing partition tables, file systems, and RAID signatures on the
assigned device before writing a new partition table."
    ))]
    wipe_device_metadata: bool,
}

#[allow(non_camel_case_types)]
impl<
        'a,
        __assign_device: ::typed_builder::Optional<Option<PathBuf>>,
        __assign_device_by_file: ::typed_builder::Optional<Option<(&'a File, PathBuf)>>,
        __device_geometry: ::typed_builder::Optional<Option<(u32, u32, u32)>>,
        __device_grain_size: ::typed_builder::Optional<Option<u64>>,
        __device_sector_sizes: ::typed_builder::Optional<Option<(u32, u32)>>,
        __device_addressing: ::typed_builder::Optional<Option<DeviceAddressing>>,
        __enable_interactive: ::typed_builder::Optional<bool>,
        __display_partition_list_only: ::typed_builder::Optional<bool>,
        __display_partition_details: ::typed_builder::Optional<bool>,
        __erase_master_boot_record: ::typed_builder::Optional<bool>,
        __partition_size_format: ::typed_builder::Optional<Option<SizeFormat>>,
        __enable_read_write: ::typed_builder::Optional<bool>,
        __wipe_device_metadata: ::typed_builder::Optional<bool>,
    >
    FdiskBuilder<
        'a,
        (
            __assign_device,
            __assign_device_by_file,
            __device_geometry,
            __device_grain_size,
            __device_sector_sizes,
            __device_addressing,
            __enable_interactive,
            __display_partition_list_only,
            __display_partition_details,
            __erase_master_boot_record,
            __partition_size_format,
            __enable_read_write,
            __wipe_device_metadata,
        ),
    >
{
    /// Completes a [`Fdisk`]'s configuration process, and creates a new instance.
    pub fn build(self) -> Result<Fdisk<'a>, FdiskBuilderError> {
        log::debug!("FdiskBuilder::build building a new `Fdisk` instance");

        let builder = self.__make();

        let mut context = Fdisk::new()?;

        match (
            builder.enable_read_write,
            builder.assign_device,
            builder.assign_device_by_file,
        ) {
            // Assign device.
            (false, Some(device_path), None) => {
                context.assign_device_read_only(device_path)?;
            }
            (true, Some(device_path), None) => {
                context.assign_device_read_write(device_path)?;
            }
            // Assign device by file.
            (false, None, Some((device_file, device_path))) => {
                context.assign_device_by_file_read_only(device_file, device_path)?;
            }
            (true, None, Some((device_file, device_path))) => {
                context.assign_device_by_file_read_write(device_file, device_path)?;
            }
            (_, None, None) => {
                let err_msg =
                    "one of the methods `assign_device` or `assign_device_by_file` must be called"
                        .to_owned();
                log::debug!("FdiskBuilder::build {}", err_msg);

                return Err(FdiskBuilderError::Required(err_msg));
            }
            (_, Some(_), Some(_)) => {
                let err_msg =
                    "methods `assign_device` and `assign_device_by_file` can not be called at the same time"
                        .to_owned();
                log::debug!("FdiskBuilder::build {}", err_msg);

                return Err(FdiskBuilderError::MutuallyExclusive(err_msg));
            }
        }

        // ----------------------------------------------------------------------------
        // Override the device's preferred values.
        // These overrides must be set BEFORE any assign_device_* function is called.

        if let Some((cylinders, heads, sectors)) = builder.device_geometry {
            context.save_device_geometry_overrides(cylinders, heads, sectors)?;
        }

        if let Some(grain_size) = builder.device_grain_size {
            context.save_device_grain_size_override(grain_size)?;
        }

        if let Some((physical_sector_size, logical_sector_size)) = builder.device_sector_sizes {
            context.save_device_sector_overrides(physical_sector_size, logical_sector_size)?;
        }
        // ----------------------------------------------------------------------------

        match builder.device_addressing {
            // Default
            None => context.set_device_addressing(DeviceAddressing::Sector)?,
            Some(addressing) => context.set_device_addressing(addressing)?,
        }

        // Enable interactive partitioning prompts.
        if builder.enable_interactive {
            context.enable_interactive()?;
        } else {
            context.disable_interactive()?;
        }

        // Display partition metadata.
        match (
            builder.display_partition_details,
            builder.display_partition_list_only,
        ) {
            // Show list only by default
            (false, false) | (false, true) => context
                .enable_partition_list_only()
                .map_err(FdiskBuilderError::from)?,
            (true, false) => context
                .enable_partition_details()
                .map_err(FdiskBuilderError::from)?,
            (true, true) => {
                let err_msg =
                    "methods `display_partition_details` and `display_partition_list_only` can not be called at the same time"
                        .to_owned();
                log::debug!("FdiskBuilder::build {}", err_msg);

                return Err(FdiskBuilderError::MutuallyExclusive(err_msg));
            }
        }

        // Partition size display format.
        match builder.partition_size_format {
            // Default
            None => context.set_partition_size_format(SizeFormat::Bytes)?,
            Some(format) => context.set_partition_size_format(format)?,
        }

        if builder.erase_master_boot_record {
            context.erase_master_boot_record()?;
        } else {
            context.protect_master_boot_record()?;
        }

        // Wipe all device metadata before writing partition table.
        if builder.wipe_device_metadata {
            context.enable_metadata_wipe()?;
        } else {
            context.disable_metadata_wipe()?;
        }

        log::debug!("FdiskBuilder::build built a new `Fdisk` instance");
        Ok(context)
    }
}
