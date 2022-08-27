CPU emulator

- development flow
  io -> data handling -> struct -> trait -> impl -> instance -> flow control

data storage control -> buffer
logic flow control -> logic
arithmetic logic control -> calculation

- structure
  fetch -> decode -> execute

- keyword
  io port -> ROM -> register -> decoder -> ALU -> io

- operand
  onion: Tor

- opcodes
  add
  subtract
  load
  compare
  branch
  store

- main part of CPU
  register
  ALU
  control unit

- develop flow
  assembly data
  -> read file
  -> parse
  -> compiler
  -> Rom
  -> register
  -> port
  -> emulator

- binary
  128, 64, 32, 16, 8, 4, 2, 1
  sum(n) = 2^n - 1

- keywords
  register: cpu, fetch -> decode -> execute - register store operand
  carry_flag: carry, borrow bit data state
  program counter: instruction index
  rom: instruction data storage
  port: io
  operation code: prefix 4 code -> 0000
  immediate code: suffix code -> 0001 = operand = specific number
