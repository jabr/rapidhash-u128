# rapidhash-u128

Bare-bones Rust implementation of rapidhash for u128 values, based on original [rapidhash implementation in C](https://github.com/Nicoshev/rapidhash).

## Usage

Library:
```
use rapidhash_u128::hash;
hash(0xff) // => 5200326291411116507
```

Binary:
```
$ rapidhash-u128 ff
RapidHash(255) = 5200326291411116507
```

## Related

https://github.com/hoxxep/rapidhash - A full-featured Rust implementation of Rapidhash

## License

This project is licensed under the terms of the [MIT license](LICENSE.txt).
