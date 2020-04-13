use core::fmt::{Error, Write};
use core::convert::TryInto;

pub struct Uart
{
    base_address: usize,
}

impl Uart
{
    pub fn new(base_address: usize) -> Self
    {
        Uart
        {
            base_address
        }
    }

    pub fn init(&mut self)
    {
        let ptr = self.base_address as *mut u8;
        unsafe
        {
            // Set word length, bits 0 and 1 of line control register (LCR), at base_addr + 3
            let lcr = (1 << 0) | (1 << 1);
            ptr.add(3).write_volatile(lcr);
 
            // Enable FIFO, bit index 0 of FIFO control register (FCR), at base_addr + 2
            ptr.add(2).write_volatile(1 << 0);
 
            // Enable receiver buffer interrupts, bit index 0 of interrupt enable register (IER),
            // at base_addr + 2
            ptr.add(1).write_volatile(1 << 0);
 
            // 2400 BAUD
            // divisor = ceil( (clock_hz) / (baud_sps x 16) )
            // divisor = ceil( 22_729_000 / (2400 x 16) )
            // divisor = ceil( 22_729_000 / 38_400 )
            // divisor = ceil( 591.901 ) = 592
            let divisor: u16 = 592;
            let divisor_least: u8 = (divisor & 0xff).try_into().unwrap();
            let divisor_most: u8 = (divisor >> 8).try_into().unwrap();
 
            // Write 1 to Divisor Latch Access Bit (DLAB), index 7 of LCR
            ptr.add(3).write_volatile(lcr | 1 << 7);
 
            // Divisor latch least (DLL) now at offset 0
            ptr.add(0).write_volatile(divisor_least);
            // Divisor latch most (DLM) now at offset 1
            ptr.add(1).write_volatile(divisor_most);
 
            // Clear DLAB
            ptr.add(3).write_volatile(lcr);
        }
    }

    pub fn get(&mut self) -> Option<u8>
    {
        let ptr = self.base_address as *mut u8;
        unsafe
        {
            // index 5 is LCR
            if ptr.add(5).read_volatile() & 1 == 0
            {
                None
            }else
            {
                Some(ptr.add(0).read_volatile())
            }
        }
    }

    pub fn put(&mut self, c: u8)
    {
        let ptr = self.base_address as *mut u8;
        unsafe
        {
            ptr.add(0).write_volatile(c);
        }
    }
}

impl Write for Uart
{
    fn write_str(&mut self, s: &str) -> Result<(), Error>
    {
        for c in s.bytes()
        {
            self.put(c);
        }

        Ok(())
    }
}
