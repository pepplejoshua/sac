.data
  hello:
    .string "Hello, from assembly! I am %d years old\n"
    .balign
  
.text
  .global main
  main:
    push {fp, lr}

    ldr r0, =hello
    mov r1, #23
    bl printf

    mov r0, #0

    pop {fp, pc}
    bx lr
    