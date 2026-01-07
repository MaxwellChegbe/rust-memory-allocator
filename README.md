# Rust Memory Allocators

This repository is a collection of memory allocator implementations written in Rust. My goal is to move from simple allocation like Bumps to more complex, production-ready designs while documenting the underlying systems concepts.

## Project Goal
To understand memory management by building it from scratch. This project focuses on the "why" of memory layout alignment requirements, pointer arithmetic, and cache efficiency, rather than just using high-level abstractions.

## Implementations

### 1. Bump Allocator
A basic "next-pointer" allocator. It is extremely fast for scenarios where all objects can be deallocated at once.

* Key Learning: Memory alignment. I replaced a linear while loop with a bitwise "mask and snap" operation to ensure types are stored on their required boundaries in $O(1)$ time.
* Logic: (addr + align - 1) & !(align - 1)

### 2. Arena Allocator (Next)
Building on the bump allocator, the Arena will handle multiple "chunks" or regions of memory to allow for growth without losing the speed of the bump strategy.

---

## Technical Notes on Alignment
Data types (like u32 or f64) cannot be stored at arbitrary memory addresses. They must be "aligned" to addresses that are multiples of their size. This repository implements manual alignment logic to ensure the CPU can access the allocated data efficiently without crashing or losing performance.

## Status
* January 2026: Started with the basic Bump Allocator. 
* Current Focus: Transitioning to Arena-based allocation.

---
Maxwell Chegbe Final Year CS Student | Abuja, Nigeria
