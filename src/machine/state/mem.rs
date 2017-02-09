use machine::state::types::*;

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct ByteAt(pub u64);

#[derive(Copy, Clone)]
pub struct WydeAt(pub u64);

#[derive(Copy, Clone)]
pub struct TetraAt(pub u64);

#[derive(Copy, Clone)]
pub struct OctaAt(pub u64);

pub struct Memory {
    buf: Vec<Octa>,
}

impl Memory {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn with_capacity(capacity: u64) -> Self {
        // Compute the number of octas needed
        let len = {
            let mut len = (capacity / 8) as usize;
            if capacity % 8 > 0 {
                len += 1;
            }
            len
        };

        // Create an initialized vector that represents the memory
        let mut buf = Vec::with_capacity(len);
        for _ in 0..len {
            buf.push(0u64.into());
        }

        // Build and return the memory
        Memory {
            buf: buf,
        }
    }
}

impl Index<ByteAt> for Memory {
    type Output = Byte;
    fn index(&self, _: ByteAt) -> &Self::Output {
        unimplemented!();
    }
}

impl IndexMut<ByteAt> for Memory {
    fn index_mut(&mut self, _: ByteAt) -> &mut Self::Output {
        unimplemented!();
    }
}

impl Index<WydeAt> for Memory {
    type Output = Wyde;
    fn index(&self, _: WydeAt) -> &Self::Output {
        unimplemented!();
    }
}

impl IndexMut<WydeAt> for Memory {
    fn index_mut(&mut self, _: WydeAt) -> &mut Self::Output {
        unimplemented!();
    }
}

impl Index<TetraAt> for Memory {
    type Output = Tetra;
    fn index(&self, idx: TetraAt) -> &Self::Output {
        // Find the octa that holds the tetra
        let octa: *const Octa = self.index(OctaAt(idx.0));

        // Calculate the tetra's position within that octa
        let mut pos = (idx.0 % 8 / 4) as isize;
        if cfg!(target_endian = "little") {
            pos = (2 - 1) - pos;
        }

        // Calculate a pointer to the tetra
        let mut tetra = octa as *const Tetra;
        tetra = unsafe {
            tetra.offset(pos)
        };

        // Return the pointer as reference
        unsafe {
            tetra.as_ref()
        }.unwrap()
    }
}

impl IndexMut<TetraAt> for Memory {
    fn index_mut(&mut self, idx: TetraAt) -> &mut Self::Output {
        // Find the octa that holds the tetra
        let octa: *mut Octa = self.index_mut(OctaAt(idx.0));

        // Calculate the tetra's position within that octa
        let mut pos = (idx.0 % 8 / 4) as isize;
        if cfg!(target_endian = "little") {
            pos = (2 - 1) - pos;
        }

        // Calculate a pointer to the tetra
        let mut tetra = octa as *mut Tetra;
        tetra = unsafe {
            tetra.offset(pos)
        };

        // Return the pointer as reference
        unsafe {
            tetra.as_mut()
        }.unwrap()
    }
}

impl Index<OctaAt> for Memory {
    type Output = Octa;
    fn index(&self, idx: OctaAt) -> &Self::Output {
        let pos = (idx.0 / 8) as usize;
        self.buf.index(pos)
    }
}

impl IndexMut<OctaAt> for Memory {
    fn index_mut(&mut self, idx: OctaAt) -> &mut Self::Output {
        let pos = (idx.0 / 8) as usize;
        self.buf.index_mut(pos)
    }
}
