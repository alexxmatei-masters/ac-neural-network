use std::vec;

use rand::Rng;

fn main() {
    const N: u8 = 0; // intrari
    const M: u8 = 0; // unitati ascunse
    const P: u8 = 0; // iesiri
    const ALPHA: u8 = 3; // rata de invatare
    const INPUT: u8 = 0; // vectorul de intrare

    // vectorul cu valorile stratului ascuns inainte de activare
    const NETH: u8 = 0;
    // matricea ponderilor dintre stratul ascuns si intrare
    let mut whin: Vec<Vec<f32>>;
    let mut bhin: Vec<f32>; // vectorul bias aferent stratului ascuns
    const HIDD: u8 = 0; // vectorul cu valorile stratului ascuns dupa activare
    const NETO: u8 = 0; // vectorul cu valorile de iesire inainte de activare

    // matricea ponderilor dintre stratul de iesire si ascuns
    const WOHI: u8 = 0;
    let mut bohi: Vec<f32>; // vectorul bias aferent stratului de iesire
    const OUT: u8 = 0; // vectorul cu valorile de iesire dupa activare
    const DELTAOUT: u8 = 0; // vectorul de erori aferent stratului de iesire
    const DELTAIN: u8 = 0; // vectorul de erori aferent stratului ascuns

    // vectorul cu iesirile corecte (folosit in etapa de invatare)
    const TARGET: u8 = 0;

    fn generare_ponderi(bhin: &mut Vec<f32>, whin: &mut Vec<Vec<f32>>, bohi: &mut Vec<f32>) {
        const WI: f32 = 4.0 / N as f32;
        const HWI: f32 = 2.0 / N as f32;

        let mut rng = rand::thread_rng();

        for i in 1..M {
            bhin[i as usize] =
                (rng.gen_range(0.0..10000.0) % (WI * 100.0 as f32).floor() as f32) / 100.0 - HWI;
            for j in 1..N {
                whin[i as usize][j as usize] =
                    (rng.gen_range(0.0..10000.0) % (WI * 100.0 as f32).floor() as f32) / 100.0
                        - HWI;
            }
            for i in 1..P {
                bohi[i as usize] =
                    (rng.gen_range(0.0..10000.0) % (WI * 100.0 as f32).floor() as f32) / 100.0
                        - HWI;
                // TODO
            }
        }
    }
}
