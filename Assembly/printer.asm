; int num = 125
; int divisor = 1
;
; while (divisor < num) {
;   divisor *= 10;
; }
;
; divisor /= 10;
;
; while (divisor != 0) {
;   print ( num/divisor );
;   divisor /= 10;
; }
;

section .bss
  current_digit resd 1
  current_divisor resd 1

section .data
  divisor dd 1
  num dd 854

  newline db 10

section .text
  global _start

_start:
  mov ecx, 0
  mov edx, 0

  mov eax, [num]
  mov ecx, [divisor]
  mov [current_divisor], ecx

  while1:
    cmp ecx, eax
    jge end_while1

    ; https://www.youtube.com/watch?v=4OP8p0owPUQ
    mov edx, 0
    mov eax, 10
    mov ecx, [current_divisor]
    mul ecx

    mov [current_divisor], edx

    jmp while1

  end_while1:


  while2:
    cmp ecx, 0
    je end_while2

    ; check if current_divisor is 0
    cmp eax, 0
    je end_while2


    mov eax, 0
    mov ebx, 0
    mov ecx, 0
    mov edx, 0

    ; need to zero edx register before performing the division
    ; https://stackoverflow.com/questions/45506439/division-of-two-numbers-in-nasm
    mov edx, 0 
    mov eax, [num]     ; dividend
    mov ecx, [current_divisor] ; divisor
    div ecx

    mov [num], edx
    add eax, 48 ; quotine is stored in eax, add 48 to display numbers
    mov [current_digit], eax

    mov eax, 4
    mov ebx, 1
    mov ecx, current_digit
    mov edx, 1
    int 0x80

    ; divide the divisor by 10
    mov edx, 0 
    mov eax, [current_divisor] ; dividend
    mov ecx, 10                ; divisor
    div ecx

    mov [current_divisor], eax ; update the value of current_divisor

    jmp while2
  
  end_while2:

  mov eax, 4
  mov ebx, 1
  mov ecx, newline
  mov edx, 1
  int 0x80

  mov eax, 1
  mov ebx, 0
  int 0x80
