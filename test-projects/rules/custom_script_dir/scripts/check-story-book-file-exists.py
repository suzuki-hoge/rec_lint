#!/usr/bin/env python
import os
import sys


def main() -> int:
    if len(sys.argv) < 2:
        print("Usage: check-story-book-file-exists.py <file_path>")
        return 1

    file_path = sys.argv[1]

    if file_path.endswith(".stories.tsx"):
        return 0

    if file_path.endswith(".tsx"):
        story_path = file_path[:-4] + ".stories.tsx"
        if os.path.isfile(story_path):
            return 0
        print(f"not found: {os.path.basename(story_path)}")
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
