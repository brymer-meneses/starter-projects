
section .data
  message   dd "hello there"
  len       dd $ - message
  key       db 1
  index     dd 0
  newline   db 10

section .bss
  letter  resb 1

section .text
  global _start

_start:
  
  ; for (int i=len; i>0; i--)

  loop_start:
    ; access letter with index as its offset
    mov ebx, [index]
    mov eax, [message + ebx]  

    cmp al, 32
    je skip

    add eax, [key]    ; add encryption key
    skip:

    mov [letter], eax


    mov eax, 4
    mov ebx, 1
    mov ecx, letter
    mov edx, 1
    int 0x80


    mov eax, [index]
    inc eax
    mov [index], eax

    mov ecx, [len]
    dec ecx
    mov [len], ecx

    loop loop_start


  ; add newline
  mov eax, 4
  mov ebx, 1
  add ecx, newline
  mov edx, 1
  int 0x80

  mov eax, 1
  mov ebx, 0
  int 0x80

  
