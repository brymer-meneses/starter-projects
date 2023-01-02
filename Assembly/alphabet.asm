
section .bss
  letter resd 1

section .data
  counter dd 27
  index   dd 1

section .text
  global _start

_start:
  
  loop_start:
    mov eax, [index]
    add eax, 64
    mov [letter], eax

    mov eax, 4
    mov ebx, 1
    mov ecx, letter
    mov edx, 1
    int 0x80

    mov eax, [index]
    inc eax
    mov [index], eax
    
    mov ecx, [counter]
    dec ecx
    mov [counter], ecx

    loop loop_start

  mov eax, 1
  mov ebx, 0
  int 0x80
   
