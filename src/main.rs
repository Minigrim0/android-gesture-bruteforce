use crypto::digest::Digest;
use crypto::sha1::Sha1;

pub fn main() {
    // Generate all combinations of non repeating number from length 4 to 9.
    let mut counter = 0;
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
        if hasher.result_str() == "2c3422d33fb9dd9cde87657408e48f4e635713cb" {
            println!("FOUND {}", str_version);
            break 'external;
        }
        hasher.reset();

        if counter % 10000 == 0 {
            println!("{} done - {}", counter, str_version);
        }
        counter += 1;
    }
    println!("{} done", counter);
}
