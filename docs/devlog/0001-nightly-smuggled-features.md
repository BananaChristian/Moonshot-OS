# 0001 — Nightly, and the features that were smuggled from dependecies

## Goal
M0 Phase 1: Toolchain pinning

## What I learned
The role of rust-toolchain.toml in instructing rustup to immediately set up the right toolchain for a given directory

Not specifying features doesnt mean no features it just means default features

Nightly features arent guaranteed to be stable they change per compiler version this was witnessed when the x86_64 crate had the step_trait dependency but it was implemented for an older nightly compiler version(2023 version)

cfg is a command to the rust compiler specifying that if this feature is enabled then compile this when I disabled the step_trait it stopped a serious compile time error caused by it


## The bug I killed
Feature attributes are only allowed in nightly builds of the rust compiler using a stable build caused some crates(like x86_64) to throw this error with the code E0554 the fix was pinning the project to use the nightly build

The error E0046 which basically means if u pick up a contract u must provide implementations for all its methods, I encountered this when the x86_64 version 0.14 crate used the step_trait and the compiler complained about this error because the shape of the Step trait had changed per  the new compiler's nightly version and is unstable so will keep changing the fix was removing the hidden importing of the step_trait that I wasnt even using

## Predictions I got wrong
I predicted E0046 would die on nightly. It didn't,
because the compiler's trait grew new required methods and the crate never grew with it. 

## Next
Migrating the bootloader to Limine from the bootloader crate
