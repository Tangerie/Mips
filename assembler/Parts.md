[Instruction Set](https://www.dsi.unive.it/~gasparetto/materials/MIPS_Instruction_Set.pdf)

# TODO
- [X] Strings
- [X] Words
    - [X] Directives
    - [X] Identifiers
- [X] Addresses
- [X] Registers
- [X] Labels
- [X] Immediates
- [X] Operators (+-)
- [X] Instructions
- [ ] Parse reg + operand in instruction

# Parts

```re
<name> = ([A-Za-z_0-9-]+)
```

There are a few key component in a standard MIPS program (without custom stuff yet)

**Directives**

| Name    | Data            | Size (bytes)   |
| ------- | --------------- | -------------- |
| .text   |                 |                |
| .data   |                 |                |
| .ascii  | `string`        | `1`/char       |
| .asciiz | `string`        | `1`/char `+ 1` |
| .byte   | `val [, ...]`   | `1`            |
| .half   | `val [, ...]`   | `2`            |
| .word   | `val [, ...]`   | `4`            |
| .float  | `val [, ...]`   | `8`            |
| .double | `val [, ...]`   | `16`           |
| .globl  | `label [, ...]` |                |
| .space  | `n`             | `n`            |
| .align  | `N`             | Up to `N - 1`  |

```re
\.(<name>)
```

**Instructions**
```
<label>: <opcode> <operand1>, <operand2>, <operand3>
```

Labels are optional and mark the memory address of the current instruction. These can exist without following operands

There can be up to `3` operands, each separated by a comma

**Operands**
| Operand | Description                                      |
| ------- | ------------------------------------------------ |
| Rn      | A register                                       |
| Imm     | A literal/constant (decimal, hex, octal or char) |
| Label   | A label                                          |
| Addr    | A memory address                                 |

**Address**
| Format           | Description                |
| ---------------- | -------------------------- |
| Label            |                            |
| (Rn)             | Value stored in Rn         |
| Imm(Rn)          | Value stored in Rn + Imm   |
| Label(Rn)        | Value stored in Rn + Label |
| Label +- Imm     |                            |
| Label +- Imm(Rn) | Label +- (Rn + Imm)        |

**syscall**
A special command

*Printing*
| $v0 | Argument  | Argument Type | Result | Result Type |
| --- | --------- | ------------- | ------ | ----------- |
| 1   | $a0       | int           |        |             |
| 2   | $f12      | float         |        |             |
| 3   | $f12/$f13 | double        |        |             |
| 4   | $a0       | char *        |        |             |
| 11  | $a0       | char          |        |             |

*Reading*
| $v0 | Argument | Argument Type | Result  | Result Type |
| --- | -------- | ------------- | ------- | ----------- |
| 5   |          |               | $v0     | int         |
| 6   |          |               | $f0     | float       |
| 7   |          |               | $f0/$f1 | double      |
| 8   |          |               | $a0     | char *      |
| 8   |          |               | $a1     | int         |
| 12  |          |               | $v0     | char        |

*Files*
| $v0 | Argument | Argument Type | Result | Result Type |
| --- | -------- | ------------- | ------ | ----------- |

*Process*
| $v0 | Argument | Argument Type | Result | Result Type |
| --- | -------- | ------------- | ------ | ----------- |
| 9   | $a0      | int           |        |             |
| 10  |          |               |        |             |
| 17  | $a0      | int           |        |             |