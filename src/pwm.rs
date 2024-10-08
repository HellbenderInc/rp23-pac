#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::ChCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub const fn div(self) -> crate::common::Reg<regs::ChDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::ChCtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub const fn cc(self) -> crate::common::Reg<regs::ChCc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub const fn top(self) -> crate::common::Reg<regs::ChTop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
}
#[doc = "Simple PWM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm {
    ptr: *mut u8,
}
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Channel {
        assert!(n < 12usize);
        unsafe { Channel::from_ptr(self.ptr.add(0usize + n * 20usize) as _) }
    }
    #[doc = "This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
    #[inline(always)]
    pub const fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "Interrupt Enable for irq0"]
    #[inline(always)]
    pub const fn irq0_inte(self) -> crate::common::Reg<regs::Irq0inte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(248usize) as _) }
    }
    #[doc = "Interrupt Force for irq0"]
    #[inline(always)]
    pub const fn irq0_intf(self) -> crate::common::Reg<regs::Irq0intf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing for irq0"]
    #[inline(always)]
    pub const fn irq0_ints(self) -> crate::common::Reg<regs::Irq0ints, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "Interrupt Enable for irq1"]
    #[inline(always)]
    pub const fn irq1_inte(self) -> crate::common::Reg<regs::Irq1inte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "Interrupt Force for irq1"]
    #[inline(always)]
    pub const fn irq1_intf(self) -> crate::common::Reg<regs::Irq1intf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing for irq1"]
    #[inline(always)]
    pub const fn irq1_ints(self) -> crate::common::Reg<regs::Irq1ints, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(268usize) as _) }
    }
}
pub mod regs;
pub mod vals;
