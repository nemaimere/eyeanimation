#!/usr/bin/env python3
"""
Split a sprite sheet into individual sprites based on transparent pixel bands.
"""

import argparse
from PIL import Image
import numpy as np
import os


def find_sprite_rows(image_array):
    """
    Find the rows that contain sprites by identifying transparent bands.
    Returns a list of (start_row, end_row) tuples for each sprite.
    """
    height = image_array.shape[0]

    # Check if each row is fully transparent
    # A row is transparent if all pixels have alpha = 0
    if image_array.shape[2] == 4:  # Has alpha channel
        row_has_content = np.any(image_array[:, :, 3] > 0, axis=1)
    else:
        # No alpha channel, assume all rows have content
        row_has_content = np.ones(height, dtype=bool)

    sprites = []
    in_sprite = False
    start_row = 0

    for i, has_content in enumerate(row_has_content):
        if has_content and not in_sprite:
            # Start of a new sprite
            start_row = i
            in_sprite = True
        elif not has_content and in_sprite:
            # End of current sprite
            sprites.append((start_row, i))
            in_sprite = False

    # Handle case where last sprite extends to end of image
    if in_sprite:
        sprites.append((start_row, height))

    return sprites


def split_sprites(input_path, output_name, output_dir=None):
    """
    Split a sprite sheet into individual sprites.

    Args:
        input_path: Path to the input sprite sheet
        output_name: Base name for output files (without extension)
        output_dir: Directory to save output files (default: same as input)
    """
    # Load the image
    img = Image.open(input_path)
    img_array = np.array(img)

    print(f"Image size: {img.size} ({img_array.shape})")
    print(f"Mode: {img.mode}")

    # Find sprite boundaries
    sprite_rows = find_sprite_rows(img_array)
    print(f"Found {len(sprite_rows)} sprites")

    # Determine output directory
    if output_dir is None:
        output_dir = os.path.dirname(input_path) or '.'

    # Create output directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)

    # Extract and save each sprite
    for i, (start_row, end_row) in enumerate(sprite_rows, 1):
        sprite_height = end_row - start_row
        print(f"Sprite {i}: rows {start_row}-{end_row} (height: {sprite_height})")

        # Extract the sprite
        sprite_array = img_array[start_row:end_row, :, :]
        sprite_img = Image.fromarray(sprite_array)

        # Save with zero-padded number
        output_path = os.path.join(output_dir, f"{output_name}{i:02d}.png")
        sprite_img.save(output_path)
        print(f"  Saved: {output_path}")

    print(f"\nSuccessfully split {len(sprite_rows)} sprites")


def main():
    parser = argparse.ArgumentParser(
        description="Split a sprite sheet into individual sprites based on transparent bands"
    )
    parser.add_argument(
        "input",
        help="Input sprite sheet image file"
    )
    parser.add_argument(
        "-o", "--output",
        default="sprite",
        help="Base name for output files (default: sprite)"
    )
    parser.add_argument(
        "-d", "--dir",
        default=None,
        help="Output directory (default: same as input file)"
    )

    args = parser.parse_args()

    if not os.path.exists(args.input):
        print(f"Error: Input file '{args.input}' not found")
        return 1

    split_sprites(args.input, args.output, args.dir)
    return 0


if __name__ == "__main__":
    exit(main())
