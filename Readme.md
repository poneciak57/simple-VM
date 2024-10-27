# Simple virtual machine
An implementation of simple virtual machine following von Neumann's specifications


# Usage
Download this program from releases, ask me or compile it yourself. Then you can run it with '--help' flag.

Your pmc file should have this format:
```
0: LOAD . 0
1: STORE . 19
3: LOAD . 21 // it can have comments at the end of line
4: STORE . 18 // indexes do not need to be in correct order
5: -123 // you can also pass numbers
...
```
```
<index>: <command> <adressing_type> <adrress> // optional comments
```


To execute your *.pmc file run:
```
./program <path_to_pmc_file>
```
If you want to run it in interactive mode add '-i' or '--interactive' flag
After an execution program will create 'mem.out' file with 512 lines describing state of memory after your program execution.
If you want to look at your program memory during interactive mode you can press 'm' to dump current state of memory.

# Language description
Every instruction is encoded on 16 bits

| 15         | 11 - 14          | 9 - 10             | 0 - 8             |
|------------|------------------|--------------------|-------------------|
| Sign       | Instruction      | Addressing Type    | Adr / Operand     |

## Memory
Memory will be adressed as MEM or MEM[index] (like an array).
The machine has 512 memory registers. So it can take at most 512 instructions.
Each register is interpreted as instruction but can be loaded into an register as a number.

## Registers
| Name | Usage |
| ---- | -------------------------------------------- |
| IR | Stores current instruction                     |
| PC | Stores next instruction adress                 |
| OP | Stores current operand                         |
| AC | Is used for performing arithmetical operations |

## Instructions
| Instruction | Binary Code | Description                                  | In short          |
|-------------|-------------|----------------------------------------------|-------------------|
| NULL        | 0000        | No operation (NOP)                           | -                 |
| STOP        | 0001        | Stop execution                               | STOP              |
| LOAD        | 0010        | Load value from OP to AC                     | AC = OP           |
| STORE       | 0011        | Store AC value in memory at address OP       | M[OP] = AC        |
| JUMP        | 0100        | Sets PC to OP. Next instruction will change  | PC = OP           |
| JNEG        | 0101        | If AC < 0 then set PC to OP                  | AC < 0: PC = OP   |
| JZERO       | 0110        | If AC is 0 then set PC to OP                 | AC == 0: PC = OP  | 
| ADD         | 1000        | Add OP to AC                                 | AC += OP          |
| SUB         | 1001        | Subtract OP from AC                          | AC -= OP          |
| AND         | 1100        | Bitwise AND operation AC with OP             | AC = AC & OP      |
| OR          | 1101        | Bitwise OR operation AC with OP              | AC = AC | OP      |
| NOT         | 1110        | Bitwise NOT (complement) of AC               | AC = ~AC          |
| XOR         | 1111        | Bitwise XOR operation AC with OP             | AC = AC ^ OP      |
| SHL         | 1010        | Shift register AC left by OP bits            | AC << OP          |
| SHR         | 1011        | Shift register AC right by OP bits           | AC >> OP          |

## Addressing Types
| Operator | Binary Code | Description                            | In short               |
|----------|-------------|----------------------------------------|------------------------|
| .        | 00          | Immediate addressing                   | OP = IR.adr            |
| @        | 01          | Direct addressing                      | OP = MEM[IR.adr]       |
| *        | 10          | Indirect addressing                    | OP = MEM[MEM[IR.adr]]  |
| +        | 11          | Indexed addressing                     | OP = MEM[OP + AC]      |

## Operand
Operand or Address is represented by 10 bits. 9 bits at the end of an instruction represent the number and first bit of an instruction represents the sign of that number. If w use direct adrresing operand will be assigned with signed 16 bit value corresponding to binary representation of instruction/number laying in addressing memory field.


# Examples