section .bss
   tape resb 3
section .text
    global _start
_start:
    mov rbx, 0
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
again0: 
    cmp byte tape[rbx], 0
    je skip1
    cmp rbx, 3
    jne skip3
    mov rbx, 0
    skip3: 
    inc rbx
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    cmp rbx, 0
   jne skip4
    mov rbx, 3
    skip4: 
    dec rbx
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
    jne again0
skip1: 
    cmp rbx, 3
    jne skip5
    mov rbx, 0
    skip5: 
    inc rbx
    cmp rbx, 3
    jne skip6
    mov rbx, 0
    skip6: 
    inc rbx
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
    inc byte tape[rbx]
again6: 
    cmp byte tape[rbx], 0
    je skip7
    cmp rbx, 0
   jne skip9
    mov rbx, 3
    skip9: 
    dec rbx
    call _print
    inc byte tape[rbx]
    cmp rbx, 3
    jne skip10
    mov rbx, 0
    skip10: 
    inc rbx
    dec byte tape[rbx]
    cmp byte tape[rbx], 0
    jne again6
skip7: 
    call _input

_end:
    mov rax, 60
    mov rdi, 0
    syscall
_print:
    mov rax, 1
    mov rdi, 1
    lea rsi, [tape+rbx]
    mov rdx, 1
    syscall
    ret
_input:
    mov rax, 0
    mov rdi, 0
    lea rsi, [tape+rbx]
    mov rdx, 1
    syscall
    ret
