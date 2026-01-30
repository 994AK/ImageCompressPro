from PIL import Image, ImageDraw, ImageFont
import os

os.makedirs('web_test/images', exist_ok=True)

def create_image(filename, size, color, text):
    img = Image.new('RGBA', size, color=color)
    d = ImageDraw.Draw(img)
    # Draw an X
    d.line([(0, 0), size], fill='white', width=5)
    d.line([(0, size[1]), (size[0], 0)], fill='white', width=5)
    # Draw a circle
    d.ellipse([size[0]//4, size[1]//4, size[0]*3//4, size[1]*3//4], outline='white', width=5)
    
    # Save
    img.save(f'web_test/images/{filename}')
    print(f"Created {filename}")

create_image('large.png', (800, 600), (255, 0, 0), "Large Red")
create_image('medium.png', (400, 400), (0, 128, 0), "Medium Green")
create_image('small.png', (100, 100), (0, 0, 255), "Small Blue")
