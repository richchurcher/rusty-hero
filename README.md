# Rusty Hero

Handmade Hero in Rust with SDL2 serving as the platform layer. Do I really want to write much X11 code? Well... maybe later. Platform is Arch Linux, and we'll see how portable it ends up.

Each episode is tagged as a release with a three digit number. To check out the code at a particular tag on a new branch, use:

```shell
git checkout -b episode-one 001
```


## Caveat

As far as rusting goes, I'm only just beginning to corrode.


## Resources

 - [Handmade Hero](https://handmadehero.org)
 - [The SDL Port on GitHub](https://github.com/HandmadeHero/sdl) if you pre-ordered (which you <a href="https://transactions.sendowl.com/packages/6671/8CB9DE0F/purchase?gateway=Stripe">should</a>)
 - [Handmade Penguin](https://davidgow.net/handmadepenguin/default.html)
 - [rust-sdl2](https://github.com/AngryLawyer/rust-sdl2)
 - [lldb.nvim](https://github.com/critiqjo/lldb.nvim) - Neovim LLDB integration


## Notes

### 003

It's been interesting learning about Rust's approach to allocating on the heap. If you don't know the size of the buffer you want, the alternatives seem to be either:

```rust
    let bytes = width * height * 4;
    let pixels = vec![0; bytes as usize];
    
    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::ARGB8888,
        width, height).unwrap();
    texture.update(None, &pixels[..], pitch as usize).unwrap();
```

or, for a buffer whose size definitely isn't going to change:

```rust
    let pixels = vec![0; bytes as usize].into_boxed_slice();
```

Either works. `Vec` always uses the heap, so the only real difference seems to be the inability to resize. There's no need to manually free buffers created in this way. The way I've done it does seem slow... there's a slightly noticeable lag between when the window is resized and when it redraws. Looking forward to finding out more about this over time, or if anyone's got any... _pointers_.
 
