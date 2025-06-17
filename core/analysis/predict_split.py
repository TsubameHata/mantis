import cv2
import numpy as np
import scipy.signal as ss
from typing import Iterable

MatLike = cv2.typing.MatLike

KERNEL_SIZE_RATIO = 20

def split_probability(img: MatLike)->MatLike:
    """Get the probability of each height being the position to split using Gaussian convolution."""
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    density = np.sum(255 - gray, axis=1)
    
    height = gray.shape[0]
    kernel_size = max(3, height//KERNEL_SIZE_RATIO | 1)
    convolve_kernel = cv2.getGaussianKernel(kernel_size, sigma=kernel_size//4).flatten()
    density_smoothed = np.convolve(density, convolve_kernel, mode="same")
    density_normed = density_smoothed / np.max(density_smoothed)
    
    return density_normed

MIN_PEAK_DISTANCE = 20
MIN_PEAK_PROMINENCE = 10

def find_peaks(probability: MatLike)->Iterable[int]:
    """Find peaks from a density mat."""
    peaks, props = ss.find_peaks(probability, distance=MIN_PEAK_DISTANCE, 
                             prominence=MIN_PEAK_PROMINENCE)
    lefts = props["left_ips"].astype(int)
    rights = props["right_ips"].astype(int)
    return np.round((lefts+rights)/2)

def paint_probabilities_and_peaks(img: MatLike, 
                                  prob_color=(255,0,0), 
                                  peak_color=(0,0,255),
                                  alpha=1.0)->MatLike:
    """Paint probabilities and peaks to a new image for tests."""
    probability = split_probability(img)
    peaks = find_peaks(probability)
    
    dest = np.zeros_like(img, dtype=np.uint8)
    alpha_layer = np.zeros_like(img, dtype=np.uint8)
    
    img_width = img.shape[1]
    
    prob_line_length = probability*img_width
    for y,l in enumerate(prob_line_length):
        cv2.line(alpha_layer, (0,y), (l,y), prob_color)
    
    for y in peaks:
        cv2.line(alpha_layer, (0, y), (img_width, y), peak_color)
        
    cv2.addWeighted(img, 1.0, alpha_layer, alpha, 0, dest)
    
    return dest