use super::{Bearing, Point, Vector};

pub type Shape = Vec<Point>;

pub trait Lattice<B: Bearing> {
    // @returns the bearing at p that corresponds to the given direction
    fn bearing(self, p: Point, v: Vector) -> B;

    // @returns all possible directions of straight lines in the lattice
    fn bearings(self) -> Vec<B>;

    // @returns the two cells with the edge with the given midpoint
    fn cells_with_edge(self, p: Point): (Point, Point);

    // @returns the cells with the given vertex point
    fn cells_with_vertex(self, p: Point): Vec<Point>;

    // @returns the dual lattice, i.e. the lattice of the vertices
    fn dual(self): Lattice;

    // @returns all directions to cells that share an edge with p
    fn edge_sharing_directions(self, p: Point): Vec<Vector>;

    // @returns all [q, v] for q that shares an edge with p
    fn edge_sharing_neighbors(self, p: Point): Vec<(Point, Vector)>;

    // @returns all cells that share an edge with p
    fn edge_sharing_points(self, p: Point): Vec<Point>;

    // @returns the two vertices of the edge between cells p1 to p2
    fn edge_vertices(self, p1: Point, p2: Point): (Point, Point);

    /** @returns whether this translation maps the lattice to itself */
    inBasis(v: Vector): boolean;

    /** @returns all transformations other than translations that map the lattice to itself */
    pointGroup(): ((p: Point) => Point)[];

    /** @returns all polyominoes with the given number of edge-connected cells */
    polyominoes(size: number, includeRotationsAndReflections?: boolean): Shape[];

    /**
     * @returns a representative for each distinct cell type in the lattice
     * e.g. square and hexagonal grids only have one type, but triangular grids have two, the
     * upward-pointing triangle, and the downward-pointing triangle.
     */
    representativeCells(): Point[];

    /** @returns all directions to cells that share a vertex with p */
    vertexSharingDirections(p: Point): Vector[];

    /** @returns all [q, v] for q that shares a vertex with p */
    vertexSharingNeighbors(p: Point): [Point, Vector][];

    /** @returns all cells that share a vertex with p */
    vertexSharingPoints(p: Point): Point[];

    /** @returns the coordinates of all vertices of p */
    vertices(p: Point): Point[];
}