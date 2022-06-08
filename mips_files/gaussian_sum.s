.globl main
.text 

main: #
li $v0, 4
la $a0, prompt1
syscall
li $v0, 5
syscall
move $t0, $v0
li $v0, 4
la $a0, prompt2
syscall
li $v0, 5
syscall
move $t1, $v0
sub $t2, $t1, $t0
addi $t2, $t2, 1
add $t3, $t0, $t1
mul $t2, $t2, $t3
li $t5, 2
div $t2, $t2

test: mflo $t2
li $v0, 4
la $a0, answer1
syscall
li $v0, 1
move $a0, $t0
syscall
li $v0, 4
la $a0, answer2
syscall
li $v0, 1
move $a0, $t1
syscall
li $v0, 4
la $a0, answer3
syscall
li $v0, 1
move $a0, $t2
syscall
li $v0, 4
la $a0, newline
syscall
li $v0, 10
syscall

.data 

prompt1: .asciiz "Enter first number: "
prompt2: .asciiz "Enter second number: "
answer1: .asciiz "The sum of all numbers between "
answer2: .asciiz " and "
answer3: .asciiz " (inclusive) is: "
newline: .asciiz "\n"