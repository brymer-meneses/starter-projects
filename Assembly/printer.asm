
; PSEUDOCODE
; num = 12345
; temp = num
; divisor = 1
; len = 1
;
; while temp > 10:
;     divisor *= 10
;     temp /= 10
;     len += 1
;
; for (int i=len; i != 0; i--):
;     digit = (num // divisor) % 10
;     print(digit)
;     divisor //= 10


section .data
  num     dd 1234591003
  divisor dd 1
  len     dd 1
  newline db 10

section .bss
  temp    resd 1
  digit   resd 1

section .text
  global _start

_start:

  ; temp = num
  mov eax, [num]
  mov [temp], eax

  ; get highest multiple of 10 less than num
  ; while temp > 10 do this
  while1:
    mov eax, [temp]
    cmp eax, 10
    jle end_while1

    ; divisor *= 10
    mov eax, [divisor]
    mov ecx, 10
    mul ecx,
    mov [divisor], eax

    ; temp /= 10
    mov edx, 0
    mov eax, [temp]
    mov ecx, 10
    div ecx
    mov [temp], eax

    ; len += 1
    mov eax, [len]
    inc eax
    mov [len], eax

    jmp while1

  end_while1:

  while2:
    mov eax, [len]
    cmp eax, 0
    jle end_while2

    ; 1. compute (num/divisor) % 10

    ; num/divisor
    mov edx, 0
    mov eax, [num]
    mov ecx, [divisor]
    div ecx

    ; (num/divisor) is stored in eax
    mov edx, 0
    mov ecx, 10
    div ecx

    ; add and get the remainder which is equivalent to mod10
    add edx, 48
    mov [digit], edx

    ; 2. Print value from step 1
    mov eax, 4
    mov ebx, 1
    mov ecx, digit
    mov edx, 1
    int 0x80

    ; 3. Continuously divide divisor by 10
    mov edx, 0
    mov eax, [divisor]
    mov ecx, 10
    div ecx
    mov [divisor], eax

    ; 4. Decrement len
    mov eax, [len]
    dec eax
    mov [len], eax

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
    

  ; while len > 0
    
    
