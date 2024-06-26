use getrandom::Error;

fn get_random(dest: &mut [u8]) -> Result<(), Error> {
    let mut rand = oorandom::Rand64::new(ic_cdk::api::time() as u128);
    for i in 1..=dest.len() {
        dest[i - 1] = rand.rand_u64() as u8
    }
    Ok(())
}

getrandom::register_custom_getrandom!(get_random);
