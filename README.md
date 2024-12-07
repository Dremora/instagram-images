# instagram-images

Prepare images to fit Instagram's requirements for the posts.

It will resize the images to the correct aspect ratio, and pad the image with a color to fill the rest of the space. Images will be saved as JPEG with 100% quality.

## Usage

```
instagram-images --aspect-ratio square --target ./out --source ./source-images --pad-color=black
```

## Building

Install `rustup`, then run `cargo build --release`.
