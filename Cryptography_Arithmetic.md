## Notes from Cryptography Arithmetic

### Part I: Addition
#### Serial Adder

Full adder equations
$$
s_i = (x_i \oplus y_i) \oplus c_{i-1} \\
c_i = x_iy_i + (x_i \oplus y_i)c_{i-1}
$$

See [serial_adder](/cryptography_arithmetic/src/serial_adder.rs) for software implementation.

#### Carry-Ripple Adder
See [carry_ripple_adder](/cryptography_arithmetic/src/carry_ripple_adder.rs) for software implementation.
