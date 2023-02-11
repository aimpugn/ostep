# Summary

- [Summary](#summary)
    - [introduction](#introduction)

## introduction

how does the operating system virtualize resources?

physical -> virtual

refer OS
1. as virtual machine
2. as resource(CPU, memory, disk) manager

OS API for system calls -> standard library for applications

due to virtualization,
1. run many programs => sharing CPU
2. the programs can **concurrently** access their own instructions and data => sharing memory
3. the progrmas cna access devices => sharing disks and so forth
