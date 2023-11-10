//! Provides handlers for managing VM exits due to the XSETBV instruction, ensuring
//! controlled manipulation of the XCR0 register by guest VMs.

use {
    crate::{
        intel::{vmexit::VmExit, vmlaunch::GuestRegisters},
        utils::instructions::{cr4, cr4_write, xsetbv},
    },
    x86::controlregs::{Cr4, Xcr0},
};

/// Manages the XSETBV instruction during a VM exit. It logs the event, updates
/// CR4 to enable the necessary feature, sets the XCR0 value, and advances the
/// guest's instruction pointer.
///
/// # Arguments
///
/// * `registers` - A mutable reference to the guest VM's general-purpose registers.
pub fn handle_xsetbv(registers: &mut GuestRegisters) {
    // Extract the XCR (extended control register) number from the guest's RCX register.
    let xcr: u32 = registers.rcx as u32;

    // Combine the guest's RAX and RDX registers to form the 64-bit value for the XCR0 register.
    let value = (registers.rax & 0xffff_ffff) | ((registers.rdx & 0xffff_ffff) << 32);

    // Attempt to create an Xcr0 structure from the given bits.
    let value = Xcr0::from_bits_truncate(value);

    log::info!("XSETBV executed with xcr: {:#x}, value: {:#x}", xcr, value);

    // Enable the OS XSAVE feature in CR4 before setting the extended control register value.
    cr4_write(cr4() | Cr4::CR4_ENABLE_OS_XSAVE);

    // Write the value to the specified XCR (extended control register).
    xsetbv(value);

    // Advance the guest's instruction pointer to the next instruction to be executed.
    VmExit::advance_guest_rip(registers);
}