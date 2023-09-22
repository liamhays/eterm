use std::io::Write;
use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use std::error::Error;
use std::thread;
use std::time::Duration;

const SCREEN_BYTES: usize = 5642;

struct EPD266 {
    spi: Spidev,
    dc: LineHandle,
    reset: LineHandle,
    busy: LineHandle,
}

impl EPD266 {
    pub fn new() -> Result<EPD266, Box<dyn Error>> {
	let mut spi_handle = Spidev::open("/dev/spidev1.0")?;
	let options = SpidevOptions::new()
	    .bits_per_word(8)
	    .max_speed_hz(4_000_000)
	    .mode(SpiModeFlags::SPI_MODE_0)
	    .build();
	spi_handle.configure(&options)?;

	// Pinout: D/C = GPIO3_B0, RESET = GPIO3_B1, BUSY = GPIO3_B2
	let mut gpiochip = Chip::new("/dev/gpiochip96")?;

	// On Rockchip, A/B/C/D groups of a GPIO block are 8 pins
	// B0, then is pin 8 (0-indexed) of GPIO3
	let dc_handle = gpiochip.get_line(8)?
	    .request(LineRequestFlags::OUTPUT, 0, "d/c")?;

	let reset_handle = gpiochip.get_line(9)?
	    .request(LineRequestFlags::OUTPUT, 0, "reset")?;

	let busy_handle = gpiochip.get_line(10)?
	    .request(LineRequestFlags::INPUT, 0, "busy")?;
	
	Ok(EPD266 {
	    spi: spi_handle,
	    dc: dc_handle,
	    reset: reset_handle,
	    busy: busy_handle,
	})
    }

    pub fn hard_reset(&self) -> Result<(), Box<dyn Error>> {
	thread::sleep(Duration::from_millis(5));
	self.reset.set_value(1)?;
	thread::sleep(Duration::from_millis(5));
	self.reset.set_value(0)?;
	thread::sleep(Duration::from_millis(10));
	self.reset.set_value(1)?;
	thread::sleep(Duration::from_millis(10));

	Ok(())
    }

    fn write_register(&mut self, reg: u8, data: &[u8]) -> Result<(), Box<dyn Error>> {
	self.dc.set_value(0)?;
	self.spi.write(&[reg])?;
	
	self.dc.set_value(1)?;
	self.spi.write(data)?;

	Ok(())
    }
	    
    pub fn full_update(&mut self, image: &[u8]) -> Result<(), Box<dyn Error>> {
	// Step 1: hard reset
	self.hard_reset()?;

	// Step 2: soft reset
	self.write_register(0x00, &[0x0e])?;

	// Step 3: set temperature (25 C)
	self.write_register(0xe5, &[25])?;

	// Step 4: set active temperature
	self.write_register(0xe0, &[0x02])?;

	// Step 5: set PSR
	self.write_register(0x00, &[0xcf, 0x8d])?;

	// Step 6: set data
	self.write_register(0x10, image)?;
	// empty frame, still have to write it
	self.write_register(0x13, &[0; SCREEN_BYTES])?;

	// Step 7: refresh
	
	// you don't have to actually write any data---setting the
	// register is enough---but that requires a second function.
	self.write_register(0x04, &[0x00])?; // turn on DC/DC
	while self.busy.get_value()? != 1 {}

	self.write_register(0x12, &[0x00])?; // refresh display
	while self.busy.get_value()? != 1 {}

	self.write_register(0x02, &[0x00])?; // turn off DC/DC
	while self.busy.get_value()? != 1 {}

	
	Ok(())
    }

    pub fn fast_update(&mut self, old_image: &[u8], new_image: &[u8]) -> Result<(), Box<dyn Error>> {
	// User must hard reset on their own

	// Step 1: soft reset
	self.write_register(0x00, &[0x0e])?;

	// Step 2: set temerature (25 C)
	self.write_register(0xe5, &[25])?;

	// Step 3: set active temperature
	self.write_register(0xe0, &[0x02])?;

	// Step 4: set PSR
	self.write_register(0x00, &[0xcf, 0x8d])?;
	self.write_register(0x00, &[0xff, 0x8f])?;

	// Step 5: "Vcom and data interval setting" and other things
	self.write_register(0x50, &[0x07])?; 
	self.write_register(0x30, &[0x0c])?;
	self.write_register(0x82, &[0x11])?;

	// A bunch of strings of data
	self.write_register(0x20, &[0x01,0x00,0x05,0x05,0x01,0x09,0x01,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00])?;
	self.write_register(0x23, &[0x01,0x55,0x05,0x05,0x01,0x09,0x01,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00])?;
	self.write_register(0x22, &[0x01,0xAA,0x05,0x05,0x01,0x09,0x01,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00])?;
	self.write_register(0x21, &[0x01,0x02,0x05,0x05,0x01,0x09,0x01,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00])?;
	self.write_register(0x24, &[0x01,0x01,0x05,0x05,0x01,0x09,0x01,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00])?;

	// Step 6: send data
	self.write_register(0x10, old_image)?;
	self.write_register(0x13, new_image)?;

	// Step 7: refresh
	self.write_register(0x04, &[0x00])?;
	while self.busy.get_value()? != 1 {}

	self.write_register(0x12, &[0x00])?;
	while self.busy.get_value()? != 1 {}

	// optionally turn off DC/DC

	Ok(())
    }
}
    

fn main() {
    let mut epd = EPD266::new().unwrap();
    let image = [0; SCREEN_BYTES];

    epd.full_update(&image).unwrap();
}
