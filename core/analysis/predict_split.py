import cv2
import numpy as np
import scipy.signal as ss
from typing import Iterable

MatLike = cv2.typing.MatLike

def gen_sigmoid(t:float, k:float):
    def f(x: MatLike):
        return 1.0 / (1.0 + np.exp(k * (x - t)))
    return f

KERNEL_SIZE_RATIO = 35
SIGMOID_T = 0.2
SIGMOID_K = 30

sigmoid = gen_sigmoid(SIGMOID_T, SIGMOID_K)

def split_probability(img: MatLike)->MatLike:
    """Get the probability of each height being the position to split using Gaussian convolution."""
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    density = np.sum(255 - gray, axis=1)
    
    height = gray.shape[0]
    kernel_size = max(3, height//KERNEL_SIZE_RATIO | 1)
    convolve_kernel = cv2.getGaussianKernel(kernel_size, sigma=kernel_size//4).flatten()
    density_smoothed = np.convolve(density, convolve_kernel, mode="same")
    density_normed = density_smoothed / np.max(density_smoothed)

    density_sig = sigmoid(density_normed)
    density_sig_normed = density_sig / np.max(density_sig)

    return density_sig_normed
    

MIN_PEAK_DISTANCE = 20
MIN_PEAK_PROMINENCE = 0.05

def find_peaks(probability: MatLike)->Iterable[int]:
    """Find peaks from a density mat."""
    peaks, props = ss.find_peaks(probability, distance=MIN_PEAK_DISTANCE, 
                             prominence=MIN_PEAK_PROMINENCE,
                             width=1)
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
    
    dst = np.zeros_like(img, dtype=np.uint8)
    alpha_layer = np.zeros_like(img, dtype=np.uint8)
    
    img_height = img.shape[0]
    img_width = img.shape[1]
    
    scaled = (probability*img_width).astype(int)
    prob_line_length = np.clip(scaled, 0, img_width - 2)
    for y,l in enumerate(prob_line_length):
        cv2.line(alpha_layer, (0,y), (l,y), prob_color)
    
    peaks = np.clip(peaks, 0, img_height - 2).astype(int)
    for y in peaks:
        cv2.line(alpha_layer, (0, y), (img_width - 1, y), peak_color)
        
    cv2.addWeighted(img, 1.0-alpha, alpha_layer, alpha, 0, dst)
    
    return dst

def test(input_dir: str, output_dir: str) -> None:
    img = cv2.imread(input_dir)
    dst = paint_probabilities_and_peaks(img, alpha=0.5)
    cv2.imwrite(output_dir, dst)