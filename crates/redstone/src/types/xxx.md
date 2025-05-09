Regarding the missing overflow checks, the main math operations are:

### Number constructing

All of the numbers being taken from the payload are constructed by the `trim_end::<>` functions, which contain the overflow checking by using the embedded `try_into` functions or by checking the value length size.


### Median picking

That's constructed by using the most algebraic way, by using algebraic traits:

```rust
    fn avg(self, other: Self) -> Self {
        let one = T::from(1);
        let two = T::from(2);

        self.shr(one) + other.shr(one) + (self % two + other % two).shr(one)
    }
```

The number cannot be overflowed here because is `shr`-ed firstly. The operations on the values are not performed in another way.

### Payload signable bytes size

```rust
        let data_point_count = payload.try_trim_end(DATA_POINTS_COUNT_BS)?;
        let value_size = payload.try_trim_end(DATA_POINT_VALUE_BYTE_SIZE_BS)?;
        let size: usize = data_point_count * (value_size + DATA_FEED_ID_BS)
            + DATA_POINT_VALUE_BYTE_SIZE_BS
            + TIMESTAMP_BS
            + DATA_POINTS_COUNT_BS;
```

But having

`DATA_POINTS_COUNT_BS = 3`  ==> `data_point_count =< 2^24`

`DATA_POINT_VALUE_BYTE_SIZE_BS = 4` ==> `value_size =< 2^32`

`DATA_FEED_ID_BS = 32`

The size cannot be greater than

`2^24 * (2^32 + 32) + eps =<  2^57` which cannot exceed `2^64`

#### The current purposes

For the current purposes, the value_size returned is strictly defined in the protocol constants as `32 = 0x0020 = 2^5`
And also limited by the max `VALUE_SIZE = 32` in the `Sanitized` trait
and the `data_point_count` should not be generally exceeding `1024 = 2^10` (in most cases, that value is less than `32`),
so

`2^10 * (2^5 + 32) + eps =<  2^16`

We'll try to mark the value as `u64` for some extreme cases or add an overflow checking, as the usize is defined as 32-bit in WASM, for some further purposes (in case when something had changed in the protocol constants).

I think I haven't missed any other place when the arithmetic operations can be considered as overflowed.
