const PORT = 55782;
const HEADERS_BOARD = {
  headers: {
    "Content-Type": "text/html",
    "Cache-Control": "no-store",
  },
};
const HEADERS_STYLESHEET = {
  headers: {
    "Content-Type": "text/css",
  },
};

async function htmlDoc(body: string, head: string): Promise<string> {
  return `<!DOCTYPE html><html><head><link rel=stylesheet href=style.css>${head}</head><body>${body}</body></html>`
}

async function createBoard(width: number, height: number, mines: number): Promise<string> {
  console.debug("width", width, "height", height, "mines", mines);
  if (width <= 0 || height <= 0 || mines <= 0) {
    return await htmlDoc("NO 0 VALUES ALLOWED", "");
  }
  if (width > 50 || height > 50) {
    return await htmlDoc("NO FIELDS LARGER THAN 50 IN EITHER DIMENSION", "");
  }
  if (mines >= width * height) {
    return await htmlDoc("TOO MANY MINES", "");
  }

  const board = new Array(width * height).fill(false);
  let safeIdx = board.map((_, idx) => idx);

  for (let placedMines = 0; placedMines < mines; placedMines++) {
    const mineIdx = safeIdx.splice(Math.floor(Math.random() * safeIdx.length), 1)[0];

    board[mineIdx] = true;
  }

  function getNeighbors(idx: number): number {
    let count = 0;
    const leftEdge = (idx % width) === 0;
    const topEdge = idx < width;
    const bottomEdge = idx > (height * (width - 1));
    const rightEdge = (idx % width) === (width - 1);

    if (!leftEdge && board[idx - 1]) {
      count++;
    }
    if (!leftEdge && !topEdge && board[idx - 1 - width]) {
      count++;
    }
    if (!leftEdge && !bottomEdge && board[idx - 1 + width]) {
      count++;
    }

    if (!rightEdge && board[idx + 1]) {
      count++;
    }
    if (!rightEdge && !topEdge && board[idx + 1 - width]) {
      count++;
    }
    if (!rightEdge && !bottomEdge && board[idx + 1 + width]) {
      count++;
    }

    if (!topEdge && board[idx - width]) {
      count++;
    }

    if (!bottomEdge && board[idx + width]) {
      count++;
    }
    return count;
  }

  let table = "<table><tbody><tr>";
  let inputs = "";
  let style = "<style>";
  for (const [idx, fieldIsMine] of board.entries()) {
    table += `<td><label for=input_${idx}></label></td>`;

    if ((idx % width) === (width - 1)) {
      table += "</tr><tr>"
    }
    inputs += `<input id=input_${idx} type=checkbox ${fieldIsMine ? "data-mine" : "data-safe"}></input>`;
    style += `#input_${idx}:checked ~ main label[for="input_${idx}"]::before { content: "${fieldIsMine ? "X" : getNeighbors(idx)}"; }\n`
           + `#input_${idx}:checked ~ main label[for="input_${idx}"] { pointer-events: none; }`;
  }
  table += "</tr></tbody></table>"
  style += "</style>"

  return await htmlDoc(`${inputs}<main>${table}</main><footer><span id=lost>HAHA you lost >:)</span><span id=won>YAY, you WON!!!</span></footer>`, style)
}

Bun.serve({
  PORT,
  async fetch(request) {
    const url = new URL(request.url);

    if (url.pathname === "/favicon.ico") {
      // TODO: Add favicon
      return new Response("Not found", { status: 404, statusText: "Not found" });
    }

    if (url.pathname === "/style.css") {
      return new Response(await Bun.file("style.css").text(), HEADERS_STYLESHEET);
    }

    if (url.pathname !== "/") {
      return new Response("Not found", { status: 404, statusText: "Not found" });
    }

    let height = Number.parseInt(url.searchParams.get("height") ?? "NaN");
    if (Number.isNaN(height)) {
      height = 10;
    }
    let width = Number.parseInt(url.searchParams.get("width") ?? "NaN");
    if (Number.isNaN(width)) {
      width = 10;
    }
    let mines = Number.parseInt(url.searchParams.get("mines") ?? "NaN");
    if (Number.isNaN(mines)) {
      mines = 10;
    }

    return new Response(await createBoard(width, height, mines), HEADERS_BOARD);
  }
})