import numpy as np
import cv2
from cv2.typing import MatLike

from ..shared.typing import Color

BACKGROUND_COLOR = np.array([0,255,0])


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
    dst = np.broadcast_to(background_color, img.shape)
    dst[mask_indices] = img[mask_indices]
    return dst

def get_border_height(img: MatLike, 
                      background_color = BACKGROUND_COLOR) -> tuple[int, int]:
    background_indices = np.all(img==background_color, axis=2)
    mask_height = np.all(background_indices==False, axis=1)
    y_indices = np.where(mask_height)[0]
    return y_indices[0], y_indices[-1]+1