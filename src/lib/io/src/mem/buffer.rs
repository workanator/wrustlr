use std::mem;
use std::io;
use std::slice;
use std::ptr;
use libc;
use wrust_types::{Result, Error};


/// Buffer usage mode
#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
	/// Data is pushing into the buffer.
	Push,
	/// Buffer is used for data consuming.
	Consume,
	/// Data is pulling from the buffer.
	Pull,
}


/// `Buffer` is intended to use in all I/O operations of processing modules.
/// The buffer is designed to work in consequent Push-Consume-Pull operations.
///
/// That is how Wrustlr works with client connections:
///
///* `Push` data into the `Buffer`, for example read from a socket and write into the buffer;
///* `Consume` data in the `Buffer` (if required), for example the stream processing modules
/// consumes read data;
///* `Pull` data from the `Buffer`, for example take data from the buffer and write to the socket.
pub struct Buffer {
	ptr: *mut u8,
	capacity: usize,
	position: usize,
	length: usize,
	mode: Mode,
}


impl Buffer {
	/// Create the new instance of `Buffer` of the given `capacity`. Memory preallocated
	/// is zeroed.
	pub fn new(capacity: usize) -> Buffer {
		let buf = unsafe { Buffer {
			ptr: libc::calloc(capacity, mem::size_of::<u8>()) as *mut u8,
			capacity: capacity,
			position: 0,
			length: 0,
			mode: Mode::Consume,
		} };

		buf
	}

	/// Get the mode the buffer is in
	pub fn mode(&self) -> Mode {
		self.mode.clone()
	}

	/// Get the capacity of the buffer.
	pub fn capacity(&self) -> usize {
		self.capacity
	}

	/// Get the length of contained data.
	pub fn len(&self) -> usize {
		self.length
	}

	/// Test if the buffer is empty.
	pub fn is_empty(&self) -> bool {
		match self.mode {
			Mode::Push => self.position == 0,
			_ => self.length == 0
		}
	}

	/// Test if the buffer is filled.
	pub fn is_filled(&self) -> bool {
		self.position == self.capacity
	}

	/// Calculate the remaining amount of bytes. The result depends on the mode the buffer in.
	///
	/// * In `Push` mode returns the available amount of bytes could be pushed into the buffer.
	/// * In `Consume` mode returns the amount of bytes in the buffer (equivalent to `len()` method).
	/// * In `Pull` mode returns the amount of bytes left in the buffer.
	pub fn remaining(&self) -> usize {
		match self.mode {
			Mode::Push => self.capacity - self.position,
			Mode::Consume => self.length,
			Mode::Pull => self.length - self.position
		}
	}

	/// Switch buffer into the `Push` mode and reset the state.
	pub fn begin_push(&mut self) {
		if self.mode != Mode::Push {
			self.mode = Mode::Push;

			// Copy the remaining data to the beiginning of the buffer
			if self.position > 0 && self.position < self.length {
				self.length = self.length - self.position;
				unsafe {
					ptr::copy(self.ptr.offset(self.position as isize), self.ptr, self.length);
				};
			}
			else {
				self.length = 0;
			}

			self.position = 0;
		}
	}

	/// Switch buffer into the `Consume` mode and reset the state.
	pub fn begin_consume(&mut self) {
		if self.mode != Mode::Consume {
			self.mode = Mode::Consume;
			self.position = 0;
		}
	}

	/// Switch buffer into the `Pull` mode and reset the state.
	pub fn begin_pull(&mut self) {
		if self.mode != Mode::Pull {
			self.mode = Mode::Pull;
			self.position = 0;
		}
	}

	/// Fill the buffer memory with zero.
	///
	/// ## Panics
	///
	/// Panics if the buffer is not in `Push` mode.
	pub fn zero(&mut self) {
		if self.mode != Mode::Push {
			panic!(format!("Cannot zero buffer in mode {:?}", self.mode));
		}

		unsafe {
			ptr::write_bytes(self.ptr, 0, self.capacity);
		};
	}

	/// Clear the buffer. The implementation just resets the inner r/w position and
	/// the length of contained data.
	pub fn clear(&mut self) {
		self.position = 0;
		self.length = 0;
	}

