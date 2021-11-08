# hibp_password_analizer_rust
Analyze the password dump by frequency

# Steps

1. Download the password leak list (order by prevalence) [here](https://downloads.pwnedpasswords.com/passwords/pwned-passwords-sha1-ordered-by-count-v7.7z)
2. Unzip using 7z
3. clone this repository
4. `cargo build --release`
5. run the binary (`./target/release/hibp_password_analizer_rust`) with the unzipped txt file as the argument.
6. You will get a csv file called output.csv
7. Values are `pwnedCount, numberOfPasswords` (ie. `1, 25` means "25 passwords were pwned 1 time")

# Terminal

```bash
$ git clone https://github.com/junderw/hibp_password_analizer_rust.git
$ cd hibp_password_analizer_rust
$ cargo build --release
$ wget https://downloads.pwnedpasswords.com/passwords/pwned-passwords-sha1-ordered-by-count-v7.7z
$ 7z x pwned-passwords-sha1-ordered-by-count-v7.7z
$ ./target/release/hibp_password_analizer_rust pwned-passwords-sha1-ordered-by-count-v7.txt
```