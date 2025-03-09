#![no_std]

//! This module provides the ability to call Raspberry Pi ROM functions.
//!
//! The RP2040 ROM contains several useful functions that can be called
//! directly from user code, including functions to reset the device and
//! enter the USB bootloader.
//!
//! # Safety
//!
//! All functions in this crate are marked as `unsafe` because they involve
//! direct hardware manipulation and can reset the device.
//!
//! # Example
//!
//! ```rust,no_run
//! use rp2040_rom::ROM;
//!
//! // Reset into USB bootloader mode
//! unsafe {
//!     ROM::reset_usb_boot(0, 0);
//! }
//! ```

// Copyright (c) 2025 Piers Finlayson <piers@piers.rocks>
//
// MIT licensed - see https://opensource.org/licenses/MIT

/// ROM function table offset for the RP2040
/// From the datasheet:
///   Pointer to a public function lookup table (rom_func_table)
const BOOTROM_FUNC_TABLE_OFFSET: u16 = 0x14;

/// ROM lookup table offset for the RP2040
/// From the datasheet:
///   Pointer to a helper function (rom_table_lookup())
const BOOTROM_TABLE_LOOKUP_OFFSET: u16 = 0x18;

/// Object containing exposed ROM functions
#[allow(clippy::upper_case_acronyms)]
pub struct ROM {}

/// Public functions
impl ROM {
    /// Resets the device and enters USB bootloader mode.
    ///
    /// # Parameters
    ///
    /// * `usb_activity_gpio_pin_mask` - Bitmask of GPIO pins to check for USB activity
    /// * `disable_interface_mask` - Bitmask to disable specific interfaces
    ///
    /// # Safety
    ///
    /// This function will reset the device and not return.
    pub unsafe fn reset_usb_boot(usb_activity_gpio_pin_mask: u32, disable_interface_mask: u32) -> ! {
        // ROM reset_usb_boot function definition
        type RomResetUsbBootFn =
            unsafe extern "C" fn(usb_activity_gpio_pin_mask: u32, disable_interface_mask: u32);

        // The two character code for the reset_usb_boot function in the
        // lookup table
        const ROM_FUNC_RESET_USB_BOOT: (u8, u8) = (b'U', b'B');

        // Get the function pointer for reset_usb_boot and turn it into a
        // function we can call
        let func_ptr = Self::rom_func_lookup(ROM_FUNC_RESET_USB_BOOT);
        let func: RomResetUsbBootFn = core::mem::transmute(func_ptr);

        // Call the function
        func(usb_activity_gpio_pin_mask, disable_interface_mask);
        
        // Loop in order to convince the compiler this function own't return
        loop {}
    }
}

// Private functions
impl ROM {
    // Get the lookup code for a function, based on the two characters
    // While the lookup table technically takes a u16, the lookup function
    // takes a u32, so we'll use a u32 internally.
    const fn rom_table_code(c1: u8, c2: u8) -> u32 {
        (c1 as u32) | ((c2 as u32) << 8)
    }

    // Convert a u16 provided by the ROM lookup table to a pointer
    unsafe fn rom_hword_as_ptr(rom_address: u16) -> *mut core::ffi::c_void {
        // Convert to usize first
        let addr_val = rom_address as usize;

        // Create pointer AND dereference it
        let value = unsafe { *(addr_val as *const u16) };

        // Convert value to pointer size then to void pointer
        value as usize as *mut core::ffi::c_void
    }

    // Get the pointer for a function, based on the two characters used to
    // index it
    unsafe fn rom_func_lookup(code: (u8, u8)) -> *mut core::ffi::c_void {
        // The ROM reset_usb_boot function definition
        type RomTableLookupFn =
            unsafe extern "C" fn(table: *const u16, code: u32) -> *mut core::ffi::c_void;

        // Get the 32-bit code for the two characters that we need to pass
        // into the lookup function
        let (c1, c2) = code;
        let code = Self::rom_table_code(c1, c2);

        // Get the function table address
        let func_table_addr = Self::rom_hword_as_ptr(BOOTROM_FUNC_TABLE_OFFSET);
        let func_table = func_table_addr as *const u16;

        // Get the lookup function address
        let lookup_addr = Self::rom_hword_as_ptr(BOOTROM_TABLE_LOOKUP_OFFSET);
        let rom_table_lookup: RomTableLookupFn = core::mem::transmute(lookup_addr);

        // Use the lookup function to lookup this code
        rom_table_lookup(func_table, code)
    }
}
