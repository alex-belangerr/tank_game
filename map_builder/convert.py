import os
from PIL import Image
import math

def rgb_distance(pixel, target_colour):
    """Calculate the Euclidean distance between two RGB colours."""
    return math.sqrt(
        (pixel[0] - target_colour[0]) ** 2 +
        (pixel[1] - target_colour[1]) ** 2 +
        (pixel[2] - target_colour[2]) ** 2
    )

def is_within_colour_range(pixel, target_colour, buffer):
    """Check if the pixel's colour is within a certain distance (buffer) from the target colour."""
    return rgb_distance(pixel, target_colour) <= buffer

def bitmap_to_ron(image_path, buffer=10):
    # Open the image in RGB mode
    img = Image.open(image_path).convert("RGB")

    # Get image dimensions
    width, height = img.size

    # Initialize lists for walls and spawn points
    walls = []
    spawn_points = []

    # Target colours
    black = (0, 0, 0)  # Black for walls
    red = (255, 0, 0)  # Red for spawn points

    # Iterate over pixels and categorise them based on colour within a buffer range
    for y in range(height):
        for x in range(width):
            pixel_value = img.getpixel((x, y))

            if is_within_colour_range(pixel_value, black, buffer):  # Check if it's close to black
                walls.append((x, y))
            elif is_within_colour_range(pixel_value, red, buffer):  # Check if it's close to red
                spawn_points.append((x, y))

    # Format the lists for Rust's vec![] syntax
    walls_str = ', '.join([f"({x}, {y})" for x, y in walls])
    spawn_points_str = ', '.join([f"({x}, {y})" for x, y in spawn_points])

    # Generate Rust code for the Map struct in RON format
    map_ron = f"""(
    dim: ({width}, {height}),
    walls: [{walls_str}],
    spawn_points: [{spawn_points_str}]
)"""

    return map_ron

def process_images(input_dir, output_dir, buffer=10):
    # Ensure output directory exists
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    # Iterate over all files in the input directory
    for filename in os.listdir(input_dir):
        if filename.endswith(".bmp"):  # Process only .bmp files
            image_path = os.path.join(input_dir, filename)

            # Generate RON code from the bitmap
            ron_code = bitmap_to_ron(image_path, buffer)

            # Create corresponding output file path
            ron_filename = f"{os.path.splitext(filename)[0]}.ron"
            ron_path = os.path.join(output_dir, ron_filename)

            # Write the RON code to the output file
            with open(ron_path, "w") as ron_file:
                ron_file.write(ron_code)

            print(f"Processed {filename} -> {ron_filename}")

input_dir = "maps"
output_dir = "output"
if not os.path.exists(output_dir):
    os.makedirs(output_dir)

process_images(input_dir, output_dir, buffer=50)  # Adjust the buffer (distance) as needed
