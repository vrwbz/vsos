# VSOS Project Instructions

- This is a Rust `no_std` x86_64 kernel.
- Preserve the bootloader entry point and panic handler when changing the kernel.
- Use the nightly Rust toolchain and QEMU for local verification.
- Keep early kernel code small, explicit, and dependency-light.
- Update `README.md` when build or run commands change.
- Validate changes with `cargo +nightly bootimage` when the toolchain is available.
