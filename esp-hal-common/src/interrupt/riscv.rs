//! Interrupt handling - RISCV
//!
//! When the `vectored` feature is enabled, CPU interrupts 1 through 15 are
//! reserved for each of the possible interrupt priorities.
//!
//! ```rust
//! interrupt1() => Priority::Priority1
//! interrupt2() => Priority::Priority2
//! ...
//! interrupt15() => Priority::Priority15
//! ```

use riscv::register::mcause;

use crate::{
    pac::{self, Interrupt},
    Cpu,
};

// User code shouldn't usually take the mutable TrapFrame or the TrapFrame in
// general. However this makes things like preemtive multitasking easier in
// future
extern "C" {
    fn interrupt1(frame: &mut TrapFrame);
    fn interrupt2(frame: &mut TrapFrame);
    fn interrupt3(frame: &mut TrapFrame);
    fn interrupt4(frame: &mut TrapFrame);
    fn interrupt5(frame: &mut TrapFrame);
    fn interrupt6(frame: &mut TrapFrame);
    fn interrupt7(frame: &mut TrapFrame);
    fn interrupt8(frame: &mut TrapFrame);
    fn interrupt9(frame: &mut TrapFrame);
    fn interrupt10(frame: &mut TrapFrame);
    fn interrupt11(frame: &mut TrapFrame);
    fn interrupt12(frame: &mut TrapFrame);
    fn interrupt13(frame: &mut TrapFrame);
    fn interrupt14(frame: &mut TrapFrame);
    fn interrupt15(frame: &mut TrapFrame);
    fn interrupt16(frame: &mut TrapFrame);
    fn interrupt17(frame: &mut TrapFrame);
    fn interrupt18(frame: &mut TrapFrame);
    fn interrupt19(frame: &mut TrapFrame);
    fn interrupt20(frame: &mut TrapFrame);
    fn interrupt21(frame: &mut TrapFrame);
    fn interrupt22(frame: &mut TrapFrame);
    fn interrupt23(frame: &mut TrapFrame);
    fn interrupt24(frame: &mut TrapFrame);
    fn interrupt25(frame: &mut TrapFrame);
    fn interrupt26(frame: &mut TrapFrame);
    fn interrupt27(frame: &mut TrapFrame);
    fn interrupt28(frame: &mut TrapFrame);
    fn interrupt29(frame: &mut TrapFrame);
    fn interrupt30(frame: &mut TrapFrame);
    fn interrupt31(frame: &mut TrapFrame);
}

/// Interrupt kind
pub enum InterruptKind {
    /// Level interrupt
    Level,
    /// Edge interrupt
    Edge,
}

/// Enumeration of available CPU interrupts.
/// It is possible to create a handler for each of the interrupts. (e.g.
/// `interrupt3`)
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum CpuInterrupt {
    Interrupt1 = 1,
    Interrupt2,
    Interrupt3,
    Interrupt4,
    Interrupt5,
    Interrupt6,
    Interrupt7,
    Interrupt8,
    Interrupt9,
    Interrupt10,
    Interrupt11,
    Interrupt12,
    Interrupt13,
    Interrupt14,
    Interrupt15,
    Interrupt16,
    Interrupt17,
    Interrupt18,
    Interrupt19,
    Interrupt20,
    Interrupt21,
    Interrupt22,
    Interrupt23,
    Interrupt24,
    Interrupt25,
    Interrupt26,
    Interrupt27,
    Interrupt28,
    Interrupt29,
    Interrupt30,
    Interrupt31,
}

/// Interrupt priority levels.
#[repr(u8)]
pub enum Priority {
    None,
    Priority1,
    Priority2,
    Priority3,
    Priority4,
    Priority5,
    Priority6,
    Priority7,
    Priority8,
    Priority9,
    Priority10,
    Priority11,
    Priority12,
    Priority13,
    Priority14,
    Priority15,
}

