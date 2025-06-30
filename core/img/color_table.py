from itertools import combinations,product
from collections.abc import Iterable

Color = tuple[int, int, int]

def is_good_color(c: tuple[int, int, int], threshold = 20) -> bool:
    """Determine if a color is suitable for masks."""
    
    channel_diffs = map(lambda x: abs(x[0]-x[1]), combinations(c, r=2))
    graylike = max(channel_diffs)<=threshold
    
    brightness = sum(c)
    bad_brightness = brightness<120 or brightness>645
    
    return not (graylike or bad_brightness)

color_table: list[Color] = []

def generate_color_table(levels_per_channel = 8) -> list[Color]:
    """Generate an ordered table of all valid colors under specific rules."""
    step1 = 255//(levels_per_channel-1)
    levels = [i*step1 for i in range(levels_per_channel)]
    all_colors = product(levels, repeat=3)
    gen_good_colors = filter(lambda x: is_good_color(x), all_colors)
    good_colors = list(gen_good_colors)
    
    gc_count = len(good_colors)
    step2 = 3   # A random prime.
    shuffled = [good_colors[(i*step2)%gc_count] for i in range(gc_count)]
    
    return shuffled

color_table = generate_color_table()

def preview_colors(colors: Iterable[Color], columns=8, block="    ") -> None:
    """Print the color to the terminal using ANSI escape sequences."""
    for i, (r, g, b) in enumerate(colors):
        print(f"\033[48;2;{r};{g};{b}m{block}\033[0m", end=" ")
        if (i + 1) % columns == 0:
            print()
    print()

if __name__=="__main__":
    preview_colors(color_table)