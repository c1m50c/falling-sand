from typing import Tuple, Set
import pygame

BACKGROUND_CLR: Tuple[int, int, int] = (16, 16, 24)
CELL_CLR: Tuple[int, int, int] = (153, 121, 178)
GRID_CLR: Tuple[int, int, int] = (39, 35, 43)

SCREEN = pygame.display.set_mode((800, 512), pygame.RESIZABLE)
WIDTH, HEIGHT = SCREEN.get_size()
CELL_SIZE: int = 32

cells: Set[Tuple[int, int]] = set()


def draw_grid() -> None:
    """
        Draws a grid for each cell within the boundaries of the inital `SCREEN` size.
    """
    
    for x in range(0, WIDTH, CELL_SIZE):
        for y in range(0, HEIGHT, CELL_SIZE):
            cell = pygame.Rect(x, y, CELL_SIZE, CELL_SIZE)
            pygame.draw.rect(SCREEN, GRID_CLR, cell, 1)


def draw_cells(cells: Set[Tuple[int, int]]) -> None:
    """
        Draws all cells within the `cells` set.
    """
    
    for x, y in cells:
        cell = pygame.Rect(x, y, CELL_SIZE, CELL_SIZE)
        SCREEN.fill(CELL_CLR, cell)


def main() -> None:
    """
        Application entry-point.
    """
    
    global cells
    
    pygame.display.set_caption("Falling Sand")
    
    while True:
        SCREEN.fill(BACKGROUND_CLR)
        
        draw_cells(cells)
        draw_grid()

        event = pygame.event.poll()
        
        if event.type == pygame.QUIT:
            return
        
        if event.type == pygame.KEYDOWN:
            if event.key == pygame.K_ESCAPE:
                return
        
        pygame.display.flip()


if __name__ == "__main__":
    main()