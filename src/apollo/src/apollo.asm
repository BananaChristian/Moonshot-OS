;Apollo is the official bootloader for the moonshot operating system
;For now it is a simple mono-stage bootloader

[org 0x7C00]
[bits 16]

KAGUYA_LOAD_SEGMENT equ 0x0000
KAGUYA_LOAD_OFFSET equ 0x1000

start:
  cli
  xor ax, ax
  mov ds,ax
  mov es,ax
  mov ss,ax
  mov sp, 0x7C00
  sti

  mov ah,0x00
  mov al,0x03
  int 0x10

  mov si, greet_msg
  call print_string

  mov si, load_msg
  call print_string
  call load_kaguya

  mov si, jump_msg
  call print_string

  jmp KAGUYA_LOAD_SEGMENT:KAGUYA_LOAD_OFFSET

load_kaguya:
  mov ax, KAGUYA_LOAD_SEGMENT
  mov es, ax
  mov bx, KAGUYA_LOAD_OFFSET

  mov ah, 0x02
  mov al, 16
  mov ch, 0
  mov cl, 2
  mov dh, 0
  
  int 0x13
  jc disk_error

  cmp al, 16
  jne disk_error
  ret

disk_error:
  mov si, error_msg
  call print_string
  hlt
  jmp $

print_string:
  lodsb
  or al,al
  jz .done
  mov ah, 0x0E
  int 0x10
  jmp print_string
.done:
  ret


greet_msg db "Apollo initialized...", 0x0D, 0x0A, 0
load_msg  db "Loading Kaguya from disk...", 0x0D, 0x0A, 0
jump_msg  db "Jumping to Kaguya...", 0x0D, 0x0A, 0
error_msg db "CRITICAL: Disk read failed! Boot halted.", 0x0D, 0x0A, 0

times 510-($-$$) db 0
dw 0xAA55
