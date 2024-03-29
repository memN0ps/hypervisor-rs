//! Module for handling Virtual Machine Stack (VmStack) operations.
//! Provides mechanisms to manage and configure the virtual machine's stack, including setup, allocation, and other related operations.

use {
    crate::{error::HypervisorError, utils::alloc::KernelAlloc},
    alloc::boxed::Box,
    core::mem::size_of,
    static_assertions::const_assert_eq,
};

/// The size of the kernel stack in bytes.
pub const KERNEL_STACK_SIZE: usize = 0x6000;

/// The size reserved for host RSP. This includes space allocated for padding.
pub const STACK_CONTENTS_SIZE: usize = KERNEL_STACK_SIZE - size_of::<*mut u64>() * 4;

/// Represents the Virtual Machine Stack (VmStack).
///
/// The structure is designed to align with 4-KByte boundaries and ensures proper setup for the host RSP during VM execution.
#[repr(C, align(4096))]
pub struct VmStack {
    /// The main contents of the VM stack during VM-exit. VMCS_HOST_RSP points to the end of this array inside the VMCS.
    pub stack_contents: [u8; STACK_CONTENTS_SIZE],

    /// A pointer to the `Vmx` instance, needed for the `launch_vm` assembly function, which is passed to vmexit handler.
    /// Padding to ensure the Host RSP remains 16-byte aligned.
    pub vmx: *mut u64,

    /// Padding to ensure the Host RSP remains 16-byte aligned.
    pub padding_3: u64,

    /// Padding to ensure the Host RSP remains 16-byte aligned.
    pub padding_2: u64,

    /// Padding to ensure the Host RSP remains 16-byte aligned.
    pub padding_1: u64,
}
const_assert_eq!(size_of::<VmStack>(), KERNEL_STACK_SIZE);
const_assert_eq!(size_of::<VmStack>() % 4096, 0);

impl VmStack {
    /// Sets up the VMCS_HOST_RSP region.
    ///
    /// Initializes the VM stack, ensuring it's properly aligned and configured for host execution.
    ///
    /// # Arguments
    ///
    /// * `vmstack` - A mutable reference to the VM stack.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the setup process.
    pub fn setup(vmstack: &mut Box<VmStack, KernelAlloc>) -> Result<(), HypervisorError> {
        log::debug!("Setting up VMCS_HOST_RSP region");
        log::trace!("VMCS_HOST_RSP Virtual Address: {:p}", vmstack);

        // Initialize the VM stack contents and reserved space.
        vmstack.stack_contents = [0u8; STACK_CONTENTS_SIZE];

        // We don't null `vmx` because it should already be populated and we don't want to overwrite it.

        vmstack.padding_3 = u64::MAX;
        vmstack.padding_2 = u64::MAX;
        vmstack.padding_1 = u64::MAX;

        log::debug!("VMCS_HOST_RSP setup successfully!");

        Ok(())
    }
}
