mod machine;

use machine::wiring::Wiring;
use machine::rotor::Rotor;
use machine::rotor_mechanism::RotorMechanism;

pub struct Enigma {
    rotor_mechanism: RotorMechanism,
    stecker: Wiring,
}

// impl Enigma {
//     // Constructor
//     pub fn new() -> Enigma {
//         Enigma {
//         }
//     }
// }
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
