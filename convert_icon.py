from PIL import Image

img_path = 'app-icon.png'
ico_path = 'app-icon.ico'

# Load the PNG image
with Image.open(img_path) as img:
    # Define the target sizes for Windows icons (16, 32, 48, 64, 128, 256)
    sizes = [(16, 16), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    
    # Create a list of resized images
    images = []
    for size in sizes:
        resized_img = img.copy()
        resized_img.thumbnail(size, Image.Resampling.LANCZOS)
        images.append(resized_img)
    
    # Save as ICO file using the first image as the main one
    images[0].save(ico_path, format='ICO', sizes=[img.size] + sizes)

print(f"Converted {img_path} to {ico_path}")