/// Assign a peripheral interrupt to an CPU interrupt.
///
/// Great care must be taken when using the `vectored` feature (enabled by
/// default). Avoid interrupts 1 - 15 when interrupt vectoring is enabled.
pub unsafe fn map(_core: Cpu, interrupt: Interrupt, which: CpuInterrupt) {
    let interrupt_number = interrupt as isize;
    let cpu_interrupt_number = which as isize;
    let intr = &*crate::pac::INTERRUPT_CORE0::PTR;
    let intr_map_base = intr.mac_intr_map.as_ptr();
    intr_map_base
        .offset(interrupt_number)
        .write_volatile(cpu_interrupt_number as u32);
}

/// Enable a CPU interrupt
pub unsafe fn enable_cpu_interrupt(which: CpuInterrupt) {
    let cpu_interrupt_number = which as isize;
    let intr = &*crate::pac::INTERRUPT_CORE0::PTR;
    intr.cpu_int_enable
        .modify(|r, w| w.bits((1 << cpu_interrupt_number) | r.bits()));
}

/// Disable the given peripheral interrupt.
pub fn disable(_core: Cpu, interrupt: Interrupt) {
    unsafe {
        let interrupt_number = interrupt as isize;
        let intr = &*crate::pac::INTERRUPT_CORE0::PTR;
        let intr_map_base = intr.mac_intr_map.as_ptr();
        intr_map_base.offset(interrupt_number).write_volatile(0);
    }
}

/// Set the interrupt kind (i.e. level or edge) of an CPU interrupt
///
/// This is safe to call when the `vectored` feature is enabled. The vectored
/// interrupt handler will take care of clearing edge interrupt bits.
pub fn set_kind(_core: Cpu, which: CpuInterrupt, kind: InterruptKind) {
    unsafe {
        let intr = &*crate::pac::INTERRUPT_CORE0::PTR;
        let cpu_interrupt_number = which as isize;

        let interrupt_type = match kind {
            InterruptKind::Level => 0,
            InterruptKind::Edge => 1,
        };
        intr.cpu_int_type.modify(|r, w| {
            w.bits(
                r.bits() & !(1 << cpu_interrupt_number) | (interrupt_type << cpu_interrupt_number),
            )
        });
    }
}

/// Set the priority level of an CPU interrupt
///
/// Great care must be taken when using the `vectored` feature (enabled by
/// default). Avoid changing the priority of interrupts 1 - 15 when interrupt
/// vectoring is enabled.
pub unsafe fn set_priority(_core: Cpu, which: CpuInterrupt, priority: Priority) {
    let intr = &*crate::pac::INTERRUPT_CORE0::PTR;
    let cpu_interrupt_number = which as isize;
    let intr_prio_base = intr.cpu_int_pri_0.as_ptr();

    intr_prio_base
        .offset(cpu_interrupt_number as isize)
        .write_volatile(priority as u32);
}

/// Clear a CPU interrupt
#[inline]
pub fn clear(_core: Cpu, which: CpuInterrupt) {
    unsafe {
        let cpu_interrupt_number = which as isize;
        let intr = &*crate::pac::INTERRUPT_CORE0::PTR;
        intr.cpu_int_clear
            .write(|w| w.bits(1 << cpu_interrupt_number));
    }
}

/// Get status of peripheral interrupts
#[inline]
pub fn get_status(_core: Cpu) -> u128 {
    unsafe {
        ((*crate::pac::INTERRUPT_CORE0::PTR)
            .intr_status_reg_0
            .read()
            .bits() as u128)
            | ((*crate::pac::INTERRUPT_CORE0::PTR)
                .intr_status_reg_1
                .read()
                .bits() as u128)
                << 32
    }
}

#[cfg(feature = "vectored")]
pub use vectored::*;

#[cfg(feature = "vectored")]
mod vectored {
    use procmacros::ram;

    use super::*;

    // Setup interrupts 1-15 ready for vectoring
    #[doc(hidden)]
    pub(crate) unsafe fn init_vectoring() {
        for i in 1..=15 {
            set_kind(
                crate::get_core(),
                core::mem::transmute(i),
                InterruptKind::Level,
            );
            set_priority(
                crate::get_core(),
                core::mem::transmute(i),
                core::mem::transmute(i as u8),
            );
            enable_cpu_interrupt(core::mem::transmute(i));
        }
    }

