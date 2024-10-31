use std::slice::Iter;

const NUM_MODULES: usize = 8;
const NUM_SECTORS: usize = 16;
const NUM_WORDS: usize = 256;
const NUM_SYLLABLES: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Word {
    value: u16,
}

impl Word {
    pub fn new() -> Self {
        Self { value: 0xFFFF }
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn read(bytes_iter: &mut Iter<'_, u8>) -> Result<Self, ()> {
        let value = read_u16(bytes_iter)?;

        Ok(Self { value })
    }
}

#[derive(Debug, Clone)]
pub struct Syllable {
    words: Vec<Word>,
}

impl Syllable {
    pub fn new() -> Self {
        Self {
            words: (0..NUM_WORDS).into_iter().map(|_| Word::new()).collect(),
        }
    }

    pub fn words(&mut self) -> &mut Vec<Word> {
        &mut self.words
    }

    pub fn read(bytes_iter: &mut Iter<'_, u8>) -> Result<Self, ()> {
        let mut words = Vec::with_capacity(NUM_WORDS);

        for _ in 0..NUM_WORDS {
            words.push(Word::read(bytes_iter)?);
        }

        Ok(Self { words })
    }
}

#[derive(Debug, Clone)]
pub struct Sector {
    syllables: Vec<Syllable>,
}

impl Sector {
    pub fn new() -> Self {
        Self {
            syllables: (0..NUM_SYLLABLES)
                .into_iter()
                .map(|_| Syllable::new())
                .collect(),
        }
    }

    pub fn syllables(&mut self) -> &mut Vec<Syllable> {
        &mut self.syllables
    }

    pub fn read(bytes_iter: &mut Iter<'_, u8>) -> Result<Self, ()> {
        let mut syllables = Vec::with_capacity(NUM_SYLLABLES);

        for _ in 0..NUM_SYLLABLES {
            syllables.push(Syllable::read(bytes_iter)?);
        }

        Ok(Self { syllables })
    }
}

#[derive(Debug, Clone)]
pub struct Module {
    sectors: Vec<Sector>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            sectors: (0..NUM_SECTORS)
                .into_iter()
                .map(|_| Sector::new())
                .collect(),
        }
    }

    pub fn sectors(&mut self) -> &mut Vec<Sector> {
        &mut self.sectors
    }

    pub fn read(bytes_iter: &mut Iter<'_, u8>) -> Result<Self, ()> {
        let mut sectors = Vec::with_capacity(NUM_SECTORS);

        for _ in 0..NUM_SECTORS {
            sectors.push(Sector::read(bytes_iter)?);
        }

        Ok(Self { sectors })
    }
}

#[derive(Debug, Clone)]
pub struct MemoryImage {
    modules: Vec<Module>,
    hop_register: u32,
    accumulator: u32,
    pq_register: u32,
}

impl MemoryImage {
    pub fn new() -> Self {
        Self {
            modules: (0..NUM_MODULES)
                .into_iter()
                .map(|_| Module::new())
                .collect(),
            hop_register: 0,
            accumulator: 0,
            pq_register: 0,
        }
    }

    pub fn modules(&mut self) -> &mut Vec<Module> {
        &mut self.modules
    }

    pub fn read(bytes: &[u8]) -> Result<Self, ()> {
        let mut modules = Vec::with_capacity(NUM_MODULES);

        let mut bytes_iter = bytes.iter();

        for _ in 0..NUM_MODULES {
            modules.push(Module::read(&mut bytes_iter)?);
        }

        let hop_register = read_u32(&mut bytes_iter)?;
        let accumulator = read_u32(&mut bytes_iter)?;
        let pq_register = read_u32(&mut bytes_iter)?;

        Ok(Self {
            modules,
            hop_register,
            accumulator,
            pq_register,
        })
    }
}

fn read_u16(bytes_iter: &mut Iter<'_, u8>) -> Result<u16, ()> {
    let left = *bytes_iter.next().ok_or(())?;
    let right = *bytes_iter.next().ok_or(())?;

    Ok(((left as u16) << 8) | (right as u16))
}

fn read_u32(bytes_iter: &mut Iter<'_, u8>) -> Result<u32, ()> {
    let left = read_u16(bytes_iter)?;
    let right = read_u16(bytes_iter)?;

    Ok(((left as u32) << 16) | (right as u32))
}
