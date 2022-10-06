section .text
  global _start

section .data
  num1 dw 300
  num2 dw 100
  msg1 db "Number 1 is bigger", 10
  msg2 db "Number 2 is bigger", 10
  msg3 db "The two numbers are equal", 10

_start:
  mov eax, [num1]
  cmp eax, [num2]
  je .equal
  cmp eax, [num2]
  jl .lesser
  cmp eax, [num2]
  jg .greater

.equal:
  mov eax, 4
  mov ebx, 1
  mov ecx, msg3
  mov edx, 25
  int 0x80
  jmp .exit

.greater:
  mov eax, 4
  mov ebx, 1
  mov ecx, msg1
  mov edx, 18
  int 0x80
  jmp .exit

.lesser:
  mov eax, 4
  mov ebx, 1
  mov ecx, msg2
  mov edx, 18
  int 0x80
  jmp .exit

.exit:
  mov ebx, 0
  mov eax, 1
  int 0x80
  
  

