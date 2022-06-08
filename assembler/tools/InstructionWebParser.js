const list = [
		[
			"ADD",
			"Add Word"
		],
		[
			"ADDI",
			"Add Immediate Word"
		],
		[
			"ADDU",
			"Add Unsigned Word"
		],
		[
			"ADDIU",
			"Add Immediate Unsigned Word"
		],
		[
			"SUB",
			"Subtract Word"
		],
		[
			"SUBU",
			"Subtract Unsigned Word"
		],
		[
			"MUL",
			"Multiply Word to GPR"
		],
		[
			"MULT",
			"Multiply Word"
		],
		[
			"MULTU",
			"Multiply Unsigned Word"
		],
		[
			"MADD",
			"Multiply and Add Word to Hi, Lo"
		],
		[
			"MADDU",
			"Multiply and Add Unsigned Word to Hi, Lo"
		],
		[
			"MSUB",
			"Multiply and Subtract Word to Hi, Lo"
		],
		[
			"MSUBU",
			"Multiply and Subtract Unsigned Word to Hi, Lo"
		],
		[
			"DIV",
			"Divide Word"
		],
		[
			"DIVU",
			"Divide Unsigned Word"
		],
		[
			"DIV",
			"Divide Word to GPR"
		],
		[
			"DIVU",
			"Divide Unsigned Word to GPR"
		],
		[
			"REM",
			"Remainder after Word Division"
		],
		[
			"REMU",
			"Remainder after Unsigned Word Division"
		],
		[
			"CLO",
			"Count Leading Ones in Word"
		],
		[
			"CLZ",
			"Count Leading Zeroes in Word"
		],
		[
			"SEB",
			"Sign-Extend Byte"
		],
		[
			"SEH",
			"Sign-Extend Halfword"
		],
		[
			"SEQ",
			"Set on Equal"
		],
		[
			"SNE",
			"Set on Not Equal"
		],
		[
			"SLE",
			"Set on Less Than or Equal"
		],
		[
			"SLEU",
			"Set on Less Than or Equal Unsigned"
		],
		[
			"SLT",
			"Set on Less Than"
		],
		[
			"SLTU",
			"Set on Less Than Unsigned"
		],
		[
			"SGT",
			"Set on Greater Than or Equal"
		],
		[
			"SGTU",
			"Set on Greater Than or Equal Unsigned"
		],
		[
			"SGE",
			"Set on Greater Than or Equal"
		],
		[
			"SGEU",
			"Set on Greater Than or Equal Unsigned"
		],
		[
			"SLTI",
			"Set on Less Than Immediate"
		],
		[
			"SLTIU",
			"Set on Less Than Immediate Unsigned"
		],
		[
			"ABS",
			"Absolute Value"
		],
		[
			"NEG",
			"Negate Value"
		],
		[
			"NEGU",
			"Negate Value Unsigned"
		],
		[
			"AND",
			"And"
		],
		[
			"ANDI",
			"And Immediate"
		],
		[
			"OR",
			"Or"
		],
		[
			"ORI",
			"Or Immediate"
		],
		[
			"NOR",
			"Not Or"
		],
		[
			"XOR",
			"Exclusive Or"
		],
		[
			"XORI",
			"Exclusive Or Immediate"
		],
		[
			"NOT",
			"Bitwise Not"
		],
		[
			"ROL",
			"Rotate Left"
		],
		[
			"ROR",
			"Rotate Right"
		],
		[
			"ROTR",
			"Rotate Word Right"
		],
		[
			"ROTRV",
			"Rotate Word Right Variable"
		],
		[
			"SLL",
			"Shift Word Left Logical"
		],
		[
			"SLLV",
			"Shift Word Left Logical Variable"
		],
		[
			"SRA",
			"Shift Word Right Arithmetic"
		],
		[
			"SRAV",
			"Shift Word Right Arithmetic Variable"
		],
		[
			"SRL",
			"Shift Word Right Logical"
		],
		[
			"SRLV",
			"Shift Word Right Logical Variable"
		],
		[
			"LI",
			"Load Immediate"
		],
		[
			"LA",
			"Load Address"
		],
		[
			"LUI",
			"Load Upper Immediate"
		],
		[
			"LB",
			"Load Byte"
		],
		[
			"LBU",
			"Load Byte Unsigned"
		],
		[
			"LH",
			"Load Halfword"
		],
		[
			"LHU",
			"Load Halfword Unsigned"
		],
		[
			"LW",
			"Load Word"
		],
		[
			"LD",
			"Load Double Word"
		],
		[
			"SB",
			"Store Byte"
		],
		[
			"SH",
			"Store Halfword"
		],
		[
			"SW",
			"Store Word"
		],
		[
			"SD",
			"Store Double Word"
		],
		[
			"PUSH",
			"Push register to stack"
		],
		[
			"POP",
			"Pop register from stack"
		],
		[
			"BEGIN",
			"Create stack frame"
		],
		[
			"END",
			"Destroy stack frame"
		],
		[
			"MFHI",
			"Move From HI Register"
		],
		[
			"MFLO",
			"Move From LO Register"
		],
		[
			"MTHI",
			"Move To HI Register"
		],
		[
			"MTLO",
			"Move To LO Register"
		],
		[
			"MOVE",
			"Move"
		],
		[
			"MOVZ",
			"Move Conditional on Zero"
		],
		[
			"MOVN",
			"Move Conditional on Not Zero"
		],
		[
			"B",
			"Unconditional Branch"
		],
		[
			"BAL",
			"Branch and Link"
		],
		[
			"BEQ",
			"Branch on Equal"
		],
		[
			"BEQZ",
			"Branch on Equal to Zero"
		],
		[
			"BNE",
			"Branch on Not Equal"
		],
		[
			"BNEZ",
			"Branch on Not Equal to Zero"
		],
		[
			"BGE",
			"Branch on Greater Than or Equal"
		],
		[
			"BGEU",
			"Branch on Unsigned Greater Than or Equal"
		],
		[
			"BGEZ",
			"Branch on Greater Than or Equal to Zero"
		],
		[
			"BGT",
			"Branch on Greater Than"
		],
		[
			"BGTU",
			"Branch on Unsigned Greater Than"
		],
		[
			"BGTZ",
			"Branch on Greater Than Zero"
		],
		[
			"BLT",
			"Branch on Less Than"
		],
		[
			"BLTU",
			"Branch on Unsigned Less Than"
		],
		[
			"BLTZ",
			"Branch on Less Than Zero"
		],
		[
			"BLE",
			"Branch on Less Than or Equal"
		],
		[
			"BLEU",
			"Branch on Unsigned Less Than or Equal"
		],
		[
			"BLEZ",
			"Branch on Less Than or Equal to Zero"
		],
		[
			"BGEZAL",
			"Branch on Greater Than or Equal to Zero and Link"
		],
		[
			"BLTZAL",
			"Branch on Less Than Zero and Link"
		],
		[
			"J",
			"Jump"
		],
		[
			"JAL",
			"Jump and Link"
		],
		[
			"JR",
			"Jump Register"
		],
		[
			"JALR",
			"Jump and Link Register"
		],
		[
			"JALR",
			"Jump and Link Register"
		],
		[
			"SYSCALL",
			"Syscall"
		]
]

