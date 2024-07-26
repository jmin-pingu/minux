use core::fmt::{Write, Error};
// Information about the NS16550A UART chipset: http://byterunner.com/16550.html

// Useful constants for working with the NS16550A UART chip
const RHR: u8 = 0;                  // receive holding register (for input bytes)
const THR: u8 = 0;                 // transmit holding register (for output bytes) 
const IER: u8 = 1;                 // interrupt enable register
const IER_RX_ENABLE: u8 = 1<<0;
const IER_TX_ENABLE: u8 = 1<<1;
const FCR: u8 = 2;                 // FIFO control register
const FCR_FIFO_ENABLE: u8 = 1<<0;
const FCR_FIFO_CLEAR: u8 = 3<<1; // clear the content of the two FIFOs
const ISR: u8 = 2;                 // interrupt status register
const LCR: u8 = 3;                 // line control register
const LCR_WORD_LEN_5: u8 = 1<<0 | 1<<1;
const LCR_EIGHT_BITS: u8 = 3<<0;
const LCR_BAUD_LATCH: u8 = 1<<7; // special mode to set baud rate
const LSR: u8 = 5;                 // line status register
const LSR_RX_READY: u8 = 1<<0;   // input is waiting to be read from RHR
const LSR_TX_IDLE: u8 = 1<<5;    // THR can accept another character to send
                                                 
pub struct UartDriver {
    base_address: usize,
}

const UART_TX_BUF_SIZE: u8 = 32;


impl UartDriver {
    pub fn new(base_address: usize) -> Self {
        UartDriver {
            base_address
        }
    }

    // TODO: research BAUD rate
	pub fn init(&mut self) {
		unsafe {
			// Set the word length for LCR
			self.getreg(LCR).write_volatile(LCR_WORD_LEN_5);

			// Enable the FIFO
			self.getreg(FCR).write_volatile(FCR_FIFO_ENABLE);

			// Enable receiver buffer interrupts            
            self.getreg(IER).write_volatile(IER_TX_ENABLE | IER_RX_ENABLE);

			// If we cared about the divisor, the code below would set the divisor
			// from a global clock rate of 22.729 MHz (22,729,000 cycles per second)
			// to a signaling rate of 2400 (BAUD). We usually have much faster signalling
			// rates nowadays, but this demonstrates what the divisor actually does.
			// The formula given in the NS16500A specification for calculating the divisor
			// is:
			// divisor = ceil( (clock_hz) / (baud_sps x 16) )
			// So, we substitute our values and get:
			// divisor = ceil( 22_729_000 / (2400 x 16) )
			// divisor = ceil( 22_729_000 / 38_400 )
			// divisor = ceil( 591.901 ) = 592
			// The divisor register is two bytes (16 bits), so we need to split the value
			// 592 into two bytes. Typically, we would calculate this based on measuring
			// the clock rate, but again, for our purposes [qemu], this doesn't really do
			// anything.
			let divisor: u16 = 592;
			let divisor_least: u8 = (divisor & 0xff).try_into().unwrap();
			let divisor_most:  u8 = (divisor >> 8).try_into().unwrap();

            // Enable internal baud rate counter latch (DLAB: Divisor Latch Access Bit)
	    	self.getreg(LCR).write_volatile(self.getreg(LCR).read_volatile() | LCR_BAUD_LATCH);

            // When the DLAB is enabled, the THR becomes the LSB (least significant bit) of the Divisor Latch 
            // and the IER becomes the MSB (most significant bit) of the Divisor Latch
			self.getreg(THR).write_volatile(divisor_least);
			self.getreg(IER).write_volatile(divisor_most);

			// Disable internal baud rate counter latch, to get access to RHR, THR and IER
			self.getreg(LCR).write_volatile(self.getreg(LCR).read_volatile() & !LCR_BAUD_LATCH);
		}
	}

    // #define Reg(index) ((volatile unsigned char *)(UART0 + reg))
    fn getreg(&self, index: u8) -> *mut u8 {
        let ptr = self.base_address as *mut u8; // TODO: if we define Uart struct, change this
                                   // Add panic condition is reg is too large
        if index > 7 {
            panic!("index error: UART only has 3 bits of space")
        }

        unsafe {
            ptr.add(index as usize) 
        }
    }

    pub fn uart_getc(&self) -> Option<u8> {
        unsafe {
            // If the UART receiver is ready (indicated by appropriate LSR bit), get a character
            if self.getreg(LSR).read_volatile() & LSR_RX_READY == 0 {
                None
            } else {
                Some(self.getreg(RHR).read_volatile())
            }
        }
    }

