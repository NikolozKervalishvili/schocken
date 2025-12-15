use rand::Rng;

#[derive(Default)]
pub struct Dice(u8);

impl Dice {
    fn roll(&mut self) {
        let mut rng = rand::rng();
        self.0 = rng.random_range(1..=6);
    }
}
