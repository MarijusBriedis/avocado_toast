[org  0x7c00]
[bits 16]

entry:
    mov ax, 0x0003
    int 0x10

    mov di, 0xb800
    mov es, di
    xor di, di
    mov ax, 0x0f41
    mov cl, 40
    rep stosw

    cli
    hlt

times 510-($-$$) db 0
dw 0xaa55