    pub fn uart_putc(&self, c: u8) {
        unsafe {
            // If the UART transmitter is idle (indicated by appropriate LSR bit), send a character
            if self.getreg(LSR).read_volatile() & LSR_TX_IDLE == 0 {

            } else {
                self.getreg(THR).write_volatile(c);
            }
        }
    }

}

// TODO: look into Write trait and whether this makes sense
impl Write for UartDriver {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        // TODO: implement buffer once we have interrupts
        s.bytes().for_each(|c| self.uart_putc(c));
        Ok(())
    }

}

// // the transmit output buffer.
// char uart_tx_buf[UART_TX_BUF_SIZE];
// uint64 uart_tx_w; // write next to uart_tx_buf[uart_tx_w % UART_TX_BUF_SIZE]
// uint64 uart_tx_r; // read next from uart_tx_buf[uart_tx_r % UART_TX_BUF_SIZE]
// 
// extern volatile int panicked; // from printf.c
// 
// void uartstart();
// 
// void
// uartinit(void)
// {
//   // disable interrupts.
//   WriteReg(IER, 0x00);
// 
//   // special mode to set baud rate.
//   WriteReg(LCR, LCR_BAUD_LATCH);
// 
//   // LSB for baud rate of 38.4K.
//   WriteReg(0, 0x03);
// 
//   // MSB for baud rate of 38.4K.
//   WriteReg(1, 0x00);
// 
//   // leave set-baud mode,
//   // and set word length to 8 bits, no parity.
//   WriteReg(LCR, LCR_EIGHT_BITS);
// 
//   // reset and enable FIFOs.
//   WriteReg(FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);
// 
//   // enable transmit and receive interrupts.
//   WriteReg(IER, IER_TX_ENABLE | IER_RX_ENABLE);
// 
//   initlock(&uart_tx_lock, "uart");
// }
// 
// // add a character to the output buffer and tell the
// // UART to start sending if it isn't already.
// // blocks if the output buffer is full.
// // because it may block, it can't be called
// // from interrupts; it's only suitable for use
// // by write().
// void
// uartputc(int c)
// {
//   acquire(&uart_tx_lock);
// 
//   if(panicked){
//     for(;;)
//       ;
//   }
//   while(uart_tx_w == uart_tx_r + UART_TX_BUF_SIZE){
//     // buffer is full.
//     // wait for uartstart() to open up space in the buffer.
//     sleep(&uart_tx_r, &uart_tx_lock);
//   }
//   uart_tx_buf[uart_tx_w % UART_TX_BUF_SIZE] = c;
//   uart_tx_w += 1;
//   uartstart();
//   release(&uart_tx_lock);
// }
// 
// 
// // alternate version of uartputc() that doesn't 
// // use interrupts, for use by kernel printf() and
// // to echo characters. it spins waiting for the uart's
// // output register to be empty.
// void
// uartputc_sync(int c)
// {
//   push_off();
// 
//   if(panicked){
//     for(;;)
//       ;
//   }
// 
//   // wait for Transmit Holding Empty to be set in LSR.
//   while((ReadReg(LSR) & LSR_TX_IDLE) == 0)
//     ;
//   WriteReg(THR, c);
// 
//   pop_off();
// }
// 
// // if the UART is idle, and a character is waiting
// // in the transmit buffer, send it.
// // caller must hold uart_tx_lock.
// // called from both the top- and bottom-half.
// void
// uartstart()
// {
//   while(1){
//     if(uart_tx_w == uart_tx_r){
//       // transmit buffer is empty.
//       return;
//     }
//     
//     if((ReadReg(LSR) & LSR_TX_IDLE) == 0){
//       // the UART transmit holding register is full,
//       // so we cannot give it another byte.
//       // it will interrupt when it's ready for a new byte.
//       return;
//     }
//     
//     int c = uart_tx_buf[uart_tx_r % UART_TX_BUF_SIZE];
//     uart_tx_r += 1;
//     
//     // maybe uartputc() is waiting for space in the buffer.
//     wakeup(&uart_tx_r);
//     
//     WriteReg(THR, c);
//   }
// }
// 
// // read one input character from the UART.
// // return -1 if none is waiting.
// int
// uartgetc(void)
// {
//   if(ReadReg(LSR) & 0x01){
//     // input data is ready.
//     return ReadReg(RHR);
//   } else {
//     return -1;
//   }
// }
// 
// // handle a uart interrupt, raised because input has
// // arrived, or the uart is ready for more output, or
// // both. called from devintr().
// void
// uartintr(void)
// {
//   // read and process incoming characters.
//   while(1){
//     int c = uartgetc();
//     if(c == -1)
//       break;
//     consoleintr(c);
//   }
// 
//   // send buffered characters.
//   acquire(&uart_tx_lock);
//   uartstart();
//   release(&uart_tx_lock);
// }
