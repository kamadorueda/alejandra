use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::rngs::OsRng;

pub(crate) fn random_ad() -> &'static str {
    let ads = [include_str!("star.txt"), include_str!("donation.txt")];

    let ads_index = Uniform::from(0..ads.len()).sample(&mut OsRng);

    ads[ads_index]
}
