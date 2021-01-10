use super::wiring::Wiring;

// Struct describing rotor
pub struct Rotor {
    position: u8,

    // Constant properties
    turnover_positions: Vec<u8>,
    ring_setting: u8,
    wiring: Wiring,  // right-to-left direction
}

impl Rotor {
    // Constructor
    pub fn new(position: u8, turnover_positions: Vec<u8>, ring_setting: u8,
        wiring: Wiring) -> Rotor {
        // Check arguments
        assert!(position < wiring.len(),
            "Rotor position setting \'{}\' of \'{}\' is out-of-bounds", position, wiring.len());
        assert!(ring_setting < wiring.len(),
            "Rotor ring setting \'{}\' of \'{}\' is out-of-bounds", ring_setting, wiring.len());
        for turnover_position in turnover_positions.iter() {
            assert!(*turnover_position < wiring.len(),
                "Rotor turnover position \'{}\' of \'{}\' is out-of-bounds",
                turnover_position, wiring.len());
        }

        Rotor {
            position,
            turnover_positions,
            ring_setting,
            wiring,
        }
    }

    // Advance rotor position
    pub fn advance(&mut self) {
        self.position = (self.position + 1) % self.num_positions();
    }

    // Manually set rotor position
    pub fn set_rotor_position(&mut self, position: u8) {
        assert!(position < self.num_positions(),
            "Cannot set rotor to position {} (out of bounds)", position);
        self.position = position;
    }

    // Get rotor position
    pub fn get_rotor_position(&self) -> u8 {
        self.position
    }

    // Return true if notch is positioned to allow turnover
    pub fn is_notch_positioned(&self) -> bool {
        for pos in self.turnover_positions.iter() {
            if self.position == *pos {
                return true
            }
        }

        return false
    }

    // Translate one symbol to another using internal wiring (right-to-left direction)
    pub fn translate_r_to_l(&self, input: u8) -> u8 {
        assert!(input < self.num_positions(),
            "Error: input \'{}\' is out of bounds (rotor has {} positions)",
            input, self.num_positions());

        let input_contact: u8 = (input + self.num_positions() - self.ring_setting) % self.num_positions();
        let output_contact: u8 = self.wiring.translate(input_contact);
        (output_contact + self.ring_setting) % self.num_positions()
    }

    // Translate one symbol to another using internal wiring (left-to-right direction)
    pub fn translate_l_to_r(&self, input: u8) -> u8 {
        assert!(input < self.num_positions(),
            "Error: input \'{}\' is out of bounds (rotor has {} positions)",
            input, self.num_positions());

        let input_contact: u8 = (input + self.num_positions() - self.ring_setting) % self.num_positions();
        let output_contact: u8 = self.wiring.translate_backwards(input_contact);
        (output_contact + self.ring_setting) % self.num_positions()
    }

    // Return number of rotor positions (number of symbols in rotor alphabet)
    pub fn num_positions(&self) -> u8 {
        self.wiring.len()
    }
}
