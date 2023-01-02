
section .data
  score dd 65

section .bss
  grade resd 1

section .text
  global _start

_start:
  mov eax, [score]

  ; 90 and above
  cmp eax, 90
  jge confirm_uno

  ; 80 - 89
  cmp eax, 89
  jle check_dos

  ; 60 -79
  cmp eax, 79
  jle check_tres

  ; 55-59
  cmp eax, 59
  jle check_kwatro


  check_dos:
    cmp eax, 80
    jge confirm_dos

  check_tres:
    cmp eax, 60
    jge confirm_tres

  check_kwatro:
    cmp eax, 55
    jge confirm_kwatro

  ; 54 and below
  confirm_singko:
    mov eax, 5
    jmp exit

  confirm_kwatro:
    mov eax, 4
    jmp exit

  confirm_tres:
    mov eax, 3
    jmp exit

  confirm_dos:
    mov eax, 2
    jmp exit

  confirm_uno:
    mov eax, 1
    jmp exit

  ; exit code
  exit:
    add eax, 48
    mov [grade], eax

    mov eax, 4
    mov ebx, 1
    mov ecx, grade
    mov edx, 1
    int 0x80

    mov eax, 1
    mov ebx, 0
    int 0x80

