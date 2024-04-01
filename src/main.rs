use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub fn main() {
    // Generate all combinations of non repeating number from length 4 to 9.
    let mut counter: i32 = 0;
    let mut hasher = Sha1::new();
    'external: for x in 1234..987654321 {
        let str_version = x.to_string();
        if str_version.matches("0").count() > 0 {
            continue;
        }
        for y in 1..10 {
            if str_version.matches(y.to_string().as_str()).count() > 1 {
                continue 'external;
            }
        }
        let mapped: Vec<u8> = str_version.chars().map(|c| c.to_digit(10).unwrap() as u8 - 1).collect();
        hasher.input(mapped.as_slice());
        let hex = hasher.result_str();
        if hex == "342cd322b93f9cdd87de7465e4084e8f5763cb13" {
            println!("FOUND {}", str_version);
            break 'external;
        }
        hasher.reset();
        counter += 1;
        if counter % 10000 == 0 {
            println!("{} done - {}", counter, str_version);
        }
    }
    println!("{}", counter);
}
