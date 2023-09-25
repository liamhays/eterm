// Text to 1bpp renderer for hardcoded screen size.

// TODO: better way to do this
use crate::text_renderer::font;

const TEXT_ROWS: usize = 37;
const TEXT_COLS: usize = 25;
const SCREEN_X: usize = 152;
const SCREEN_Y: usize = 296;
const SCREEN_X_BYTES: usize = 19;

const SCREEN_BYTES: usize = 5642;
pub struct TextRenderer {
    text: [[char; TEXT_COLS]; TEXT_ROWS],
}

impl TextRenderer {
    pub fn new() -> TextRenderer {

	let t: [[char; TEXT_COLS]; TEXT_ROWS] = [[' '; TEXT_COLS]; TEXT_ROWS];
	
	TextRenderer {
	    text: t
	}
	
    }

    pub fn set_char(&mut self, x: usize, y: usize, chr: char) -> Result<(), &str> {
	// bounds checking
	if x > TEXT_COLS {
	    return Err("destination column greater than total columns!");
	}
	if y > TEXT_ROWS {
	    return Err("destination row greater than total row!");
	}

	//println!("{:?}", self.text);
	self.text[y][x] = chr;
	Ok(())
    }

    pub fn to_1bpp(&self) -> [u8; SCREEN_BYTES] {
	let mut frame: [[u8; SCREEN_X_BYTES]; SCREEN_Y] = [[0; SCREEN_X_BYTES]; SCREEN_Y];

	let mut bit_index = 0;
	// rows are 1 per byte but columns are 8 per byte
	let mut pixel_row = 0;
	let mut pixel_byte_col = 0;
	
	for y in 0..TEXT_ROWS {
	    pixel_byte_col = 0;
	    bit_index = 0;
	    for x in 0..TEXT_COLS {
		let letter = font::ETERM_FONT.get(&self.text[y][x]).unwrap();

		for char_col in 0..5 {
		    for char_row in 0..7 {
			frame[pixel_row + char_row][pixel_byte_col] <<= 1;
			if letter[char_row] & (1 << (4-char_col)) == 0 {
			    // zero
			    frame[pixel_row + char_row][pixel_byte_col] |= 0;
			} else {
			    frame[pixel_row + char_row][pixel_byte_col] |= 1;
			}
		    }
		    bit_index += 1;
		    if bit_index == 8 {
			// move to next byte
			pixel_byte_col += 1;
			bit_index = 0;
		    }
		}

		// iterate over pixel in each row of the letter
		//println!("{:?}", font::ETERM_FONT.get(&self.text[y][x]));
	    }
	    pixel_row += 7;
	}

	let mut frame_linear: [u8; SCREEN_BYTES] = [0; SCREEN_BYTES];
	let mut i = 0;
	for row in frame {
	    for b in row {
		if b != 0 {
		    println!("{:?}", b);
		}
		frame_linear[i] = b;
		i += 1;
	    }
	}
	/*
	for row in frame {
	    for col in row {
		print!("{:08b}", col);
	    }
	    println!();
    }*/
		
	frame_linear
	
    }
}

