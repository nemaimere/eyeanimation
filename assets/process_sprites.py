#!/usr/bin/env python3
"""
Process sprite images by splitting them left/right, resizing, and cropping.
For each input image, creates left (L) and right (R) variants at 480x800.
"""

import argparse
from PIL import Image
import os
import glob


def process_image(input_path, output_dir):
    """
    Process a single image:
    1. Split into left and right halves (each 1920px wide)
    2. Resize each half to height=800px (maintaining aspect ratio)
    3. Crop symmetrically from left/right edges to width=480px

    Args:
        input_path: Path to input image
        output_dir: Directory to save output files

    Returns:
        Tuple of (left_output_path, right_output_path)
    """
    # Load image
    img = Image.open(input_path)
    width, height = img.size

    # Get base filename without extension
    basename = os.path.splitext(os.path.basename(input_path))[0]

    print(f"Processing {basename}: {width}x{height}")

    # Split into left and right halves
    mid_x = width // 2
    left_half = img.crop((0, 0, mid_x, height))
    right_half = img.crop((mid_x, 0, width, height))

    print(f"  Split into halves: {left_half.size[0]}x{left_half.size[1]} each")

    # Process left half
    left_processed = resize_and_crop(left_half, target_height=800, target_width=480)
    left_output = os.path.join(output_dir, f"{basename}L.png")
    left_processed.save(left_output)
    print(f"  Saved left: {left_output} ({left_processed.size[0]}x{left_processed.size[1]})")

    # Process right half
    right_processed = resize_and_crop(right_half, target_height=800, target_width=480)
    right_output = os.path.join(output_dir, f"{basename}R.png")
    right_processed.save(right_output)
    print(f"  Saved right: {right_output} ({right_processed.size[0]}x{right_processed.size[1]})")

    return left_output, right_output


def resize_and_crop(img, target_height, target_width):
    """
    Resize image to target height, then crop symmetrically to target width.

    Args:
        img: PIL Image
        target_height: Desired height in pixels
        target_width: Desired width in pixels

    Returns:
        Processed PIL Image
    """
    # Resize to target height while maintaining aspect ratio
    current_width, current_height = img.size
    scale_factor = target_height / current_height
    new_width = int(current_width * scale_factor)

    resized = img.resize((new_width, target_height), Image.LANCZOS)

    # Crop symmetrically from left and right to reach target width
    if new_width > target_width:
        # Calculate how much to crop from each side
        crop_total = new_width - target_width
        crop_left = crop_total // 2
        crop_right = crop_left + target_width

        cropped = resized.crop((crop_left, 0, crop_right, target_height))
        return cropped
    else:
        # Image is already narrower than target, return as-is
        # (or could pad, but spec assumes it will be wider)
        return resized


def process_directory(input_dir, output_dir):
    """
    Process all PNG files in the input directory.

    Args:
        input_dir: Directory containing input PNG files
        output_dir: Directory to save output files
    """
    # Create output directory if it doesn't exist
    os.makedirs(output_dir, exist_ok=True)

    # Find all PNG files in input directory
    pattern = os.path.join(input_dir, "*.png")
    png_files = sorted(glob.glob(pattern))

    if not png_files:
        print(f"No PNG files found in {input_dir}")
        return

    print(f"Found {len(png_files)} PNG files to process\n")

    # Process each file
    for i, png_file in enumerate(png_files, 1):
        print(f"[{i}/{len(png_files)}]")
        process_image(png_file, output_dir)
        print()

    print(f"Successfully processed {len(png_files)} images")
    print(f"Created {len(png_files) * 2} output files in {output_dir}")


def main():
    parser = argparse.ArgumentParser(
        description="Split, resize, and crop sprite images. "
                    "Each input creates left (L) and right (R) outputs at 480x800."
    )
    parser.add_argument(
        "input_dir",
        help="Input directory containing PNG files"
    )
    parser.add_argument(
        "-o", "--output",
        required=True,
        help="Output directory for processed images"
    )

    args = parser.parse_args()

    if not os.path.isdir(args.input_dir):
        print(f"Error: Input directory '{args.input_dir}' not found")
        return 1

    process_directory(args.input_dir, args.output)
    return 0


if __name__ == "__main__":
    exit(main())
