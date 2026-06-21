import os
import glob

# Fix mojibake in all README files
for pattern in ['03-Collections/*/README.md', '04-FileIO/*/README.md', '04-FileIO/*/*/README.md']:
    for path in glob.glob(pattern):
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original = content
        # Fix common mojibake patterns
        content = content.replace('\u00e2\u20ac\u201d', '\u2014')  # â€" -> —
        content = content.replace('\u00e2\u20ac\u2019', '\u2019')  # â€™ -> '
        content = content.replace('\u00e2\u20ac\u201c', '\u201c')  # â€" -> "
        content = content.replace('\u00e2\u20ac\u2018', '\u2018')  # â€' -> '
        
        if content != original:
            with open(path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f'Fixed: {path}')
