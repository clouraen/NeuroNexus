#!/usr/bin/env python3
"""Generate project structure overview"""
import os
from pathlib import Path

def print_tree(directory, prefix="", max_depth=3, current_depth=0):
    """Print directory tree structure"""
    if current_depth >= max_depth:
        return
    
    try:
        entries = sorted(Path(directory).iterdir(), key=lambda x: (not x.is_dir(), x.name))
        for i, entry in enumerate(entries):
            is_last = i == len(entries) - 1
            current_prefix = "└── " if is_last else "├── "
            print(f"{prefix}{current_prefix}{entry.name}")
            
            if entry.is_dir() and current_depth < max_depth - 1:
                extension = "    " if is_last else "│   "
                print_tree(entry, prefix + extension, max_depth, current_depth + 1)
    except PermissionError:
        pass

if __name__ == "__main__":
    schema_dir = Path(__file__).parent / "schemas"
    print("\nJSON Schema Directory Structure:")
    print("=" * 60)
    print(f"\n{schema_dir.name}/")
    print_tree(schema_dir)
    print("\n" + "=" * 60)
    
    # Count files
    schema_files = list(schema_dir.rglob("*.schema.json"))
    md_files = list(schema_dir.rglob("*.md"))
    
    print(f"\n📊 Statistics:")
    print(f"   Schema files (.schema.json): {len(schema_files)}")
    print(f"   Documentation files (.md): {len(md_files)}")
    print(f"   Total files: {len(schema_files) + len(md_files)}")
    
    total_size = sum(f.stat().st_size for f in schema_dir.rglob("*") if f.is_file())
    print(f"   Total size: {total_size / 1024:.1f} KB")
    print()
