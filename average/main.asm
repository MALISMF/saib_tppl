section .data
    x dd 5, 3, 2, 6, 1, 7, 4
    y dd 0, 10, 1, 9, 2, 8, 5
    count equ 7

section .bss
    sum resq 1
    average resq 1

section .text
    global _start

_start:
    call calculate_sum
    call calculate_avg
    call exit_program

calculate_sum:
    mov rax, 0
    mov rcx, count

sum_loop:
    dec rcx
    mov rbx, [x + rcx * 4]
    sub rbx, [y + rcx * 4]
    add rax, rbx
    
    loop sum_loop
    
    mov [sum], rax
    ret

calculate_avg:
    mov rax, [sum]
    cqo
    mov rbx, count
    idiv rbx
    mov [average], rax
    ret

exit_program:
    mov rax, 60
    mov rdi, 0
    syscall