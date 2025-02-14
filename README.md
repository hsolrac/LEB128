## LEB128
LEB-128 or `Little Endia Base 128` is a variable-lenght code compression used to store  arbitrarily large integers in a small number of bytes. Used in the WebAssembly binary encoding for all integer literal.

##### TODOS:

- Unsiged LEB128

- [ ]  Represent the number in binary.
- [ ]  Extend the number for zero, seven multiples.
- [ ]  Divide the number into groups of seven bits.
- [ ]  Byte each group of seven bits.
- [ ]  Define the most significant bit in each byte (excluding the last byte).


