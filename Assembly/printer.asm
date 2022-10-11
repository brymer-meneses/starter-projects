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
  divisor dd 100
  num dd 129

section .text
  global _start

_start:
  mov ecx, 0

  mov eax, [num]
  mov ecx, [divisor]
  mov [current_divisor], ecx

  while:
    cmp ecx, 0
    je end_while

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

    jmp while
  
  end_while:

  mov eax, 1
  mov ebx, 0
  int 0x80
