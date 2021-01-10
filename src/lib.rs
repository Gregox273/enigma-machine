mod machine;

pub use machine::wiring::Wiring;
pub use machine::rotor::Rotor;
use machine::rotor_mechanism::RotorMechanism;

pub struct Enigma {
    rotor_mechanism: RotorMechanism,
    stecker: Wiring,
    alphabet_size: u8,
}

impl Enigma {
    // Constructor
    pub fn new(
        rotors: Vec<Rotor>,
        etw_wiring: Wiring,
        reflector_wiring: Wiring,
        stecker_wiring: Wiring,
        ) -> Enigma {

        // Check input sizes
        let num_wiring_connections: u8 = etw_wiring.len();
        assert!(num_wiring_connections == reflector_wiring.len() &&
            num_wiring_connections == stecker_wiring.len(),
            "Supplied wiring maps must contain the same number of connections");

        for rotor in rotors.iter() {
            assert!(rotor.num_positions() == num_wiring_connections,
            "Supplied rotor contains {} connections, but supplied wiring maps contain {} connections",
            rotor.num_positions(), num_wiring_connections);
        }

        let etw: Rotor = Rotor::new(0, Vec::new(), 0, etw_wiring);

        let rotor_mechanism: RotorMechanism =
            RotorMechanism::new(etw, rotors, reflector_wiring, num_wiring_connections);

        Enigma {
            rotor_mechanism,
            stecker: stecker_wiring,
            alphabet_size: num_wiring_connections
        }
    }

    // Translate a symbol: enter keypress, return lamp value
    pub fn translate(&mut self, input: u8) -> u8 {
        assert!(input < self.alphabet_size, "Invalid input character: \'{}\'", input);

        // Advance rotors
        self.rotor_mechanism.advance();

        // First pass through stecker
        let steckered_keypress: u8 = self.stecker.translate(input);

        // Pass through rotor mechanism
        let rotor_mech_output: u8 = self.rotor_mechanism.translate(steckered_keypress);

        // Pass through stecker
        self.stecker.translate(rotor_mech_output)
    }
}
//
//
//
//
// pub fn enter_text() {}
//
// pub fn set_rotors() {}
//
// pub fn get_rotors() {}
//
// pub fn set_plugboard() {}
//
// pub fn get_plugboard() {}
