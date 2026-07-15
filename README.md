# gensisOS
a SMALL x86_64 kernel made with Rust from scratch in ~10 hours. 
There is no std or underlying things, just the barebones and me. 

## What it does:
- Boots on Bare metal (with the bootloader(custom) and QEMU) as a freestadning Rust binary
- Writes directly to VGA memoriy to draw the text on the screen
- built MY VERY OWN prinln! function!
- It also cathces interrupts like button presses and stuff
- Has real paging
- and also a working heap allocator

## To Run it:
cargo run

## The story
My god, I have never made an OS before, idk waht it even entails. I had some level of Rust understanding, but 
doign this project was a tough nut to crack. I followed Philip Opperman's Writing an OS in Rust guide. Even though it
was SUPPOSED TO work the way it went in the tutorial, I spent some time throguhout fixing typos or dependency conflicts.
I typed everythign painstakingly and badly and repeatedly until I got it right.

Genesis INDEED
