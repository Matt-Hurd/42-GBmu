# 42-GBmu
A GB Emulator Written in Rust

## What is GBmu?
GBmu is a project in the 42 curriculum. The goals of the project are to create a fully functioning app, as well as learn about emulation and electronic documentation.  

## Why Rust?
I have no prior knowledge of Rust, so I figure this will be a fun time learning it the hard way.

## Progress
- I've implemented nearly all of the opcodes, and the timings seem to be nearly perfect.  
- The GPU is implemented, but has some timing issues that are causing lines to be drawn a few cycles too early.  
- The MMU implementation needs more work, but works for the current scope of the project.  
- Currently only have MBU0 supported, but the infrastructure will handle the others being added.  
- GPU and gamepad interrupts are implemented.  

## Images

![GBmu](http://i.imgur.com/pZFsseW.png)
