#![no_std]
#![feature(panic_info_message, asm)]

pub mod uart;

#[macro_export]
macro_rules! print
{
    ($($args:tt)+) =>
    ({
        use core::fmt::Write;
        let _ = write!(crate::uart::Uart::new(0x1000_0000), $($args)+);
    });
}

#[macro_export]
macro_rules! println
{
    () => 
    ({
        print!("\r\n")
    });
    ($fmt:expr) =>
    ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => 
    ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}

#[no_mangle]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> !
{
    print!("Aborting: ");
    if let Some(p) = info.location()
    {
        println!("line {}, file {}: {}", p.line(), p.file(), info.message().unwrap());
    }else
    {
        println!("no info available");
    }

    abort();
}

#[no_mangle]
extern "C"
fn abort() -> !
{
    loop
    {
        unsafe
        {
            asm!("wfi"::::"volatile");
        }
    }
}

#[no_mangle]
extern "C"
fn kmain()
{
    let mut my_uart = uart::Uart::new(0x1000_0000);
    my_uart.init();

    println!("This is seraph_riscv");
    println!("input will be echoed");

    loop
    {
        if let Some(c) = my_uart.get()
        {
            match c
            {
                8 =>
                {
                    print!("{}{}{}", 8 as char, ' ', 8 as char);
                },
                10 | 13 =>
                {
                    println!();
                },
                _ =>
                {
                    print!("{}", c as char);
                }
            }
        }
    }
}
