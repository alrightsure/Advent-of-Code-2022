const INPUT_FILE = "./input.txt";

type Point = {
    row: number;
    column: number;
    elevation: string;
};

const main = async () => {
    const input = await Deno.readTextFile(INPUT_FILE);
    const lines = input.split("\n");
    const { grid, startingPoint, endPoint } = getGrid(lines);

    const answer1 = shortestPathBfs(grid, startingPoint, endPoint);
    const answer2 = shortestPathBfsAnyA(grid, endPoint);

    console.log(answer1);
    console.log(answer2);
};

const getGrid = (lines: string[]) => {
    const grid: Point[] = [];
    let startingPoint: Point = { row: 0, column: 0, elevation: "" };
    let endPoint: Point = { row: 0, column: 0, elevation: "" };

    for (let rowCount = 0; rowCount < lines.length; rowCount++) {
        for (let column = 0; column < lines[rowCount].length; column++) {
            let elevation = lines[rowCount][column];

            if (elevation == "S") {
                startingPoint = { row: rowCount, column, elevation };
                elevation = "a";
            }

            if (elevation == "E") {
                endPoint = { row: rowCount, column, elevation };
                elevation = "z";
            }

            grid.push({
                row: rowCount,
                column,
                elevation
            });
        }
    }

    return { grid, startingPoint, endPoint };
};

const getPoint = (row: number, column: number, grid: Point[]) => {
    return grid.find(point => point.row === row && point.column === column);
};

const getNeighbors = (grid: Point[], point: Point) => {
    const neighbors: Point[] = [];

    const up: Point = {
        row: point.row - 1,
        column: point.column,
        elevation: getPoint(point.row - 1, point.column, grid)?.elevation || ""
    };
    const down: Point = {
        row: point.row + 1,
        column: point.column,
        elevation: getPoint(point.row + 1, point.column, grid)?.elevation || ""
    };
    const left: Point = {
        row: point.row,
        column: point.column - 1,
        elevation: getPoint(point.row, point.column - 1, grid)?.elevation || ""
    };
    const right: Point = {
        row: point.row,
        column: point.column + 1,
        elevation: getPoint(point.row, point.column + 1, grid)?.elevation || ""
    };

    if (grid[up.row] && up.row >= 0 && up.column >= 0 && parseInt(up.elevation, 36) - 10 <= parseInt(point.elevation, 36) - 9) {
        neighbors.push(up);
    }

    if (grid[down.row] && down.row >= 0 && down.column >= 0 && parseInt(down.elevation, 36) - 10 <= parseInt(point.elevation, 36) - 9) {
        neighbors.push(down);
    }

    if (grid[left.row] && left.row >= 0 && left.column >= 0 && parseInt(left.elevation, 36) - 10 <= parseInt(point.elevation, 36) - 9) {
        neighbors.push(left);
    }

    if (grid[right.row] && right.row >= 0 && right.column >= 0 && parseInt(right.elevation, 36) - 10 <= parseInt(point.elevation, 36) - 9) {
        neighbors.push(right);
    }

    return neighbors;
};

const shortestPathBfs = (graph: Point[], startingPoint: Point, endPoint: Point) => {
    const previous = new Map();
    const visited: Point[] = [];
    const queue: { node: Point; dist: number }[] = [];

    queue.push({ node: startingPoint, dist: 0 });
    visited.push(startingPoint);

    while (queue.length > 0) {
        const { node, dist } = queue.shift()!;
        if (node.column === endPoint.column && node.row == endPoint.row) return dist;

        for (const neighbour of getNeighbors(graph, node)) {
            if (!visited.some(visitedNode => visitedNode.column === neighbour.column && visitedNode.row === neighbour.row)) {
                previous.set(neighbour, node);
                queue.push({ node: neighbour, dist: dist + 1 });
                visited.push(neighbour);
            }
        }
    }
    return -1;
};

const shortestPathBfsAnyA = (graph: Point[], endPoint: Point) => {
    const previous = new Map();
    const visited: Point[] = [];
    const queue: { node: Point; dist: number }[] = [];

    for (const point of graph) {
        if (point.elevation === "a") {
            queue.push({ node: point, dist: 0 });
            visited.push(point);
        }
    }

    while (queue.length > 0) {
        const { node, dist } = queue.shift()!;
        if (node.column === endPoint.column && node.row == endPoint.row) return dist;

        for (const neighbour of getNeighbors(graph, node)) {
            if (!visited.some(visitedNode => visitedNode.column === neighbour.column && visitedNode.row === neighbour.row)) {
                previous.set(neighbour, node);
                queue.push({ node: neighbour, dist: dist + 1 });
                visited.push(neighbour);
            }
        }
    }
    return -1;
}

main();