	/// Push as much bytes as possible from `src` to the buffer.
	pub fn push(&mut self, src: &[u8]) -> Result<usize> {
		match self.mode {
			Mode::Push => {
				// Calculate the amount of bytes we can copy from src.
				let mut count = self.remaining();
				if count > src.len() {
					count = src.len();
				}

				// Copy non-zero block of bytes and update pointers.
				if count > 0 {
					unsafe {
						ptr::copy(&src[0], self.ptr.offset(self.position as isize), count);
						self.position = self.position + count;
						self.length = self.length + count;
					}
				}

				Ok(count)
			},
			_ => Error::general("Cannot push data into buffer").because(format!("Mode is {:?}", self.mode)).result()
		}
	}

	/// Push as much bytes as possible from the range in `src` to the buffer.
	///
	/// ## Panics
	///
	/// Panics if the range is out of bounds of `src`.
	pub fn push_range(&mut self, src: &[u8], start: usize, length: usize) -> Result<usize> {
		match self.mode {
			Mode::Push => {
				// Check bounds
				if start > src.len() {
					panic!(format!("Start range index is out of bounds ({}:{})", start, src.len()));
				}

				if start + length - 1 > src.len() {
					panic!(format!("End range index is out of bounds ({}:{})", start + length - 1, src.len()));
				}

				// Calculate the amount of bytes we can copy from src.
				let mut count = self.remaining();
				if count > length {
					count = length;
				}

				// Copy non-zero block of bytes and update pointers.
				if count > 0 {
					unsafe {
						ptr::copy(&src[start], self.ptr, count);
						self.position = self.position + count;
						self.length = self.length + count;
					}
				}

				Ok(count)
			},
			_ => Error::general("Cannot push data into buffer").because(format!("Mode is {:?}", self.mode)).result()
		}
	}

	/// Read the required amount of bytes from the `reader` and push them into the buffer.
	pub fn read_and_push_from<R>(&mut self, reader: &mut R) -> Result<usize>
		where R: io::Read {
		match self.mode {
			Mode::Push => {
				match unsafe { reader.read(slice::from_raw_parts_mut(self.ptr.offset(self.position as isize), self.remaining())) } {
					Ok(count) => {
						self.position = self.position + count;
						self.length = self.length + count;
						Ok(count)
					},
					Err(msg) => make_err!(msg),
				}
			},
			_ => Error::general("Cannot read and push data into buffer").because(format!("Mode is {:?}", self.mode)).result()
		}
	}

