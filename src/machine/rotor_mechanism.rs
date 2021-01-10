use super::wiring::Wiring;
use super::rotor::Rotor;

pub const MIN_ROTORS: usize = 3;

// Struct describing mechanism to handle rotor movement
pub struct RotorMechanism {
    etw: Rotor,
    rotors: Vec<Rotor>,  // ordered right to left
    reflector: Wiring,
    rotor_positions: u8,
}

impl RotorMechanism {
    // Constructor
    pub fn new(etw: Rotor, rotors: Vec<Rotor>,
        reflector: Wiring, rotor_positions: u8) -> RotorMechanism {
        assert!(rotors.len() >= MIN_ROTORS,
            "Error: mechanism requires at least {} rotors", MIN_ROTORS);

        for rotor in rotors.iter() {
            assert!(rotor.num_positions() == rotor_positions,
            "Error: {} position mechanism specified but {} position rotor was supplied",
            rotor_positions, rotor.num_positions());
        }

        RotorMechanism {
            etw,
            rotors,
            reflector,
            rotor_positions,
        }
    }

    pub fn advance(&mut self) {
        // leftmost rotor
        let number_of_rotors: usize = self.rotors.len();
        if self.rotors[number_of_rotors - 2].is_notch_positioned() {
            self.rotors[number_of_rotors - 1].advance();
        }

        // middle rotor(s)
        for i in (1..self.rotors.len()-1).rev() {
            let rotor: &mut Rotor = &mut self.rotors[i];
            let rotor_notch_positioned: bool = rotor.is_notch_positioned();

            let rotor_to_right: &mut Rotor = &mut self.rotors[i-1];
            let rotor_to_right_notch_positioned: bool = rotor_to_right.is_notch_positioned();


            if rotor_notch_positioned || rotor_to_right_notch_positioned {
                self.rotors[i].advance();
            }
        }

        // rightmost rotor always advances
        self.rotors[0].advance();
    }

    // Translate input character (coming from stecker) to lamp character (going back to stecker)
    pub fn translate(&self, input: u8) -> u8 {
        assert!(input < self.rotor_positions, "Invalid input character: \'{}\'", input);

        // Enter via etw
        // Contact position is relative to fixed reference ('A' input on ETW)
        let mut contact_position: u8 = self.etw.translate_r_to_l(input);

        //Pass through rotors right to left
        for rotor in self.rotors.iter() {
            // Transform to rotor reference frame
            contact_position =
                (contact_position + rotor.get_rotor_position()) % self.rotor_positions;
            // Pass current through rotor wiring
            contact_position = rotor.translate_r_to_l(contact_position);
            // Transform back to fixed (ETW) reference frame
            // (unnecessary operations, could be optimised...)
            contact_position =
                contact_position + self.rotor_positions - rotor.get_rotor_position();
            contact_position = contact_position % self.rotor_positions;
        }

        // reflector
        contact_position = self.reflector.translate(contact_position);

        // Pass through rotors left to right
        for rotor in self.rotors.iter().rev() {
            // Transform to rotor reference frame
            contact_position =
                (contact_position + rotor.get_rotor_position()) % self.rotor_positions;
            // Pass current through rotor wiring
            contact_position = rotor.translate_l_to_r(contact_position);
            // Transform back to fixed (ETW) reference frame
            // (unnecessary operations, could be optimised...)
            contact_position =
                contact_position + self.rotor_positions - rotor.get_rotor_position();
            contact_position = contact_position % self.rotor_positions;
        }

        // Exit via etw
        self.etw.translate_l_to_r(contact_position)
    }
}
