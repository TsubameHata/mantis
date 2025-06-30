from collections.abc import Iterable
from typing import IO

def save_imgs(imgs: Iterable[IO], path: str)-> None:
    imgs_and_paths = ((img,f"{path}_{index}.png") for index,img in enumerate(imgs))
    for i,p in imgs_and_paths:
        with open(p, mode="wb") as f:
            f.write(i.read())