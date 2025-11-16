# Images Directory

This directory contains input and output images for the visual processing system.

## Structure

```
images/
├── input/          # Test images and user-provided images
│   ├── test_vertical.png
│   ├── test_horizontal.png
│   ├── test_checkerboard.png
│   ├── test_diagonal.png
│   ├── test_face.png
│   └── test_texture.png
│
└── output/         # Generated results (edge maps, etc.)
    └── edges.png   # Output from visual processing
```

## Usage

### Generate test images:
```bash
cargo run --example create_test_images --release
```

### Process an image:
```bash
cargo run --example process_image --release -- images/input/test_face.png
```

### Process your own image:
```bash
# Copy your image to images/input/
cp myimage.jpg images/input/

# Process it
cargo run --example process_image --release -- images/input/myimage.jpg
```

## Output

The processing results are saved to `images/output/`:
- `edges.png` - Edge detection visualization from ganglion cells

## Note

- Input images are tracked in git
- Output images are ignored by git (regenerated on each run)
