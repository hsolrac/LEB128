## LEB128
LEB-128 or `Little Endia Base 128` is a variable-lenght code compression used to store  arbitrarily large integers in a small number of bytes. Used in the WebAssembly binary encoding for all integer literal.

##### TODOS:

- Unsiged LEB128
  - [x]  Represent the number in binary.
  - [x]  Extend the number for zero, seven multiples.
  - [x]  Divide the number into groups of seven bits.
  - [x]  Byte each group of seven bits.
  - [x]  Define the most significant bit in each byte (excluding the last byte).
- [ ]  Add create `inquire` for interactive cli
- [ ]  Add tests


