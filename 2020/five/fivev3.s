  .section  __DATA,__data
.input:
  .string   "BFFFBBFRRR"
.format:
  .string   "%d\n"

  .section  __TEXT,__text
  .globl    _main
_main:
  subq    $24, %rsp
  movq    $0, 1(%rsp)

  leaq    .input(%rip), %r8   # char* input = "..."
  movq    $0, %rdx            # counter = 0

.top:
  movb    (%r8,%rdx), %r9b    # copy input[counter] single char
  cmpb    $0, %r9b            # if input[counter] == \0
  je      .exit               # jump to exit

  andb    $4, %r9b            # y = foo[i] & 4;
  shrb    $2, %r9b            # y >>= 2;
  xorb    $1, %r9b            # y ^= 1;
  movb    $9, %cl             # 9 -> cl
  subb    %dl, %cl            # 9 - counter
  shlw    %cl, %r9w           # y <<= cl;
  orw     %r9w, 1(%rsp)       # seatid = seatid | y;

  addq    $1, %rdx            # counter++
  jmp     .top                # loop

.exit:
  movq    1(%rsp), %rsi       # seatid 
  leaq    .format(%rip), %rdi # format string
  xorq    %rax, %rax
  callq   _printf             # printf("%d",seatid);

  addq    $24, %rsp           # deallocate space on stack

  movq    $0x2000001, %rax    # macOS exit syscall $60 on Linux
  movq    $1, %rdi            # return code
  syscall
