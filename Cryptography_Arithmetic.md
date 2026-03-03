## Notes from Cryptography Arithmetic

### Part I: Addition
#### Serial Adder

$$
s_i = (x_i \oplus y_i) \oplus c_{i-1} \\
c_i = x_iy_i + (x_i \oplus y_i)c_{i-1}
$$