	/// Get the contained data as byte slice.
	///
	/// ## Panics
	///
	/// Panics if the buffer is not in `Consume` mode.
	pub fn consume<'a>(&'a self) -> &'a [u8] {
		unsafe {
			match self.mode {
				Mode::Consume => {
					// Create and return the slice
					slice::from_raw_parts(self.ptr, self.length)
				},
				_ => panic!(format!("Cannot consume buffer in mode {:?}", self.mode))
			}
		}
	}

	/// Get the range  of the contained data as byte slice.
	///
	/// ## Panics
	///
	/// Panics if the range is out of buffer bounds or if the buffer is not in `Consume` mode.
	pub fn consume_range<'a>(&'a self, start: usize, length: usize) -> &'a [u8] {
		unsafe {
			match self.mode {
				Mode::Consume => {
					// Check bounds
					if start > self.length {
						panic!(format!("Start range index is out of bounds ({}:{})", start, self.length));
					}

					if start + length - 1 > self.length {
						panic!(format!("End range index is out of bounds ({}:{})", start + length - 1, self.length));
					}

					// Create and return the slice
					slice::from_raw_parts(self.ptr.offset(start as isize), length)
				},
				_ => panic!(format!("Cannot consume buffer range in mode {:?}", self.mode))
			}
		}
	}

	/// Pull data from the buffer to `dst`.
	pub fn pull(&mut self, dst: &mut [u8]) -> Result<usize> {
		match self.mode {
			Mode::Pull => {
				// Calculate the amount of bytes we can copy from the buffer.
				let mut count = self.remaining();
				if count > dst.len() {
					count = dst.len();
				}

				// Copy non-zero block of bytes into dst.
				if count > 0 {
					unsafe {
						ptr::copy(self.ptr.offset(self.position as isize), &mut dst[0], count);
						self.position = self.position + count;
					}
				}

				Ok(count)
			},
			_ => Error::general("Cannot pull data from buffer").because(format!("Mode is {:?}", self.mode)).result()
		}
	}

	/// Pull data from the buffer to the `dst`'s range.
	///
	/// ## Panics
	///
	/// Panics if the range is out of `dst`'s bounds.
	pub fn pull_range(&mut self, dst: &mut [u8], start: usize, length: usize) -> Result<usize> {
		match self.mode {
			Mode::Pull => {
				// Check bounds
				if start > dst.len() {
					panic!(format!("Start range index is out of bounds ({}:{})", start, dst.len()));
				}

				if start + length - 1 > dst.len() {
					panic!(format!("End range index is out of bounds ({}:{})", start + length - 1, dst.len()));
				}

				// Calculate the amount of bytes we can copy from the buffer.
				let mut count = self.remaining();
				if count > length {
					count = length;
				}

				// Copy non-zero block of bytes into dst.
				if count > 0 {
					unsafe {
						ptr::copy(self.ptr.offset(self.position as isize), &mut dst[start], count);
						self.position = self.position + count;
					}
				}

				Ok(count)
			},
			_ => Error::general("Cannot pull data range from buffer").because(format!("Mode is {:?}", self.mode)).result()
		}
	}

	/// Pull data from the buffer and write to the `reader`.
	pub fn pull_and_write_to<W>(&mut self, writer: &mut W) -> Result<usize>
		where W: io::Write {
		match self.mode {
			Mode::Pull => {
				let remain = self.remaining();
				if remain > 0 {
					match unsafe { writer.write(slice::from_raw_parts(self.ptr.offset(self.position as isize), remain)) } {
						Ok(count) => {
							self.position = self.position + count;
							Ok(count)
						},
						Err(msg) => make_err!(msg),
					}
				}
				else {
					Ok(0)
				}
			},
			_ => Error::general("Cannot pull and write data from buffer").because(format!("Mode is {:?}", self.mode)).result()
		}
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		unsafe {
			libc::free(self.ptr as *mut libc::c_void);
		};
	}
}


#[cfg(test)]
mod tests {
	use super::{Buffer, Mode};

	#[test]
	fn buffer_create() {
		let size = 1024;
		let buf = Buffer::new(size);
		assert_eq!(buf.capacity(), size);
		assert_eq!(buf.len(), 0);
		assert_eq!(buf.mode(), Mode::Consume);
		assert_eq!(buf.is_empty(), true);
		assert_eq!(buf.is_filled(), false);
		assert_eq!(buf.remaining(), buf.len());
	}

	#[test]
	fn buffer_remaining() {
		let byte = [1u8];
		let count = 10;
		let size = 1024;
		let mut buf = Buffer::new(size);

		buf.begin_push();
		for _ in 0..count {
			let _ = buf.push(&byte);
		}

		assert_eq!(buf.remaining(), size - count);
		buf.begin_consume();
		assert_eq!(buf.remaining(), count);
		buf.begin_pull();
		assert_eq!(buf.remaining(), count);
	}

	#[test]
	fn buffer_push() {
		let byte = [1u8];
		let size = 1024;
		let mut buf = Buffer::new(size);

		buf.begin_push();
		buf.zero();

		for n in 0..size {
			assert_eq!(buf.remaining(), size - n);
			match buf.push(&byte) {
				Ok(c) if c == 1 => (),
				Ok(n) => panic!(format!("Pushed invalid number of bytes {}", n)),
				Err(msg) => panic!("{}", msg)
			};
		}

		match buf.push(&byte) {
			Ok(n) => {
				if n > 0 {
					panic!(format!("Pushed more ({}:{}) than 0 bytes", n, buf.remaining()));
				}
			},
			Err(msg) => panic!("{}", msg)
		};

		assert_eq!(buf.is_filled(), true);
		assert_eq!(buf.is_empty(), false);
	}

	#[test]
	#[should_panic]
	fn push_range_panic() {
		let pattern = [1u8, 2u8, 3u8, 4u8, 5u8];
		let size = 1024;
		let mut buf = Buffer::new(size);

		buf.begin_push();
		let _ = buf.push_range(&pattern, 1, 10);
	}

