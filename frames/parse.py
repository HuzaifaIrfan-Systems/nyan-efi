import os
import glob

# Configuration
input_folder = "input"
output_folder = "output"
rows, cols = 25, 80
required_size = rows * cols

os.makedirs(output_folder, exist_ok=True)

for filepath in glob.glob(os.path.join(input_folder, "*.csv")):
    
    print(f"Processing: {filepath}")
    filename=os.path.splitext(os.path.basename(filepath))[0]
    print(filename)

    with open(filepath, "r", encoding="utf-8") as f:
        raw = f.read()
        # Split on comma and newline
        items = [x.strip() for x in raw.replace('\n', ',').split(',') if x.strip()]

    if len(items) < required_size:
        print(f"Skipping: not enough values ({len(items)} < {required_size})")
        continue

    items = items[:required_size]

    # Reshape into [[...], [...]]
    matrix = [items[i * cols:(i + 1) * cols] for i in range(rows)]
    
    

    # Convert to string like Python list: [[...], [...]]
    matrix_string=f"""
use uefi::proto::console::text::Color;
use uefi::proto::console::text::Color::*;

pub const NYAN_{filename}: [[Color; 80]; 25] = """
    matrix_string += "[\n" + "\n".join("  [" + ", ".join(row) + "]," for row in matrix) + "\n]"
    matrix_string +=";"
    # Save output
    out_file = os.path.join(output_folder, f"nyan_{filename}.rs")
    with open(out_file, "w", encoding="utf-8") as f:
        f.write(matrix_string)

    print(f"Saved: {out_file}")
