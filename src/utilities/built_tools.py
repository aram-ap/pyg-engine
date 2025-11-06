from PIL import Image

def add_png_to_module(module_name, img_path):
    """
    Adds a PNG image to a Python module.

    Args:
        module_name (str): The name of the Python module.
        img_path (str): The path to the PNG image file.

    Returns:
        None
    """

    # Import the required modules
    import os

    # Check if the image file exists
    if not os.path.exists(img_path):
        raise FileNotFoundError(f"The image file {img_path} does not exist.")

    # Open the image file using Pillow
    img = Image.open(img_path)

    # Save the image to a bytes buffer
    buf = io.BytesIO()
    img.save(buf, format="PNG")
    buf.seek(0)

    # Add the image data to the Python module
    exec(f"import {module_name}; {module_name}.image_data = buf.read()")

# Example usage:

