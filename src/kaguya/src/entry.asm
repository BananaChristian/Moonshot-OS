section .text.prologue
global start
extern kmain
[bits 16]

start:
  cli
  lgdt [gdt_descriptor]
  mov eax, cr0
  or eax,0x1
  mov cr0, eax

  jmp 0x08:init_pm

align 8
gdt_start:
gdt_null_segment:
  dq 0x0 ;64 bits of zeros

gdt_code_segment_32:
  dw 0xFFFF ;the limit
  dw 0x0000 ;the base
  db 0x00 ;the mid of the base
  db 10011010b ;access byte (Permissions for code segment)
  db 11001111b ;Flags + last 4 bits of 0xFFFFF
  db 0x00 ;Last 8 bits of low

gdt_data_segment_32:
  dw 0xFFFF 
  dw 0x0000 
  db 0x00 
  db 10010010b ; access byte(Permissions for data segment) 
  db 11001111b 
  db 0x00

gdt_code_segment_64:
    dw 0x0000     ; Limit ignored in 64-bit
    dw 0x0000     ; Base ignored in 64-bit
    db 0x00 
    db 10011010b  ; Access byte (Code, Readable)
    db 00100000b  ; Flag Bit 5 set to 1 enables Long Mode!
    db 0x00
gdt_end:

gdt_descriptor:
    dw gdt_end -gdt_start - 1
    dd gdt_start


;PROTECTED MODE
[bits 32]
init_pm:
  mov ax , 0x10
  mov ds , ax
  mov ss, ax
  mov es, ax
  mov gs, ax
  mov fs, ax

  mov ebp, 0x90000
  mov esp, ebp

set_up_page_tables:
  mov edi, 0x10000
  mov cr3, edi
  xor eax, eax
  mov ecx, 4096
  rep stosd

  mov dword [0x10000], 0x11003
  mov dword [0x11000], 0x12003
  mov dword [0x12000], 0x13003

  mov edi, 0x13000
  mov ebx, 0x00000003
  mov ecx, 512

.map_loop:
  mov [edi], ebx ; Move the start of RAM with the present and writeable flags with what is in the edi (0x13000)
  add edi, 8; Add 8 to the edi this moves slot per slot each slot is 8 bytes
  add ebx, 4096
  loop .map_loop

flip_hardware_switches:
  mov eax, 0x10000
  mov cr3, eax ;Pass level 4 table's address to the CPU via cr3

  ;Enable PAE(Physical address Extension by flipping the 5th of what was in cr4)
  mov eax,cr4
  bts eax, 5
  mov cr4, eax

  ;Enable long mode from EFER MSR register
  mov ecx, 0xC0000080 ;Select the EFER register from the MSR
  rdmsr
  bts eax, 8 ; Flip bit 8 to 1
  wrmsr ; Write back the result 

  ;Flip bit 31 to 1 to enable paging
  mov eax, cr0
  bts eax, 31
  mov cr0, eax

  jmp 0x18:long_mode_start

;LONG MODE
[bits 64]
long_mode_start:
  mov ax, 0x10
  mov ds, ax
  mov ss, ax
  mov es, ax

  mov rbp, 0x90000
  mov rsp, rbp

  call kmain

  jmp $



