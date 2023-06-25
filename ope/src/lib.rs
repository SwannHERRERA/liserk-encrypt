pub mod hgd;
pub mod simplified_version;
pub mod stat;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use stat::sample_hgd;
    use stat::ValueRange;

    #[test]
    fn it_works() {
        let in_range = ValueRange::new(0.0, 127.0);
        let out_range = ValueRange::new(0.0, 127.0);
        let nsample = 64.0;

        let mut rng = rand::thread_rng();
        let mut seed_coins = [0u8; 32];
        for coin in seed_coins.iter_mut() {
            *coin = rng.gen_range(0..=1);
        }

        let result_1 = sample_hgd(&in_range, &out_range, &nsample, &seed_coins);
        let nsample = 24.0;
        let result_2 = sample_hgd(&in_range, &out_range, &nsample, &seed_coins);

        // Afficher le résultat
        println!("Résultat de sample_hgd: {}, {}", result_1, result_2);
        println!("1 > 2 sample_hgd: {}", result_1 > result_2);
    }
}
