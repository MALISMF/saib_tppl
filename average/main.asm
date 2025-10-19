section .data
    x dd 5, 3, 2, 6, 1, 7, 4
    y dd 0, 10, 1, 9, 2, 8, 5
    count equ 7
    
    avg_msg db "Average: ", 0
    newline db 10, 0
    minus_sign db "-", 0
    

section .bss
    sum resd 1
    average resd 1
    buffer resb 32

section .text
    global _start

_start:
    call calculate_sum
    call calculate_avg
    call print_results
    call exit_program

calculate_sum:
    mov eax, 0
    mov ecx, 0

sum_loop:
    mov ebx, [x + ecx * 4]
    sub ebx, [y + ecx * 4]
    add eax, ebx
    
    inc ecx
    cmp ecx, count
    jl sum_loop
    
    mov [sum], eax
    ret

calculate_avg:
    mov eax, [sum]
    cdq
    mov ebx, count
    idiv ebx
    mov [average], eax
    ret


print_results:
    mov rax, 1
    mov rdi, 1
    mov rsi, avg_msg
    mov rdx, 10
    syscall
    
    mov eax, [average]
    call print_number
    
    mov rax, 1
    mov rdi, 1
    mov rsi, newline
    mov rdx, 1
    syscall
    
    ret

print_number:
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    
    mov edi, eax
    mov rsi, buffer
    add rsi, 31
    mov byte [rsi], 0
    dec rsi
    
    mov ebx, 10
    xor ecx, ecx
    
    mov eax, edi
    test eax, eax
    jns positive_number
    
    neg eax
    mov edi, 1
    jmp convert_digits
    
positive_number:
    xor edi, edi
    
convert_digits:
    xor edx, edx
    div ebx
    add dl, '0'
    mov [rsi], dl
    dec rsi
    inc ecx
    test eax, eax
    jnz convert_digits
    
    test edi, edi
    jz print_string
    mov byte [rsi], '-'
    dec rsi
    inc ecx
    
print_string:
    inc rsi             
    mov rax, 1
    mov rdi, 1
    mov rdx, rcx       
    syscall
    
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rax
    ret

exit_program:
    mov rax, 60
    mov rdi, 0
    syscall