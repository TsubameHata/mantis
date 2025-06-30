import cv2
from cv2.typing import MatLike
import numpy as np
import scipy.signal as ss
from collections.abc import Iterable

def gen_sigmoid(t:float, k:float):
    def f(x: MatLike):
        return 1.0 / (1.0 + np.exp(k * (x - t)))
    return f

KERNEL_SIZE_RATIO = 35
THRESHOLD_PERCENTAGE = 50
SIGMOID_T = 0.2
SIGMOID_K = 25

sigmoid = gen_sigmoid(SIGMOID_T, SIGMOID_K)

def split_probability(img: MatLike)->MatLike:
    """Get the probability of each height being the position to split using Gaussian convolution."""
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    gray_reversed = 255 - gray
    
    white_threshold = np.percentile(gray_reversed, THRESHOLD_PERCENTAGE)
    
    gray_clipped = gray_reversed.copy()
    gray_clipped[gray_clipped <= white_threshold] = 0
    
    density = np.sum(gray_clipped, axis=1)
    
    height = gray.shape[0]
    kernel_size = max(3, height//KERNEL_SIZE_RATIO | 1)
    convolve_kernel = cv2.getGaussianKernel(kernel_size, sigma=kernel_size//4).flatten()
    density_smoothed = np.convolve(density, convolve_kernel, mode="same")
    density_normed = density_smoothed / np.max(density_smoothed)

    density_sig = sigmoid(density_normed)
    density_sig_normed = density_sig / np.max(density_sig)

    return density_sig_normed
    

MIN_PEAK_DISTANCE = 20

def find_peaks(probability: MatLike, 
               MIN_PEAK_PROMINENCE=0.03, 
               MIN_PEAK_DISTANCE=20)->Iterable[int]:
    """Find peaks from a density mat."""
    peaks, props = ss.find_peaks(probability, distance=MIN_PEAK_DISTANCE, 
                             prominence=MIN_PEAK_PROMINENCE,
                             width=(1,))
    lefts = props["left_ips"].astype(int) # type: ignore
    rights = props["right_ips"].astype(int) # type: ignore
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
    
    for y in peaks:
        if y>=(img_height-3): continue
        cv2.line(alpha_layer, (0, y), (img_width - 1, y), peak_color)
        cv2.line(alpha_layer, (0, y+1), (img_width - 1, y+1), peak_color)
        
    cv2.addWeighted(img, 1.0-alpha, alpha_layer, alpha, 0, dst)
    
    return dst

def test(input_dir: str, output_dir: str) -> None:
    """Test all the features above.

    Example:
    ```python
    import core.analysis.gaussian_conv as gc
    ps.test(...)
    ```
    """
    img = cv2.imread(input_dir)
    dst = paint_probabilities_and_peaks(img, alpha=0.5)
    cv2.imwrite(output_dir, dst)