import numpy as np
import cv2
from cv2.typing import MatLike

from ..shared.typing import Color

def remove_background(img: MatLike, mask: MatLike, color: Color)->MatLike:
    r,g,b = color
    mask_color = np.array((b,g,r))
    mask_indices = np.all(mask==mask_color, axis=2)
    
    dst = np.zeros_like(img)
    dst[:,:] = [0,255,0]
    dst[mask_indices] = img[mask_indices]
    
    return dst

