

section .data
; insert a number here
  num1 dw 2 
  num2 dw 5

section .bss
  result resb 1

section .text
  global _start
 
_start:
  
  mov eax, [num1]
  add eax, [num2]
  add eax, 48
  mov [result], eax

  mov ecx, result
  mov eax, 4
  mov ebx, 1
  mov edx, 1
  int 0x80

  mov eax, 1
  mov ebx, 0
  int 0x80

  


