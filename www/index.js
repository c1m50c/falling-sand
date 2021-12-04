import { CellChunk, Vector2U } from "wasm-falling-sand";

const new_chunk = CellChunk.new(Vector2U.new(16, 16));
alert(new_chunk.cell_count());