section .multiboot_header
header_start:
    dd 0xE85250D6
    dd 0
    dd header_end - header_start
    dd 0x100000000 - (0xE85250D6 + 0 + (header_end - header_start))
    dw 0
    dw 0
    dd 8
header_end:

section .bss
align 16
stack_bottom:
    resb 16384
stack_top:

section .data
align 8
mb_magic: dd 0
mb_info: dd 0

gdt_start:
    dq 0
gdt_code:
    dw 0xFFFF
    dw 0
    db 0
    db 10011010b
    db 10101111b
    db 0
gdt_data:
    dw 0xFFFF
    dw 0
    db 0
    db 10010010b
    db 10001111b
    db 0
gdt_end:
gdt_descriptor:
    dw gdt_end - gdt_start - 1
    dd gdt_start

section .text
global start
extern kernel_main

bits 32
start:
    cli
    cmp eax, 0x36d76289
    jne no_multiboot

    mov [mb_magic], eax
    mov [mb_info], ebx

    mov edi, 0x1000
    mov cr3, edi
    xor eax, eax
    mov ecx, 4096
    rep stosd
    mov edi, cr3

    mov dword [edi], 0x2003
    add edi, 0x1000
    mov dword [edi], 0x3003
    add edi, 0x1000
    mov dword [edi], 0x0083
    add edi,8
    mov dword [edi], 0x200083

    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    lgdt [dword gdt_descriptor]

    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    o32 jmp 0x08:long_mode_start

no_multiboot:
    hlt

bits 64
long_mode_start:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    mov rsp, stack_top

    mov edi,[mb_magic] ;first argument - magic number
    mov rsi, [mb_info];second argument - multiboot info pointer
    call kernel_main

hang:
    cli
    hlt
    jmp hang