	#[test]
	fn buffer_consume() {
		let pattern = [1u8, 2u8, 3u8, 4u8, 5u8];
		let size = 1024;
		let mut buf = Buffer::new(size);

		buf.begin_push();
		let _ = buf.push(&pattern);
		assert_eq!(buf.len(), pattern.len());

		buf.begin_consume();
		assert_eq!(buf.is_empty(), false);
		assert_eq!(buf.consume(), pattern);
	}

	#[test]
	fn buffer_consume_range() {
		let pattern = [1u8, 2u8, 3u8, 4u8, 5u8];
		let pattern_range1 = [1u8, 2u8];
		let pattern_range2 = [2u8, 3u8, 4u8];
		let size = 1024;
		let mut buf = Buffer::new(size);

		buf.begin_push();
		let _ = buf.push(&pattern);
		assert_eq!(buf.len(), pattern.len());

		buf.begin_consume();
		assert_eq!(buf.is_empty(), false);
		assert_eq!(buf.consume_range(0, buf.len()), pattern);
		assert_eq!(buf.consume_range(0, 2), pattern_range1);
		assert_eq!(buf.consume_range(1, 3), pattern_range2);
	}

	#[test]
	#[should_panic]
	fn buffer_consume_panic_mode() {
		let pattern = [1u8, 2u8, 3u8, 4u8, 5u8];
		let size = 1024;
		let mut buf = Buffer::new(size);

		buf.begin_push();
		let _ = buf.push(&pattern);
		assert_eq!(buf.len(), pattern.len());

		assert_eq!(buf.consume(), pattern);
	}

	#[test]
	#[should_panic]
	fn buffer_consume_panic_range() {
		let pattern = [1u8, 2u8, 3u8, 4u8, 5u8];
		let size = 1024;
		let mut buf = Buffer::new(size);

		buf.begin_push();
		let _ = buf.push(&pattern);
		assert_eq!(buf.len(), pattern.len());

		assert_eq!(buf.consume_range(1, 10), pattern);
	}

	#[test]
	#[should_panic]
	fn buffer_zero_panic() {
		let mut buf = Buffer::new(1024);
		buf.zero();
	}

	#[test]
	fn buffer_pull() {
		let pattern = [1u8, 2u8, 3u8, 4u8, 5u8];
		let mut byte = [0u8; 1];
		let size = 1024;
		let mut buf = Buffer::new(size);

		buf.begin_push();
		let _ = buf.push(&pattern);
		assert_eq!(buf.len(), pattern.len());

		buf.begin_pull();
		assert_eq!(buf.is_empty(), false);

		for n in 0..pattern.len() {
			assert_eq!(buf.remaining(), pattern.len() - n);
			match buf.pull(&mut byte) {
				Ok(c) if c == 1 => assert_eq!(pattern[n], byte[0]),
				Ok(n) => panic!(format!("Pulled invalid number of bytes {}", n)),
				Err(msg) => panic!("{}", msg)
			};
		}

		match buf.pull(&mut byte) {
			Ok(n) => {
				if n > 0 {
					panic!(format!("Pulled more ({}:{}) than 0 bytes", n, buf.len()));
				}
			},
			Err(msg) => panic!("{}", msg)
		};

		assert_eq!(buf.is_filled(), false);
		assert_eq!(buf.is_empty(), false);
	}

	#[test]
	fn buffer_push_pull_push() {
		let pattern = [1u8, 2u8, 3u8, 4u8, 5u8];
		let pattern_shifted = [4u8, 5u8, 1u8, 2u8, 3u8];
		let mut byte = [0u8];
		let size = 1024;
		let mut buf = Buffer::new(size);

		buf.begin_push();
		let _ = buf.push(&pattern);

		buf.begin_pull();
		for _ in 0..3 {
			let _ = buf.pull(&mut byte);
		}

		buf.begin_push();
		let _ = buf.push_range(&pattern, 0, 3);

		buf.begin_consume();
		assert_eq!(buf.len(), pattern_shifted.len());
		assert_eq!(buf.consume(), pattern);
	}

	// Test reader and writer
}