    /// Get the interrupts configured for the core
    #[inline]
    fn get_configured_interrupts(_core: Cpu, mut status: u128) -> [u128; 15] {
        unsafe {
            let intr = &*crate::pac::INTERRUPT_CORE0::PTR;
            let intr_map_base = intr.mac_intr_map.as_ptr();
            let intr_prio_base = intr.cpu_int_pri_0.as_ptr();

            let mut prios = [0u128; 15];

            while status != 0 {
                let interrupt_nr = status.trailing_zeros();
                let i = interrupt_nr as isize;
                let cpu_interrupt = intr_map_base.offset(i).read_volatile();
                // safety: cast is safe because of repr(u32)
                let cpu_interrupt: CpuInterrupt = core::mem::transmute(cpu_interrupt);
                let prio = intr_prio_base
                    .offset(cpu_interrupt as isize)
                    .read_volatile();

                prios[prio as usize] |= 1 << i;
                status &= !(1u128 << interrupt_nr);
            }

            prios
        }
    }

    /// Interrupt Error
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Error {
        InvalidInterruptPriority,
    }

    /// Enables a interrupt at a given priority
    ///
    /// Note that interrupts still need to be enabled globally for interrupts
    /// to be serviced.
    pub fn enable(interrupt: Interrupt, level: Priority) -> Result<(), Error> {
        if matches!(level, Priority::None) {
            return Err(Error::InvalidInterruptPriority);
        }
        unsafe {
            let cpu_interrupt = core::mem::transmute(level as u8 as u32);
            map(crate::get_core(), interrupt, cpu_interrupt);
            enable_cpu_interrupt(cpu_interrupt);
        }
        Ok(())
    }

    #[ram]
    unsafe fn handle_interrupts(cpu_intr: CpuInterrupt, context: &mut TrapFrame) {
        let status = get_status(crate::get_core());

        // this has no effect on level interrupts, but the interrupt may be an edge one
        // so we clear it anyway
        clear(crate::get_core(), cpu_intr);

        let configured_interrupts = get_configured_interrupts(crate::get_core(), status);
        let mut interrupt_mask = status & configured_interrupts[cpu_intr as usize];
        while interrupt_mask != 0 {
            let interrupt_nr = interrupt_mask.trailing_zeros();
            // Interrupt::try_from can fail if interrupt already de-asserted:
            // silently ignore
            if let Ok(interrupt) = pac::Interrupt::try_from(interrupt_nr as u8) {
                handle_interrupt(interrupt, context)
            }
            interrupt_mask &= !(1u128 << interrupt_nr);
        }
    }

