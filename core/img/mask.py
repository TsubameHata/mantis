import numpy as np
import cv2
from cv2.typing import MatLike
from numpy.typing import NDArray

from ..shared.typing import Color

BACKGROUND_COLOR: NDArray[3] = np.array([0,255,0])

def get_mask_indicies(mask: MatLike, color: Color) -> MatLike:
    r,g,b = color
    mask_color = np.array((b,g,r))
    mask_indices = np.all(mask==mask_color, axis=2)
    return mask_indices

def remove_background(img: MatLike, 
                      mask: MatLike, 
                      color: Color, 
                      background_color = BACKGROUND_COLOR)->MatLike:
    mask_indices = get_mask_indicies(mask, color)
    dst = np.zeros_like(img)
    dst[:,:] = background_color
    dst[mask_indices] = img[mask_indices]
    return dst

def get_border_height(img: MatLike, 
                      background_color = BACKGROUND_COLOR) -> tuple[int, int]:
    background_indices = np.all(img==background_color, axis=2)
    mask_height = np.all(background_indices==False, axis=1)
    y_indices = np.where(mask_height)[0]
    return int(y_indices[0]), int(y_indices[-1]+1)

def resize_and_paste(img: MatLike,
                     inner_height: tuple[int, int],
                     border_height: tuple[int, int],
                     output_img_size: tuple[int, int],
                     output_inner_height: int,
                     output_background_color: Color = (255,255,255),
                     shrink_y_overflow = False,
                     background_color = BACKGROUND_COLOR)->MatLike:
    """Resize the image and paste it into a new image of specified size.
    Though the output_inner_height is specified in int, it is greatly recommended to calculate it based on the aspect ratio of the area."""
    # Compute resizing coefficient
    origin_inner_height = inner_height[1]-inner_height[0]
    origin_height = border_height[1]-border_height[0]
    out_height = output_img_size[0]
    k = output_inner_height / origin_inner_height

    center_direction = (inner_height[0] + inner_height[1]) // 2
    upper = center_direction - border_height[0]
    lower = border_height[1] - center_direction
    max_height_from_center = max(upper, lower)

    y_overflow_to_be_handled = False

    if out_height <= 2*k*max_height_from_center:
        if shrink_y_overflow:
            k = out_height / (origin_height + 2)
        else:
            y_overflow_to_be_handled = True

    img[get_mask_indicies(img, background_color)] = output_background_color
    img_cropped = img[border_height[0]:border_height[1], :]

    if y_overflow_to_be_handled:
        delta_height = int(((2*k*max_height_from_center - out_height)/k) // 2)
        img_cropped = img_cropped[delta_height:img_cropped.shape[0]-delta_height, :]
        upper = img_cropped.shape[0]//2

    # Hide the overflowed width regardless of flag shrink_y_overflow
    if output_img_size[1] <= k*img_cropped.shape[1]:
        delta_width = int(((k*img_cropped.shape[1] - output_img_size[1])/k + 4)//2)
        img_cropped = img_cropped[:, delta_width:img_cropped.shape[1]-delta_width]

    resized = cv2.resize(img_cropped, dsize=(int(k*img_cropped.shape[1]), int(k*img_cropped.shape[0])), interpolation=cv2.INTER_LINEAR)

    out = np.zeros((output_img_size[0], output_img_size[1], 3), dtype=np.uint8)
    out[:,:] = output_background_color

    # Center the image if possible
    begin_height = max(int(out_height//2 - k*upper),0)
    begin_width = max(int(output_img_size[1]//2 - resized.shape[1]//2), 0)

    out[begin_height:begin_height+resized.shape[0], begin_width:begin_width+resized.shape[1]] = resized
    
    return out

def test(img: MatLike, mask:MatLike, color: Color, 
         inner_height:tuple[int, int], output_img_size = (1920,1080), output_inner_height = 864, shrink_y_overflow = False)->MatLike:
    rb = remove_background(img, mask, color)
    border_height = get_border_height(rb)
    out = resize_and_paste(rb, inner_height, border_height, output_img_size, output_inner_height, shrink_y_overflow=shrink_y_overflow)
    return out