const fn_args = [
	[
		"add",
		"Rd, Rs, Rt"
	],
	[
		"addi",
		"Rt, Rs, Imm16"
	],
	[
		"addu",
		"Rd, Rs, Rt"
	],
	[
		"addiu",
		"Rt, Rs, Imm16"
	],
	[
		"sub",
		"Rd, Rs, Rt"
	],
	[
		"subu",
		"Rd, Rs, Rt"
	],
	[
		"mul",
		"Rd, Rs, Rt"
	],
	[
		"mult",
		"Rs, Rt"
	],
	[
		"multu",
		"Rs, Rt"
	],
	[
		"madd",
		"Rs, Rt"
	],
	[
		"maddu",
		"Rs, Rt"
	],
	[
		"msub",
		"Rs, Rt"
	],
	[
		"msubu",
		"Rs, Rt"
	],
	[
		"div",
		"Rs, Rt"
	],
	[
		"divu",
		"Rs, Rt"
	],
	[
		"div",
		"Rd, Rs, Rt"
	],
	[
		"divu",
		"Rd, Rs, Rt"
	],
	[
		"rem",
		"Rd, Rs, Rt"
	],
	[
		"remu",
		"Rd, Rs, Rt"
	],
	[
		"clo",
		"Rd, Rs"
	],
	[
		"clz",
		"Rd, Rs"
	],
	[
		"seb",
		"Rd, Rs"
	],
	[
		"seh",
		"Rd, Rs"
	],
	[
		"seq",
		"Rd, Rs, Rt"
	],
	[
		"sne",
		"Rd, Rs, Rt"
	],
	[
		"sle",
		"Rd, Rs, Rt"
	],
	[
		"sleu",
		"Rd, Rs, Rt"
	],
	[
		"slt",
		"Rd, Rs, Rt"
	],
	[
		"sltu",
		"Rd, Rs, Rt"
	],
	[
		"sgt",
		"Rd, Rs, Rt"
	],
	[
		"sgtu",
		"Rd, Rs, Rt"
	],
	[
		"sge",
		"Rd, Rs, Rt"
	],
	[
		"sgeu",
		"Rd, Rs, Rt"
	],
	[
		"slti",
		"Rt, Rs, Imm16"
	],
	[
		"sltiu",
		"Rt, Rs, Imm16"
	],
	[
		"abs",
		"Rt, Rs"
	],
	[
		"neg",
		"Rt, Rs"
	],
	[
		"negu",
		"Rt, Rs"
	],
	[
		"and",
		"Rd, Rs, Rt"
	],
	[
		"andi",
		"Rt, Rs, Imm16"
	],
	[
		"or",
		"Rd, Rs, Rt"
	],
	[
		"ori",
		"Rt, Rs, Imm16"
	],
	[
		"nor",
		"Rd, Rs, Rt"
	],
	[
		"xor",
		"Rd, Rs, Rt"
	],
	[
		"xori",
		"Rt, Rs, Imm16"
	],
	[
		"not",
		"Rt, Rs"
	],
	[
		"rol",
		"Rd, Rt, Rs"
	],
	[
		"ror",
		"Rd, Rt, Rs"
	],
	[
		"rotr",
		"Rd, Rt, a"
	],
	[
		"rotrv",
		"Rd, Rt, Rs"
	],
	[
		"sll",
		"Rd, Rt, a"
	],
	[
		"sllv",
		"Rd, Rt, Rs"
	],
	[
		"sra",
		"Rd, Rt, a"
	],
	[
		"srav",
		"Rd, Rt, Rs"
	],
	[
		"srl",
		"Rd, Rt, a"
	],
	[
		"srlv",
		"Rd, Rt, Rs"
	],
	[
		"li",
		"Rt Imm32"
	],
	[
		"la",
		"Rt Label"
	],
	[
		"lui",
		"Rt, Imm16"
	],
	[
		"lb",
		"Rt, Offset16(Rb)"
	],
	[
		"lbu",
		"Rt, Offset16(Rb)"
	],
	[
		"lh",
		"Rt, Offset16(Rb)"
	],
	[
		"lhu",
		"Rt, Offset16(Rb)"
	],
	[
		"lw",
		"Rt, Offset16(Rb)"
	],
	[
		"ld",
		"Rt, Offset16(Rb)"
	],
	[
		"sb",
		"Rt, Offset16(Rb)"
	],
	[
		"sh",
		"Rt, Offset16(Rb)"
	],
	[
		"sw",
		"Rt, Offset16(Rb)"
	],
	[
		"sd",
		"Rt, Offset16(Rb)"
	],
	[
		"push",
		"Rs"
	],
	[
		"pop",
		"Rs"
	],
	[
		"begin",
		""
	],
	[
		"end",
		""
	],
	[
		"mfhi",
		"Rd"
	],
	[
		"mflo",
		"Rd"
	],
	[
		"mthi",
		"Rd"
	],
	[
		"mtlo",
		"Rd"
	],
	[
		"move",
		"Rt, Rs"
	],
	[
		"movz",
		"Rd, Rs, Rt"
	],
	[
		"movn",
		"Rd, Rs, Rt"
	],
	[
		"b",
		"Offset16"
	],
	[
		"bal",
		"Offset16"
	],
	[
		"beq",
		"Rs, Rt, Offset16"
	],
	[
		"beqz",
		"Rs, Offset16"
	],
	[
		"bne",
		"Rs, Rt, Offset16"
	],
	[
		"bnez",
		"Rs, Offset16"
	],
	[
		"bge",
		"Rs, Rt , Offset16"
	],
	[
		"bgeu",
		"Rs, Rt , Offset16"
	],
	[
		"bgez",
		"Rs, Offset16"
	],
	[
		"bgt",
		"Rs, Rt , Offset16"
	],
	[
		"bgtu",
		"Rs, Rt , Offset16"
	],
	[
		"bgtz",
		"Rs, Offset16"
	],
	[
		"blt",
		"Rs, Rt , Offset16"
	],
	[
		"bltu",
		"Rs, Rt , Offset16"
	],
	[
		"bltz",
		"Rs, Offset16"
	],
	[
		"ble",
		"Rs, Rt , Offset16"
	],
	[
		"bleu",
		"Rs, Rt , Offset16"
	],
	[
		"blez",
		"Rs, Offset16"
	],
	[
		"bgezal",
		"Rs, Offset16"
	],
	[
		"bltzal",
		"Rs, Offset16"
	],
	[
		"j",
		"Address26"
	],
	[
		"jal",
		"Address26"
	],
	[
		"jr",
		"Rs"
	],
	[
		"jalr",
		"Rs"
	],
	[
		"jalr",
		"Rd, Rs"
	],
	[
		"syscall",
		""
	]
];

const ArgMap = {
	"Rd": "Register",
	"Rs": "Register",
	"Rt": "Register",
	"Imm16": "Immediate",
	"Imm32": "Immediate",
	"a": "Immediate",
	"Label": "Label",
	"Offset16": "Offset",
	"Offset16(Rb)": "Offset",
	"Address26": "Address",
}

const formattedNames = list.map(x => x[1].replaceAll(/[ ,-]/g, ""))

const padSize = Math.max(...formattedNames.map(x => x.length));
const maxInstructSize = Math.max(...Object.values(ArgMap).map(x => x.length)) + "InstructionArgType::".length  + 3;
for(const [i, [inst, arg_str]] of fn_args.entries()) {
	const formattedName = formattedNames[i];

	let args = arg_str.split(" ").map(x => x.trim().replaceAll(",", "")).filter(x => x.length > 0);
	
	args = args.map(x => `InstructionArgType::${ArgMap[x]}`);
	while(args.length < 3) {
		args.push("InstructionArgType::Empty")
	}

	let [a, b, c] = args;
	a += ",";
	b += ",";
	console.log(`Inst::${formattedName.padEnd(padSize)} => (${a.padEnd(maxInstructSize)} ${b.padEnd(maxInstructSize)} ${c}),`);
}