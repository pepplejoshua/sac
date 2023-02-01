.text
  .global main
  main:
    push {fp, lr} // store caller's fp and return addr
    mov fp, sp // set fp to current sp

    mov r0, #0 // set r0 (arg 1) to 0
    bl addFourtyTwo // call addFourtyTwo
    sub r0, r0, #3 // subtract 3 from r0 (which should be unclobbered)

    mov sp, fp // reset sp back for caller's stack frame
    pop {ip, lr} // get return addr of caller
    bx lr // return to caller

  addFourtyTwo:
    add r0, r0, #42 // add 42 to r0
    bx lr // return to caller
  