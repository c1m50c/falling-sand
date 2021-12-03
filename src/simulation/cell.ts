/**
 * Cell Class for use when simulating the Falling Sand Cellular Automata.
 * ## Fields
 * ```ts
 * position: [number, number] // Position of the Cell within the Chunk.
 * material_id: number // MaterialId of the Cell, 0 for Air.
 * ```
 */
class Cell {
    position: [number, number];
    material_id: number;

    constructor(position: [number, number], material_id: number = 0) {
        this.material_id = material_id;
        this.position = position;
    }
}

export default Cell;