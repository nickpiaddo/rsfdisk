# Web snapshots

----

Snapshots of helpful website, blog, and documentation pages used while building
this project.

A bookmark is useful, but you never know when a site will go dark forever.
Having a local copy is a good remedy to having information disappear overnight.

These tools were essential for collecting copies:

- [Monolith](https://github.com/Y2Z/monolith)
- [SingleFile](https://addons.mozilla.org/en-US/firefox/addon/single-file/)

----

`[a]`link to the version saved in this repository.

## Alpine Linux

[Managing repositories][1] [[a]][2]

[How to enable automatic login in alpine linux for root][3] [[a]][4]

## Linux Kernel

[List available modules][5] [[a]][6]

## QEMU

[QEMU Machine Protocol][7] [[a]][8]

## NixOS

[C header includes in NixOS][9] [[a]][10]

[NixOS - Environment variables][11] [[a]][12]

## Rust

[How do I handle errors from libc functions in an idiomatic Rust manner?][15] [[a]][16]

[Neat Rust Tricks: Passing Rust Closures to C][21] [[a]][22]

[How can I write binding to a C function that expects an open file handle in
Rust?][29] [[a]][30]

## Partition table/ partition

[GUID Partition Table (GPT) Disk Layout][13] [[a]][14]

[Partition type][17] [[a]][18]

[GUID Partition Table][19] [[a]][20]

[How to script sfdisk or parted for multiple partitions?][31] [[a]][32]

## File system

[newfs — construct a new UFS1/UFS2 file system][23] [[a]][24]

[bsdlabel — read and write BSD label][25] [[a]][26]

[disktab — disk description file][27] [[a]][28]

[1]: https://wiki.alpinelinux.org/wiki/Repositories#Managing_repositories
[2]: alpine-linux/managing-repositories.html
[3]: https://unix.stackexchange.com/questions/751105/how-to-enable-automatic-login-in-alpine-linux-for-root
[4]: alpine-linux/automatic-login-in-alpine-linux-for-root.html
[5]: https://wiki.gentoo.org/wiki/Kernel_Modules#List_available_modules
[6]: linux-kernel/kernel-modules.html
[7]: https://wiki.qemu.org/Documentation/QMP#By_hand
[8]: qemu/qemu-machine-protocol.html
[9]: https://discourse.nixos.org/t/c-header-includes-in-nixos/17410
[10]: nixos/c-header-includes.html
[11]: https://nixos.wiki/wiki/Environment_variables
[12]: nixos/environment-variables.html
[13]: https://uefi.org/specs/UEFI/2.10/05_GUID_Partition_Table_Format.html
[14]: web-snapshots/standards/GPT-MBR-partition-table-format.html
[15]: https://stackoverflow.com/questions/42772307/how-do-i-handle-errors-from-libc-functions-in-an-idiomatic-rust-manner
[16]: rust/handle_libc_error_in_idiomatic_rust.html
[17]: https://en.wikipedia.org/wiki/Partition_type
[18]: misc/partition-type.html
[19]: https://en.wikipedia.org/w/index.php?title=GUID_Partition_Table&oldid=1214116294#Partition_type_GUIDs
[20]: misc/guid-partition-table.html
[21]: http://blog.sagetheprogrammer.com/neat-rust-tricks-passing-rust-closures-to-c
[22]: web-snapshots/rust/passing-rust-closures-to-c.html
[23]: https://www.gsp.com/cgi-bin/man.cgi?section=8&topic=newfs
[24]: web-snapshots/fs/construct-a-new-UFS1-UFS2-file-system.html
[25]: https://www.gsp.com/cgi-bin/man.cgi?topic=BSDLABEL#SAVED_FILE_FORMAT
[26]: web-snapshots/fs/read-and-write-BSD-label.html
[27]: https://man.openbsd.org/disktab.5
[28]: web-snapshots/fs/disktab-5-OpenBSD-manual-pages.html
[29]: https://stackoverflow.com/questions/32484641/how-can-i-write-binding-to-a-c-function-that-expects-an-open-file-handle-in-rust/
[30]: rust/ffi-rust-File-to-C-FILE.html
[31]: https://stackoverflow.com/questions/12150116/how-to-script-sfdisk-or-parted-for-multiple-partitions
[32]: web-snapshots/fdisk/sfdisk-script-structure.html
