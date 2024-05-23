#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "UART register block, DW_apb_uart"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}
impl Uart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Divisor latch low. when LCR\\[7\\]
bit = 1"]
    #[inline(always)]
    pub const fn dll(self) -> crate::common::Reg<regs::Dll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Receiver buffer register. when LCR\\[7\\]
bit = 0"]
    #[inline(always)]
    pub const fn rbr(self) -> crate::common::Reg<regs::Rbr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Transmitter holding register. when LCR\\[7\\]
bit = 0"]
    #[inline(always)]
    pub const fn thr(self) -> crate::common::Reg<regs::Thr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Divisor latch high. when LCR\\[7\\]
bit = 1"]
    #[inline(always)]
    pub const fn dlh(self) -> crate::common::Reg<regs::Dlh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Interrupt enable register. when LCR\\[7\\]
bit = 0"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FIFO control register."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Interrupt identification register."]
    #[inline(always)]
    pub const fn iir(self) -> crate::common::Reg<regs::Iir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Line control register."]
    #[inline(always)]
    pub const fn lcr(self) -> crate::common::Reg<regs::Lcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Modem control register."]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Line status register."]
    #[inline(always)]
    pub const fn lsr(self) -> crate::common::Reg<regs::Lsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Modem status register."]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Scratch register."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Low power divisor latch low."]
    #[inline(always)]
    pub const fn lpdll(self) -> crate::common::Reg<regs::Dll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Low power divisor latch high."]
    #[inline(always)]
    pub const fn lpdlh(self) -> crate::common::Reg<regs::Dlh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Shadow receiver buffer register."]
    #[inline(always)]
    pub const fn srbr(self, n: usize) -> crate::common::Reg<regs::Rbr, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "Shadow transmitter holding register."]
    #[inline(always)]
    pub const fn sthr(self, n: usize) -> crate::common::Reg<regs::Thr, crate::common::W> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "FIFO access register."]
    #[inline(always)]
    pub const fn far(self) -> crate::common::Reg<regs::Far, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Transmit FIFO read."]
    #[inline(always)]
    pub const fn tfr(self) -> crate::common::Reg<regs::Tfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Receive FIFO write."]
    #[inline(always)]
    pub const fn rfw(self) -> crate::common::Reg<regs::Rfw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "UART status register."]
    #[inline(always)]
    pub const fn usr(self) -> crate::common::Reg<regs::Usr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Transmit FIFO level."]
    #[inline(always)]
    pub const fn tfl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Receive FIFO level."]
    #[inline(always)]
    pub const fn rfl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Software reset register."]
    #[inline(always)]
    pub const fn srr(self) -> crate::common::Reg<regs::Srr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Shadow request to send."]
    #[inline(always)]
    pub const fn srts(self) -> crate::common::Reg<regs::Srts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Shadow break control register."]
    #[inline(always)]
    pub const fn sbcr(self) -> crate::common::Reg<regs::Sbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Shadow DMA mode."]
    #[inline(always)]
    pub const fn sdmam(self) -> crate::common::Reg<regs::Sdmam, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Shadow FIFO enable."]
    #[inline(always)]
    pub const fn sfe(self) -> crate::common::Reg<regs::Sfe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Shadow RCVR trigger."]
    #[inline(always)]
    pub const fn srt(self) -> crate::common::Reg<regs::Srt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Shadow TX empty trigger."]
    #[inline(always)]
    pub const fn stet(self) -> crate::common::Reg<regs::Stet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "Halt TX."]
    #[inline(always)]
    pub const fn htx(self) -> crate::common::Reg<regs::Htx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "DMA software acknowledge."]
    #[inline(always)]
    pub const fn dmasa(self) -> crate::common::Reg<regs::Dmasa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "Component parameter register."]
    #[inline(always)]
    pub const fn cpr(self) -> crate::common::Reg<regs::Cpr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "UART component version."]
    #[inline(always)]
    pub const fn ucv(self) -> crate::common::Reg<regs::Ucv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "Component type register."]
    #[inline(always)]
    pub const fn ctr(self) -> crate::common::Reg<regs::Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod common {
    use core::marker::PhantomData;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct RW;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct R;
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct W;
    mod sealed {
        use super::*;
        pub trait Access {}
        impl Access for R {}
        impl Access for W {}
        impl Access for RW {}
    }
    pub trait Access: sealed::Access + Copy {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    pub trait Read: Access {}
    impl Read for RW {}
    impl Read for R {}
    pub trait Write: Access {}
    impl Write for RW {}
    impl Write for W {}
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct Reg<T: Copy, A: Access> {
        ptr: *mut u8,
        phantom: PhantomData<*mut (T, A)>,
    }
    unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
    unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}
    impl<T: Copy, A: Access> Reg<T, A> {
        #[allow(clippy::missing_safety_doc)]
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
            Self {
                ptr: ptr as _,
                phantom: PhantomData,
            }
        }
        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut T {
            self.ptr as _
        }
    }
    impl<T: Copy, A: Read> Reg<T, A> {
        #[inline(always)]
        pub fn read(&self) -> T {
            unsafe { (self.ptr as *mut T).read_volatile() }
        }
    }
    impl<T: Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write_value(&self, val: T) {
            unsafe { (self.ptr as *mut T).write_volatile(val) }
        }
    }
    impl<T: Default + Copy, A: Write> Reg<T, A> {
        #[inline(always)]
        pub fn write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = Default::default();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
    impl<T: Copy, A: Read + Write> Reg<T, A> {
        #[inline(always)]
        pub fn modify<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let mut val = self.read();
            let res = f(&mut val);
            self.write_value(val);
            res
        }
    }
}
pub mod regs {
    #[doc = "Component parameter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cpr(pub u32);
    impl Cpr {
        #[doc = "APB data width"]
        #[inline(always)]
        pub const fn apb_data_width(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "APB data width"]
        #[inline(always)]
        pub fn set_apb_data_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Auto flow control enable mode"]
        #[inline(always)]
        pub const fn afce_mode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Auto flow control enable mode"]
        #[inline(always)]
        pub fn set_afce_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "THRE mode"]
        #[inline(always)]
        pub const fn thre_mode(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "THRE mode"]
        #[inline(always)]
        pub fn set_thre_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SIR mode"]
        #[inline(always)]
        pub const fn sir_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SIR mode"]
        #[inline(always)]
        pub fn set_sir_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SIR low power mode"]
        #[inline(always)]
        pub const fn sir_lp_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SIR low power mode"]
        #[inline(always)]
        pub fn set_sir_lp_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Additional features"]
        #[inline(always)]
        pub const fn additional_feat(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Additional features"]
        #[inline(always)]
        pub fn set_additional_feat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FIFO access"]
        #[inline(always)]
        pub const fn fifo_access(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO access"]
        #[inline(always)]
        pub fn set_fifo_access(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FIFO status"]
        #[inline(always)]
        pub const fn fifo_stat(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO status"]
        #[inline(always)]
        pub fn set_fifo_stat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Shadow"]
        #[inline(always)]
        pub const fn shadow(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Shadow"]
        #[inline(always)]
        pub fn set_shadow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "UART additional encoded parameters"]
        #[inline(always)]
        pub const fn uart_add_encoded_params(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "UART additional encoded parameters"]
        #[inline(always)]
        pub fn set_uart_add_encoded_params(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DMA extra"]
        #[inline(always)]
        pub const fn dma_extra(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DMA extra"]
        #[inline(always)]
        pub fn set_dma_extra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "FIFO mode"]
        #[inline(always)]
        pub const fn fifo_mode(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "FIFO mode"]
        #[inline(always)]
        pub fn set_fifo_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Cpr {
        #[inline(always)]
        fn default() -> Cpr {
            Cpr(0)
        }
    }
    #[doc = "Component type register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctr(pub u32);
    impl Ctr {
        #[doc = "Peripherals identification code"]
        #[inline(always)]
        pub const fn pid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Peripherals identification code"]
        #[inline(always)]
        pub fn set_pid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ctr {
        #[inline(always)]
        fn default() -> Ctr {
            Ctr(0)
        }
    }
    #[doc = "Divisor latch high"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlh(pub u32);
    impl Dlh {
        #[doc = "Data"]
        #[inline(always)]
        pub const fn dlh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data"]
        #[inline(always)]
        pub fn set_dlh(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dlh {
        #[inline(always)]
        fn default() -> Dlh {
            Dlh(0)
        }
    }
    #[doc = "Divisor latch low"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dll(pub u32);
    impl Dll {
        #[doc = "Data"]
        #[inline(always)]
        pub const fn dll(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data"]
        #[inline(always)]
        pub fn set_dll(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dll {
        #[inline(always)]
        fn default() -> Dll {
            Dll(0)
        }
    }
    #[doc = "DMA software acknowledge"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmasa(pub u32);
    impl Dmasa {
        #[doc = "DMA software acknowledge"]
        #[inline(always)]
        pub const fn dmasa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA software acknowledge"]
        #[inline(always)]
        pub fn set_dmasa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Dmasa {
        #[inline(always)]
        fn default() -> Dmasa {
            Dmasa(0)
        }
    }
    #[doc = "FIFO access register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Far(pub u32);
    impl Far {
        #[doc = "FIFO access register"]
        #[inline(always)]
        pub const fn far(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO access register"]
        #[inline(always)]
        pub fn set_far(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Far {
        #[inline(always)]
        fn default() -> Far {
            Far(0)
        }
    }
    #[doc = "FIFO control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "FIFO enable"]
        #[inline(always)]
        pub const fn fifoe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO enable"]
        #[inline(always)]
        pub fn set_fifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RCVR FIFO reset"]
        #[inline(always)]
        pub const fn rfifor(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RCVR FIFO reset"]
        #[inline(always)]
        pub fn set_rfifor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "XMIT FIFO reset"]
        #[inline(always)]
        pub const fn xfifor(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "XMIT FIFO reset"]
        #[inline(always)]
        pub fn set_xfifor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA mode"]
        #[inline(always)]
        pub const fn dmam(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA mode"]
        #[inline(always)]
        pub fn set_dmam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX Empty Trigger"]
        #[inline(always)]
        pub const fn tet(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "TX Empty Trigger"]
        #[inline(always)]
        pub fn set_tet(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "RCVR Trigger"]
        #[inline(always)]
        pub const fn rt(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RCVR Trigger"]
        #[inline(always)]
        pub fn set_rt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    #[doc = "Halt TX"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htx(pub u32);
    impl Htx {
        #[doc = "Halt TX"]
        #[inline(always)]
        pub const fn htx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Halt TX"]
        #[inline(always)]
        pub fn set_htx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Htx {
        #[inline(always)]
        fn default() -> Htx {
            Htx(0)
        }
    }
    #[doc = "Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Enable received data available interrupt"]
        #[inline(always)]
        pub const fn erbfi(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable received data available interrupt"]
        #[inline(always)]
        pub fn set_erbfi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable transmitter holding register empty interrupt"]
        #[inline(always)]
        pub const fn etbei(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable transmitter holding register empty interrupt"]
        #[inline(always)]
        pub fn set_etbei(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable receiver line status interrupt"]
        #[inline(always)]
        pub const fn elsi(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable receiver line status interrupt"]
        #[inline(always)]
        pub fn set_elsi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable modem status interrupt"]
        #[inline(always)]
        pub const fn edssi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable modem status interrupt"]
        #[inline(always)]
        pub fn set_edssi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Programmable THRE Interrupt Mode Enable"]
        #[inline(always)]
        pub const fn ptime(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable THRE Interrupt Mode Enable"]
        #[inline(always)]
        pub fn set_ptime(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "Interrupt Identity register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iir(pub u32);
    impl Iir {
        #[doc = "Interrupt ID"]
        #[inline(always)]
        pub const fn iid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Interrupt ID"]
        #[inline(always)]
        pub fn set_iid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "FIFOs Enabled"]
        #[inline(always)]
        pub const fn fifose(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "FIFOs Enabled"]
        #[inline(always)]
        pub fn set_fifose(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
    }
    impl Default for Iir {
        #[inline(always)]
        fn default() -> Iir {
            Iir(0)
        }
    }
    #[doc = "Line control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lcr(pub u32);
    impl Lcr {
        #[doc = "Data length select, aka. CLS, DLS"]
        #[inline(always)]
        pub const fn wls(&self) -> super::vals::DataBits {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::DataBits::from_bits(val as u8)
        }
        #[doc = "Data length select, aka. CLS, DLS"]
        #[inline(always)]
        pub fn set_wls(&mut self, val: super::vals::DataBits) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Number of stop bits"]
        #[inline(always)]
        pub const fn stop(&self) -> super::vals::StopBits {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::StopBits::from_bits(val as u8)
        }
        #[doc = "Number of stop bits"]
        #[inline(always)]
        pub fn set_stop(&mut self, val: super::vals::StopBits) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Parity enable"]
        #[inline(always)]
        pub const fn pen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Parity enable"]
        #[inline(always)]
        pub fn set_pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Even parity select"]
        #[inline(always)]
        pub const fn eps(&self) -> super::vals::ParitySelect {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::ParitySelect::from_bits(val as u8)
        }
        #[doc = "Even parity select"]
        #[inline(always)]
        pub fn set_eps(&mut self, val: super::vals::ParitySelect) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Stick parity (reserved in 16550, read as 0)"]
        #[inline(always)]
        pub const fn sp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Stick parity (reserved in 16550, read as 0)"]
        #[inline(always)]
        pub fn set_sp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Break control"]
        #[inline(always)]
        pub const fn bc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Break control"]
        #[inline(always)]
        pub fn set_bc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Divisor latch access bit"]
        #[inline(always)]
        pub const fn dlab(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Divisor latch access bit"]
        #[inline(always)]
        pub fn set_dlab(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Lcr {
        #[inline(always)]
        fn default() -> Lcr {
            Lcr(0)
        }
    }
    #[doc = "Line status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lsr(pub u32);
    impl Lsr {
        #[doc = "Data ready"]
        #[inline(always)]
        pub const fn dr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data ready"]
        #[inline(always)]
        pub fn set_dr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Overrun error"]
        #[inline(always)]
        pub const fn oe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error"]
        #[inline(always)]
        pub fn set_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Parity error"]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error"]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Framing error"]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error"]
        #[inline(always)]
        pub fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Break interrupt"]
        #[inline(always)]
        pub const fn bi(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt"]
        #[inline(always)]
        pub fn set_bi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Transmitter holding register empty"]
        #[inline(always)]
        pub const fn thre(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter holding register empty"]
        #[inline(always)]
        pub fn set_thre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmitter empty"]
        #[inline(always)]
        pub const fn temt(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter empty"]
        #[inline(always)]
        pub fn set_temt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Receiver FIFO error"]
        #[inline(always)]
        pub const fn rfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver FIFO error"]
        #[inline(always)]
        pub fn set_rfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Lsr {
        #[inline(always)]
        fn default() -> Lsr {
            Lsr(0)
        }
    }
    #[doc = "Modem control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "Data terminal ready"]
        #[inline(always)]
        pub const fn dtr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data terminal ready"]
        #[inline(always)]
        pub fn set_dtr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Request to send"]
        #[inline(always)]
        pub const fn rts(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Request to send"]
        #[inline(always)]
        pub fn set_rts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Output 1"]
        #[inline(always)]
        pub const fn out1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Output 1"]
        #[inline(always)]
        pub fn set_out1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Output 2"]
        #[inline(always)]
        pub const fn out2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output 2"]
        #[inline(always)]
        pub fn set_out2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Loopback mode, aka. LOOP"]
        #[inline(always)]
        pub const fn lb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Loopback mode, aka. LOOP"]
        #[inline(always)]
        pub fn set_lb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Auto flow control enable"]
        #[inline(always)]
        pub const fn afce(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Auto flow control enable"]
        #[inline(always)]
        pub fn set_afce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SIR mode enable"]
        #[inline(always)]
        pub const fn sire(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SIR mode enable"]
        #[inline(always)]
        pub fn set_sire(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    #[doc = "Modem status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msr(pub u32);
    impl Msr {
        #[doc = "Delta clear to send"]
        #[inline(always)]
        pub const fn dcts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Delta clear to send"]
        #[inline(always)]
        pub fn set_dcts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Delta data set ready"]
        #[inline(always)]
        pub const fn ddsr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Delta data set ready"]
        #[inline(always)]
        pub fn set_ddsr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Trailing edge ring indicator"]
        #[inline(always)]
        pub const fn teri(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Trailing edge ring indicator"]
        #[inline(always)]
        pub fn set_teri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Delta data carrier detect"]
        #[inline(always)]
        pub const fn ddcd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Delta data carrier detect"]
        #[inline(always)]
        pub fn set_ddcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear to send"]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear to send"]
        #[inline(always)]
        pub fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data set ready"]
        #[inline(always)]
        pub const fn dsr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Data set ready"]
        #[inline(always)]
        pub fn set_dsr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Ring indicator"]
        #[inline(always)]
        pub const fn ri(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Ring indicator"]
        #[inline(always)]
        pub fn set_ri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data carrier detect"]
        #[inline(always)]
        pub const fn dcd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data carrier detect"]
        #[inline(always)]
        pub fn set_dcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Msr {
        #[inline(always)]
        fn default() -> Msr {
            Msr(0)
        }
    }
    #[doc = "Receiver buffer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rbr(pub u32);
    impl Rbr {
        #[doc = "Data"]
        #[inline(always)]
        pub const fn rbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data"]
        #[inline(always)]
        pub fn set_rbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rbr {
        #[inline(always)]
        fn default() -> Rbr {
            Rbr(0)
        }
    }
    #[doc = "Receive FIFO write"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfw(pub u32);
    impl Rfw {
        #[doc = "Receive FIFO Write Data"]
        #[inline(always)]
        pub const fn rfwd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Receive FIFO Write Data"]
        #[inline(always)]
        pub fn set_rfwd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Receive FIFO Parity Error"]
        #[inline(always)]
        pub const fn rfpe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO Parity Error"]
        #[inline(always)]
        pub fn set_rfpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Receive FIFO Frame Error"]
        #[inline(always)]
        pub const fn rffe(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO Frame Error"]
        #[inline(always)]
        pub fn set_rffe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Rfw {
        #[inline(always)]
        fn default() -> Rfw {
            Rfw(0)
        }
    }
    #[doc = "Shadow break control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sbcr(pub u32);
    impl Sbcr {
        #[doc = "Shadow break control register"]
        #[inline(always)]
        pub const fn sbcr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Shadow break control register"]
        #[inline(always)]
        pub fn set_sbcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sbcr {
        #[inline(always)]
        fn default() -> Sbcr {
            Sbcr(0)
        }
    }
    #[doc = "Scratch register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Data"]
        #[inline(always)]
        pub const fn scr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data"]
        #[inline(always)]
        pub fn set_scr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    #[doc = "Shadow DMA mode"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sdmam(pub u32);
    impl Sdmam {
        #[doc = "Shadow DMA mode"]
        #[inline(always)]
        pub const fn sdmam(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Shadow DMA mode"]
        #[inline(always)]
        pub fn set_sdmam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sdmam {
        #[inline(always)]
        fn default() -> Sdmam {
            Sdmam(0)
        }
    }
    #[doc = "Shadow FIFO enable"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sfe(pub u32);
    impl Sfe {
        #[doc = "Shadow FIFO enable"]
        #[inline(always)]
        pub const fn sfe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Shadow FIFO enable"]
        #[inline(always)]
        pub fn set_sfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sfe {
        #[inline(always)]
        fn default() -> Sfe {
            Sfe(0)
        }
    }
    #[doc = "Shadow receiver buffer register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srbr(pub u32);
    impl Srbr {
        #[doc = "Data"]
        #[inline(always)]
        pub const fn srbr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data"]
        #[inline(always)]
        pub fn set_srbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Srbr {
        #[inline(always)]
        fn default() -> Srbr {
            Srbr(0)
        }
    }
    #[doc = "Software reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srr(pub u32);
    impl Srr {
        #[doc = "UART reset"]
        #[inline(always)]
        pub const fn ur(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UART reset"]
        #[inline(always)]
        pub fn set_ur(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RCVR FIFO reset"]
        #[inline(always)]
        pub const fn rfr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RCVR FIFO reset"]
        #[inline(always)]
        pub fn set_rfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "XMIT FIFO reset"]
        #[inline(always)]
        pub const fn xfr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "XMIT FIFO reset"]
        #[inline(always)]
        pub fn set_xfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Srr {
        #[inline(always)]
        fn default() -> Srr {
            Srr(0)
        }
    }
    #[doc = "Shadow RCVR trigger"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srt(pub u32);
    impl Srt {
        #[doc = "Shadow RCVR trigger"]
        #[inline(always)]
        pub const fn srt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Shadow RCVR trigger"]
        #[inline(always)]
        pub fn set_srt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Srt {
        #[inline(always)]
        fn default() -> Srt {
            Srt(0)
        }
    }
    #[doc = "Shadow request to send"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srts(pub u32);
    impl Srts {
        #[doc = "Shadow request to send"]
        #[inline(always)]
        pub const fn srts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Shadow request to send"]
        #[inline(always)]
        pub fn set_srts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Srts {
        #[inline(always)]
        fn default() -> Srts {
            Srts(0)
        }
    }
    #[doc = "Shadow TX empty trigger"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stet(pub u32);
    impl Stet {
        #[doc = "Shadow TX empty trigger"]
        #[inline(always)]
        pub const fn stet(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Shadow TX empty trigger"]
        #[inline(always)]
        pub fn set_stet(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Stet {
        #[inline(always)]
        fn default() -> Stet {
            Stet(0)
        }
    }
    #[doc = "Shadow transmitter holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sthr(pub u32);
    impl Sthr {
        #[doc = "Data"]
        #[inline(always)]
        pub const fn sthr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data"]
        #[inline(always)]
        pub fn set_sthr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Sthr {
        #[inline(always)]
        fn default() -> Sthr {
            Sthr(0)
        }
    }
    #[doc = "Transmit FIFO read"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tfr(pub u32);
    impl Tfr {
        #[doc = "Data"]
        #[inline(always)]
        pub const fn tfr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data"]
        #[inline(always)]
        pub fn set_tfr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Tfr {
        #[inline(always)]
        fn default() -> Tfr {
            Tfr(0)
        }
    }
    #[doc = "Transmitter holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Thr(pub u32);
    impl Thr {
        #[doc = "Data"]
        #[inline(always)]
        pub const fn thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data"]
        #[inline(always)]
        pub fn set_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Thr {
        #[inline(always)]
        fn default() -> Thr {
            Thr(0)
        }
    }
    #[doc = "UART component version"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ucv(pub u32);
    impl Ucv {
        #[doc = "UART component version"]
        #[inline(always)]
        pub const fn ucv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "UART component version"]
        #[inline(always)]
        pub fn set_ucv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ucv {
        #[inline(always)]
        fn default() -> Ucv {
            Ucv(0)
        }
    }
    #[doc = "UART status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usr(pub u32);
    impl Usr {
        #[doc = "Busy"]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit FIFO Not Full"]
        #[inline(always)]
        pub const fn tfnf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO Not Full"]
        #[inline(always)]
        pub fn set_tfnf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transmit FIFO Empty"]
        #[inline(always)]
        pub const fn tfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO Empty"]
        #[inline(always)]
        pub fn set_tfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Receive FIFO Not Empty"]
        #[inline(always)]
        pub const fn rfne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO Not Empty"]
        #[inline(always)]
        pub fn set_rfne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive FIFO Full"]
        #[inline(always)]
        pub const fn rff(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO Full"]
        #[inline(always)]
        pub fn set_rff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Usr {
        #[inline(always)]
        fn default() -> Usr {
            Usr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum DataBits {
        #[doc = "5 bits"]
        BIT5 = 0x0,
        #[doc = "6 bits"]
        BIT6 = 0x01,
        #[doc = "7 bits"]
        BIT7 = 0x02,
        #[doc = "8 bits"]
        BIT8 = 0x03,
    }
    impl DataBits {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DataBits {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DataBits {
        #[inline(always)]
        fn from(val: u8) -> DataBits {
            DataBits::from_bits(val)
        }
    }
    impl From<DataBits> for u8 {
        #[inline(always)]
        fn from(val: DataBits) -> u8 {
            DataBits::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ParitySelect {
        #[doc = "Odd parity (one)"]
        ODD = 0x0,
        #[doc = "Even parity (zero)"]
        EVEN = 0x01,
    }
    impl ParitySelect {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ParitySelect {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ParitySelect {
        #[inline(always)]
        fn from(val: u8) -> ParitySelect {
            ParitySelect::from_bits(val)
        }
    }
    impl From<ParitySelect> for u8 {
        #[inline(always)]
        fn from(val: ParitySelect) -> u8 {
            ParitySelect::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum StopBits {
        #[doc = "1 stop bit"]
        STOP1 = 0x0,
        #[doc = "2 stop bits (1.5 when 5 bits)"]
        STOP2 = 0x01,
    }
    impl StopBits {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> StopBits {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for StopBits {
        #[inline(always)]
        fn from(val: u8) -> StopBits {
            StopBits::from_bits(val)
        }
    }
    impl From<StopBits> for u8 {
        #[inline(always)]
        fn from(val: StopBits) -> u8 {
            StopBits::to_bits(val)
        }
    }
}