    #[ram]
    unsafe fn handle_interrupt(interrupt: Interrupt, save_frame: &mut TrapFrame) {
        extern "C" {
            // defined in each hal
            fn EspDefaultHandler(interrupt: Interrupt);
        }
        let handler = pac::__EXTERNAL_INTERRUPTS[interrupt as usize]._handler;
        if handler as *const _ == EspDefaultHandler as *const unsafe extern "C" fn() {
            EspDefaultHandler(interrupt);
        } else {
            let handler: fn(&mut TrapFrame) = core::mem::transmute(handler);
            handler(save_frame);
        }
    }

    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt1(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt1, context)
    }

    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt2(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt2, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt3(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt3, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt4(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt4, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt5(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt5, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt6(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt6, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt7(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt7, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt8(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt8, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt9(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt9, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt10(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt10, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt11(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt11, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt12(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt12, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt13(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt13, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt14(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt14, context)
    }
    #[no_mangle]
    #[ram]
    pub unsafe fn interrupt15(context: &mut TrapFrame) {
        handle_interrupts(CpuInterrupt::Interrupt15, context)
    }
}

/// Registers saved in trap handler
#[doc(hidden)]
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct TrapFrame {
    pub ra: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub gp: usize,
    pub tp: usize,
    pub sp: usize,
    pub pc: usize,
    pub mstatus: usize,
    pub mcause: usize,
    pub mtval: usize,
}

/// # Safety
///
/// This function is called from an assembly trap handler.
#[doc(hidden)]
#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust_hal"]
pub unsafe extern "C" fn start_trap_rust_hal(trap_frame: *mut TrapFrame) {
    extern "C" {
        // defined in riscv-rt
        pub fn DefaultHandler();
    }

    let cause = mcause::read();
    if cause.is_exception() {
        let pc = riscv::register::mepc::read();
        handle_exception(pc, trap_frame);
    } else {
        let code = riscv::register::mcause::read().code();
        match code {
            1 => interrupt1(trap_frame.as_mut().unwrap()),
            2 => interrupt2(trap_frame.as_mut().unwrap()),
            3 => interrupt3(trap_frame.as_mut().unwrap()),
            4 => interrupt4(trap_frame.as_mut().unwrap()),
            5 => interrupt5(trap_frame.as_mut().unwrap()),
            6 => interrupt6(trap_frame.as_mut().unwrap()),
            7 => interrupt7(trap_frame.as_mut().unwrap()),
            8 => interrupt8(trap_frame.as_mut().unwrap()),
            9 => interrupt9(trap_frame.as_mut().unwrap()),
            10 => interrupt10(trap_frame.as_mut().unwrap()),
            11 => interrupt11(trap_frame.as_mut().unwrap()),
            12 => interrupt12(trap_frame.as_mut().unwrap()),
            13 => interrupt13(trap_frame.as_mut().unwrap()),
            14 => interrupt14(trap_frame.as_mut().unwrap()),
            16 => interrupt16(trap_frame.as_mut().unwrap()),
            15 => interrupt15(trap_frame.as_mut().unwrap()),
            17 => interrupt17(trap_frame.as_mut().unwrap()),
            18 => interrupt18(trap_frame.as_mut().unwrap()),
            19 => interrupt19(trap_frame.as_mut().unwrap()),
            20 => interrupt20(trap_frame.as_mut().unwrap()),
            21 => interrupt21(trap_frame.as_mut().unwrap()),
            22 => interrupt22(trap_frame.as_mut().unwrap()),
            23 => interrupt23(trap_frame.as_mut().unwrap()),
            24 => interrupt24(trap_frame.as_mut().unwrap()),
            25 => interrupt25(trap_frame.as_mut().unwrap()),
            26 => interrupt26(trap_frame.as_mut().unwrap()),
            27 => interrupt27(trap_frame.as_mut().unwrap()),
            28 => interrupt28(trap_frame.as_mut().unwrap()),
            29 => interrupt29(trap_frame.as_mut().unwrap()),
            30 => interrupt30(trap_frame.as_mut().unwrap()),
            31 => interrupt31(trap_frame.as_mut().unwrap()),
            _ => DefaultHandler(),
        };
    }
}

/// Apply atomic emulation if needed. Call the default exception handler
/// otherwise.
///
/// # Safety
///
/// This function is called from an trap handler.
#[doc(hidden)]
unsafe fn handle_exception(pc: usize, trap_frame: *mut TrapFrame) {
    let insn: usize = *(pc as *const _);
    let needs_atomic_emulation = (insn & 0b1111111) == 0b0101111;

    if !needs_atomic_emulation {
        extern "C" {
            fn ExceptionHandler(tf: *mut TrapFrame);
        }
        ExceptionHandler(trap_frame);

        return;
    }

    extern "C" {
        pub fn _start_trap_atomic_rust(trap_frame: *mut riscv_atomic_emulation_trap::TrapFrame);
    }

    let mut atomic_emulation_trap_frame = riscv_atomic_emulation_trap::TrapFrame {
        x0: 0,
        ra: (*trap_frame).ra,
        sp: (*trap_frame).sp,
        gp: (*trap_frame).gp,
        tp: (*trap_frame).tp,
        t0: (*trap_frame).t0,
        t1: (*trap_frame).t1,
        t2: (*trap_frame).t2,
        fp: (*trap_frame).s0,
        s1: (*trap_frame).s1,
        a0: (*trap_frame).a0,
        a1: (*trap_frame).a1,
        a2: (*trap_frame).a2,
        a3: (*trap_frame).a3,
        a4: (*trap_frame).a4,
        a5: (*trap_frame).a5,
        a6: (*trap_frame).a6,
        a7: (*trap_frame).a7,
        s2: (*trap_frame).s2,
        s3: (*trap_frame).s3,
        s4: (*trap_frame).s4,
        s5: (*trap_frame).s5,
        s6: (*trap_frame).s6,
        s7: (*trap_frame).s7,
        s8: (*trap_frame).s8,
        s9: (*trap_frame).s9,
        s10: (*trap_frame).s10,
        s11: (*trap_frame).s11,
        t3: (*trap_frame).t3,
        t4: (*trap_frame).t4,
        t5: (*trap_frame).t5,
        t6: (*trap_frame).t6,
        pc: (*trap_frame).pc,
    };

    _start_trap_atomic_rust(&mut atomic_emulation_trap_frame);

    (*trap_frame).pc = atomic_emulation_trap_frame.pc;
    (*trap_frame).ra = atomic_emulation_trap_frame.ra;
    (*trap_frame).sp = atomic_emulation_trap_frame.sp;
    (*trap_frame).gp = atomic_emulation_trap_frame.gp;
    (*trap_frame).tp = atomic_emulation_trap_frame.tp;
    (*trap_frame).t0 = atomic_emulation_trap_frame.t0;
    (*trap_frame).t1 = atomic_emulation_trap_frame.t1;
    (*trap_frame).t2 = atomic_emulation_trap_frame.t2;
    (*trap_frame).s0 = atomic_emulation_trap_frame.fp;
    (*trap_frame).s1 = atomic_emulation_trap_frame.s1;
    (*trap_frame).a0 = atomic_emulation_trap_frame.a0;
    (*trap_frame).a1 = atomic_emulation_trap_frame.a1;
    (*trap_frame).a2 = atomic_emulation_trap_frame.a2;
    (*trap_frame).a3 = atomic_emulation_trap_frame.a3;
    (*trap_frame).a4 = atomic_emulation_trap_frame.a4;
    (*trap_frame).a5 = atomic_emulation_trap_frame.a5;
    (*trap_frame).a6 = atomic_emulation_trap_frame.a6;
    (*trap_frame).a7 = atomic_emulation_trap_frame.a7;
    (*trap_frame).s2 = atomic_emulation_trap_frame.s2;
    (*trap_frame).s3 = atomic_emulation_trap_frame.s3;
    (*trap_frame).s4 = atomic_emulation_trap_frame.s4;
    (*trap_frame).s5 = atomic_emulation_trap_frame.s5;
    (*trap_frame).s6 = atomic_emulation_trap_frame.s6;
    (*trap_frame).s7 = atomic_emulation_trap_frame.s7;
    (*trap_frame).s8 = atomic_emulation_trap_frame.s8;
    (*trap_frame).s9 = atomic_emulation_trap_frame.s9;
    (*trap_frame).s10 = atomic_emulation_trap_frame.s10;
    (*trap_frame).s11 = atomic_emulation_trap_frame.s11;
    (*trap_frame).t3 = atomic_emulation_trap_frame.t3;
    (*trap_frame).t4 = atomic_emulation_trap_frame.t4;
    (*trap_frame).t5 = atomic_emulation_trap_frame.t5;
    (*trap_frame).t6 = atomic_emulation_trap_frame.t6;
}

#[doc(hidden)]
#[no_mangle]
pub fn _setup_interrupts() {
    extern "C" {
        static _vector_table_hal: *const u32;
    }

    unsafe {
        let vec_table = &_vector_table_hal as *const _ as usize;
        riscv::register::mtvec::write(vec_table, riscv::register::mtvec::TrapMode::Vectored);

        #[cfg(feature = "vectored")]
        crate::interrupt::init_vectoring();
    };
}
