import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;
import java.util.Queue;

class Point
{
    Point(int x, int y)
    {
        this.x = x;
        this.y = y;
    }

    @Override
    public boolean equals(Object o)
    {
        Point that = (Point) o;
        return that.x == x && that.y == y;
    }

    @Override
    public int hashCode()
    {
        return Objects.hash(x, y);
    }

    int x;
    int y;
}

class GridGenerator
{
    private static Character WALL = '#';
    private static Character FLOOR = '.';

    private int magicNumber;
    private int width;
    private int height;
    private Character[] tiles;

    GridGenerator(int magicNumber, int width, int height)
    {
        this.magicNumber = magicNumber;
        this.width = width;
        this.height = height;
        tiles = new Character[width * height];

        generate();
    }

    int locationCountWithingDistance(Point start, int distance)
    {
        int c = 0;

        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                if (shortestPath(start, new Point(x, y)) <= 50) {
                    c++;
                }
            }
        }

        return c;
    }

    int shortestPath(Point start, Point goal)
    {

        Queue<Point> frontier = new ArrayDeque<>();
        frontier.add(start);

        Map<Point, Point> cameFrom = new LinkedHashMap<>();
        cameFrom.put(start, start);

        while (frontier.size() > 0) {
            Point current = frontier.poll();
            List<Point> nbs = getNeighbours(current);
            for (Point next : nbs) {
                if (!cameFrom.containsKey(next)) {
                    frontier.add(next);
                    cameFrom.put(next, current);
                }
            }
        }

        Point current = goal;
        int size = 0;
        while (!current.equals(start)) {
            current = cameFrom.get(current);
            size++;
            if (current == null) {
                size = Integer.MAX_VALUE;
                break;
            }
        }
        return size;
    }

    private List<Point> getNeighbours(Point p)
    {
        List<Point> nb = new ArrayList<>();

        if (isValid(p.x - 1, p.y) && getChar(p.x - 1, p.y) == FLOOR) {
            nb.add(new Point(p.x - 1, p.y));
        }

        if (isValid(p.x, p.y + 1) && getChar(p.x, p.y + 1) == FLOOR) {
            nb.add(new Point(p.x, p.y + 1));
        }

        if (isValid(p.x + 1, p.y) && getChar(p.x + 1, p.y) == FLOOR) {
            nb.add(new Point(p.x + 1, p.y));
        }

        if (isValid(p.x, p.y - 1) && getChar(p.x, p.y - 1) == FLOOR) {
            nb.add(new Point(p.x, p.y - 1));
        }

        return nb;
    }

    Character getChar(int x, int y)
    {
        int idx = y * width + x;
        return tiles[idx];
    }

    private boolean isValid(int x, int y)
    {
        int idx = y * width + x;
        return idx >= 0 && idx < tiles.length;
    }

    @Override
    public String toString()
    {
        String s = "";
        int i = 0;
        for (Character c : tiles) {
            s += c;
            i++;
            if (i % width == 0) {
                s += "\n";
            }
        }
        return s;
    }

    private void generate()
    {
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                tiles[width * y + x] = calculate(x, y);
            }
        }
    }

    private Character calculate(int x, int y)
    {
        Integer num = x * x + 3 * x + 2 * x * y + y + y * y + magicNumber;
        return Integer.bitCount(num) % 2 == 0 ? FLOOR : WALL;
    }
}

public class PuzzleNo13
{
    static public void main(String[] args)
    {
        GridGenerator grid = new GridGenerator(1364, 100, 100);
        System.out.println(grid);
        System.out.println("Shortest path to (31,39): "
                + grid.shortestPath(new Point(1, 1), new Point(31, 39)));
        System.out.println("Location count could be reached in at most 50 steps: "
                + grid.locationCountWithingDistance(new Point(1, 1), 50));
    }
}
