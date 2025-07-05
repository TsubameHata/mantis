import numpy as np
import cv2
from cv2.typing import MatLike

def crop_horizontal(img: MatLike, begin: int, end: int)->MatLike:
    return img[:,begin:end+1]

def crop_vertical(img: MatLike, begin: int, end: int)->MatLike:
    # NOTICE: Be aware of coordinate transformation when cropping vertically!!
    # When used, crop the mask img simultaneously and re-calculate the dividing line.
    return img[begin:end+1, :]

def concat_vertical(img1: MatLike, img2: MatLike)->MatLike:
    height = img1.shape[0] + img2.shape[0]
    width = max(img1.shape[1], img2.shape[1])
    dst = np.ones((height,width,3), dtype=np.uint8)*255
    delta_width = abs(img1.shape[1]-img2.shape[1])//2
    if img1.shape[1]>=img2.shape[1]:
        dst[:img1.shape[0], :img1.shape(1)] = img1
        dst[img1.shape[0]:, delta_width:delta_width+img2.shape[1]] = img2
    else:
        dst[:img1.shape[0], delta_width:delta_width+img1.shape(1)] = img1
        dst[img1.shape[0]:, :img2.shape[1]] = img2
    return dst