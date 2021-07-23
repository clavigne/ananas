# ananas 

🍍 Put arbitrary bytes in your NaNs! 🍍

As wikipedia explains,

    For example, a bit-wise IEEE 754 single precision (32-bit) NaN would be
	
    `s111 1111 1xxx xxxx xxxx xxxx xxxx xxxx`

	where s is the sign (most often ignored in applications) and the x sequence
	represents a non-zero number (the value zero encodes infinities). The most
	significant bit from x is used to determine the type of NaN: "quiet NaN" or
	"signaling NaN". *The remaining bits encode a payload (most often ignored in
	applications).*

	[NaN, Wikipedia (2021)](https://en.wikipedia.org/wiki/NaN) (Emphasis mine)

The NaN payload is ignored no longer! Using ananas you can imbue entirely new
meaning to your NaNs.

A NaN has an (ignored) sign of 0 or 1, an exponent of all 1 and a significand.
The first bit of the significand determines if the NaN is signalling or quiet.
The remaining 22 bits can be set to anything except 0. We define our special
NaNs like so,

```text
i111 1111 1s1nn 0000 xxxx xxxx xxxx xxxx
```

where `i` is the sign (ignored), `s` is the signaling bit, `n` is a tag for the
number of encoded bytes and `x` is our payload. The middle 0000 are unused.

## Examples

We can put and take out arbitrary bytes into NaNs, encoded as f32. Two bytes are
stored in each f32 number,
 ```rust
// Encoding to NaNs
let s = "Hello, world!";
let x = ananas::to_nanvec(&s.as_bytes());
println!("{:?}", x); // prints '[NaN, NaN, NaN, NaN, NaN, NaN, NaN]'

// Decoding from NaNs
let y = String::from_utf8(ananas::from_nanvec(&x)).unwrap();
assert_eq!(y, s);
```
This crate provides convenience translators to and from strings. 

NaNs are propagated and this maintains the payload,
```rust
let x = ananas::str2nans("😎");
let y = [x[0] + 10000.0, x[1] / 0.0];
assert!(y[0].is_nan());
assert_eq!(ananas::nans2str(&y).unwrap(),  "😎");
```

This can produce strange behaviour when two NaNs meet, such as loss of
commutativity,
```rust
let x1 = ananas::str2nans("nan nan nan nan");
let x2 = ananas::str2nans("batman! batman!");
let y1 :Vec<_> = x1.iter().zip(&x2).map(|(a,b)| a*b).collect();
let y2 :Vec<_> = x1.iter().zip(&x2).map(|(a,b)| b * a).collect();
println!("{:?}", ananas::nans2str(&y1)); // nan nan nan nan
println!("{:?}", ananas::nans2str(&y2)); // batman! batman!
```

## Use case

`¯\_(ツ)_/¯`


## License



