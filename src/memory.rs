#[derive(PartialEq)]
pub enum MemoryTrait {
    Readble,
    Writable,
}

pub struct Memory {
    pub memory_modules: Vec<MemoryModule>,
}

pub struct MemoryModule {
    name: String,
    data: Vec<u8>,
    size: u16,
    start_location: u16,
    traits: Vec<MemoryTrait>,
}

impl MemoryModule {
    fn default_eeprom(program: Vec<u8>) -> Self {
        let data = program.clone();
        Self {
            name: "EEPROM".to_string(),
            data,
            size: 0x7fff,
            start_location: 0x8000,
            traits: vec![MemoryTrait::Readble],
        }
    }
    fn default_ram() -> Self {
        Self {
            name: "RAM".to_string(),
            data: Vec::new(),
            size: 0xff,
            start_location: 0x100,
            traits: vec![MemoryTrait::Readble, MemoryTrait::Writable],
        }
    }
    fn default_2port() -> Self {
        Self {
            name: "2Port Chip".to_string(),
            data: Vec::new(),
            size: 0x2,
            start_location:0x6000, //i'm a loyal boy
            traits: vec![MemoryTrait:: Readble, MemoryTrait:: Writable],
        }
    }

    fn new(
        name: String,
        data: Vec<u8>,
        size: u16,
        start_location: u16,
        traits: Vec<MemoryTrait>,
    ) -> Self {
        Self {
            name,
            data,
            size,
            start_location,
            traits,
        }
    }
}

impl Memory {
    pub fn default_init(program: Vec<u8>) -> Self {
        Self {
            memory_modules: vec![
                MemoryModule::default_eeprom(program),
                MemoryModule::default_ram(),
                MemoryModule::default_2port(),
            ],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        //can happen from any address but will return garbage if not read in the designated areas.
        for module in &self.memory_modules {
            if address >= module.start_location && address <= (module.start_location + module.size)
            {
                //start + size is the last location.
                if module.traits.contains(&MemoryTrait::Readble) {
                    //if try to search an uninitialized value: 0 will be returned.
                    return *module
                        .data
                        .get((address - module.start_location) as usize)
                        .unwrap_or(&0);
                } else {
                    //if the address that is searched for is not a readble module exit.
                    panic!("read does not exist");
                }
            }
        }
        0
    }
    pub fn write(&mut self, address: u16, data: u8) {
        for module in &mut self.memory_modules {
            if address >= module.start_location && address <= (module.start_location + module.size)
            {
                if address == 0x6002 {
                    println!("{}", data);
                }
                //start + size is the last location.
                if module.traits.contains(&MemoryTrait::Writable) {
                    let address_module = (address - module.start_location) as usize;
                    // println!("{} {}", module.data.len(), address_module);
                    //if the address that is written to is not yet intialized all the addresses before it need to be intialized.
                    if address_module >= module.data.len() {
                        for _ in (module.data.len())..address_module {
                            module.data.push(0);
                        }
                        module.data.push(data);
                        return;
                    } else {
                        module.data[address_module] = data;
                        return;
                    }
                } else {
                    //if the address that is searched for is not a writable module exit.
                    panic!("write address does not exist");
                }
            }
        }
        panic!(
            "attempted to write to an address that isn't writable: {}",
            address
        );
    }
}
