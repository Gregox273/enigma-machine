use std::convert::TryFrom;

// Generic struct to describe wiring that translates one symbol to another
pub struct Wiring {
    map: Vec<u8>,
}

impl Wiring {
    // Constructor
    pub fn new(map: Vec<u8>) -> Wiring {
        // Check input
        assert!(map.len() <= (std::u8::MAX as usize) + 1,
            "Error: Wiring map too large to indexed by type \'u8\'");

        for (_i, value) in map.iter().enumerate() {
            assert!((*value as usize) < map.len(),
                "out-of-bounds value for wiring connection: \'{}\' of \'{}\'", value, map.len());
        }
        // TODO: check validity of mapping (should be 1 to 1 i.e. no repeats or missing values)?
        Wiring {
            map
        }
    }

    // Translate one symbol to another based on wire mapping
    pub fn translate(&self, input: u8) -> u8 {
        self.map[input as usize]
    }

    // Translate one symbol to another moving in the opposite direction to the wire mapping
    pub fn translate_backwards(&self, input: u8) -> u8 {
        let output_opt: Option<usize> = self.map.iter().position(|&x| x == input);
        let output_usize: usize = match output_opt {
            None =>
                panic!("Rotor missing connection for reverse-direction input \'{}\'", input),
            Some(x) => x,
        };

        let try_cast = u8::try_from(output_usize);
        let output_u8: u8 = match try_cast {
            Ok(x) => x,
            Err(e) => panic!("Integer overflow - wiring map vector contains too many elements to \
            fit inside return type \'u8\': {:?}", e),
        };
        output_u8
    }

    // Return the number of connections in the wiring map
    pub fn len(&self) -> u8 {
        let try_cast = u8::try_from(self.map.len());
        let output_u8: u8 = match try_cast {
            Ok(x) => x,
            Err(e) => panic!("Integer overflow - wiring map vector contains too many elements to \
            be indexed by type \'u8\': {:?}", e),
        };
        output_u8
    }
}
