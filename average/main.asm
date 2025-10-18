section .data
    x dd 5, 3, 2, 6, 1, 7, 4
    y dd 0, 10, 1, 9, 2, 8, 5
    count equ 7
    
    ; Строки для вывода
    avg_msg db "Average: ", 0
    newline db 10, 0
    minus_sign db "-", 0
    

section .bss
    sum resd 1
    average resd 1
    buffer resb 32      ; буфер для преобразования числа в строку

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
    mov ebx, [x + ecx * 4]    ; x[i]
    sub ebx, [y + ecx * 4]    ; x[i] - y[i]
    add eax, ebx              ; добавляем к сумме
    
    inc ecx
    cmp ecx, count
    jl sum_loop
    
    mov [sum], eax
    ret

calculate_avg:
    mov eax, [sum]
    cdq                 ; расширение знака eax в edx:eax
    mov ebx, count
    idiv ebx
    mov [average], eax
    ret


print_results:
    ; Вывод "Average: "
    mov rax, 1
    mov rdi, 1
    mov rsi, avg_msg
    mov rdx, 10
    syscall
    
    ; Вывод значения среднего
    mov eax, [average]
    call print_number
    
    ; Вывод новой строки
    mov rax, 1
    mov rdi, 1
    mov rsi, newline
    mov rdx, 1
    syscall
    
    ret

print_number:
    ; Функция для вывода 32-битного числа
    ; eax содержит число для вывода
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    
    mov edi, eax        ; сохраняем число в edi
    mov rsi, buffer     ; указатель на буфер
    add rsi, 31         ; начинаем с конца буфера
    mov byte [rsi], 0   ; нулевой байт в конце
    dec rsi
    
    mov ebx, 10         ; делитель
    xor ecx, ecx        ; счетчик цифр
    
    ; Проверка на отрицательное число
    mov eax, edi
    test eax, eax
    jns positive_number
    
    ; Отрицательное число
    neg eax
    mov edi, 1          ; флаг отрицательного числа
    jmp convert_digits
    
positive_number:
    xor edi, edi        ; флаг положительного числа
    
convert_digits:
    ; Преобразование числа в строку (в обратном порядке)
    xor edx, edx
    div ebx
    add dl, '0'
    mov [rsi], dl
    dec rsi
    inc ecx
    test eax, eax
    jnz convert_digits
    
    ; Если число было отрицательным, добавляем минус
    test edi, edi
    jz print_string
    mov byte [rsi], '-'
    dec rsi
    inc ecx
    
print_string:
    ; Вывод строки
    inc rsi             ; указатель на начало строки
    mov rax, 1          ; sys_write
    mov rdi, 1          ; stdout
    mov rdx, rcx        ; длина строки
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