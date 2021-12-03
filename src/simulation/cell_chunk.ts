import Cell from "./cell";

/**
 * Chunk containing an array of Cells, For use in simulation the Falling Sands Cellular Automata.
 * 
 * ## Fields
 * ```ts
 * size: [number, number] // Size of the Chunk, the Chunk's Cells will corespond to its size.
 * cells: Array<Cell> // Array containing the Cells of the Chunk.
 * ```
 */
class CellChunk {
    size: [number, number];
    cells: Array<Cell>;

    constructor(size: [number, number]) {
        this.cells = [  ];
        this.size = size;

        for (let x: number = 0; x < size[0]; x += 1) {
            for (let y: number = 0; y < size[1]; y += 1) {
                let new_cell: Cell = new Cell([x, y], 0);
                this.cells.push(new_cell);
            }
        }
    }

    /**
     * Returns the `index` of the `Cell` at the specified position within the chunk.
     * 
     * ## Parameters
     * ```ts
     * position: [number, number] // Position of the Cell relative to the Chunk.
     * ```
     */
    get_index(position: [number, number]): number {
        /* TODO: Complete this function. */
        return 0;
    }
}

export default CellChunk;