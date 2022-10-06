
section .bss
  f1 resb 0
  f2 resb 1
  f3 resb 1
  current_iteration 0
  limit resb 10

section .text
  global _start

iterate:
  

_start:
  mov eax, [f1]
  add eax, [f2]
  mov [f3], eax


