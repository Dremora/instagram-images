# instagram-images

Prepare images to fit Instagram's requirements for the posts.

It will resize the images to the correct aspect ratio and resolution, and pad them with a color to fill the rest of the space. Images will be saved as JPEG with 100% quality.

## Usage

```
instagram-images --aspect-ratio square --target ./out --source ./source-images --pad-color=black
```

Run `instagram-images --help` to see all the options.

## Building

Install `rustup`, then run `cargo build --release`. The binary will be located in `target/release/instagram-images`.
