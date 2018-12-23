BITS 64

_start:
  xor rax, rax
  xor rdi, rdi
  mov al, 60
  syscall
  ud2
