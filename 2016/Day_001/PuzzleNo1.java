import java.util.*;
import java.util.stream.IntStream;

enum Side {
    LEFT,
    RIGHT
}

class Coord implements Comparable{
    public Coord(int x, int y){
        this.x = x;
        this.y = y;
    }

    public int x;
    public int y;

    @Override
    public boolean equals(Object c){
        Coord that = (Coord)c;
        boolean eq = that.x == this.x && that.y == this.y;
        if ( eq )
            System.out.println(String.format(" ## EQ  %s == %s", that.toString(), this.toString()));
        return eq;
    }
    @Override
    public String toString(){
        return String.format("Coord(X = %d, Y = %d)", x, y);
    }

    @Override
    public int compareTo(Object o) {
        if ( this.equals(o) )
            return 0;
        Coord that = (Coord)o;
        int thatVal = Math.abs(that.x) + Math.abs(that.y);
        int thisVal = Math.abs(this.x) + Math.abs(this.y);

        return thisVal - thatVal;
    }
}

public class PuzzleNo1 {

    private static final int NORTH = 0;
    private static final int EAST = 1;
    private static final int SOUTH = 2;
    private static final int WEST = 3;

    private static int x = 0;
    private static int y = 0;

    private static List<Coord> visited = new ArrayList<>();
    private static Coord duplicated = null;

    public static void main(String[] args) {
        String instructionString = "R5, R4, R2, L3, R1, R1, L4, L5, R3, L1, L1, R4, L2, R1, R4, R4, L2, L2, R4, L4, R1, R3, L3, L1, L2, R1, R5, L5, L1, L1, R3, R5, L1, R4, L5, R5, R1, L185, R4, L1, R51, R3, L2, R78, R1, L4, R188, R1, L5, R5, R2, R3, L5, R3, R4, L1, R2, R2, L4, L4, L5, R5, R4, L4, R2, L5, R2, L1, L4, R4, L4, R2, L3, L4, R2, L3, R3, R2, L2, L3, R4, R3, R1, L4, L2, L5, R4, R4, L1, R1, L5, L1, R3, R1, L2, R1, R1, R3, L4, L1, L3, R2, R4, R2, L2, R1, L5, R3, L3, R3, L1, R4, L3, L3, R4, L2, L1, L3, R2, R3, L2, L1, R4, L3, L5, L2, L4, R1, L4, L4, R3, R5, L4, L1, L1, R4, L2, R5, R1, R1, R2, R1, R5, L1, L3, L5, R2";
        List<String> instructions = Arrays.asList(instructionString.split(", "));

        int heading = -1;

        for (String i : instructions) {
            System.out.println(" # Instr = " + i);
            int val = Integer.valueOf(i.substring(1));
            Side side = instruction2Side(i);

            if ( heading == -1 ){
                if ( side == Side.RIGHT )
                    modX(val);
                else
                    modX(-val);
                heading = side == Side.RIGHT ? EAST : WEST;
            }
            else{
                heading = turn(heading, side);
                System.out.println("  H = " + heading);
                switch(heading){
                    case NORTH:
                        modY(-val);
                        break;
                    case SOUTH:
                        modY(val);
                        break;
                    case WEST:
                        modX(-val);
                        break;
                    case EAST:
                        modX(val);
                        break;
                }
            }
            if ( duplicated != null ) {
                System.out.println("Gotcha");
                x = duplicated.x;
                y = duplicated.y;
                break;
            }
        }

        System.out.println(String.format("X=%d, Y=%d, Abs X+Y=%d", x, y, Math.abs(x) + Math.abs(y)));

    }

    private static void modX(int val){
        IntStream.range(0, Math.abs(val)).forEach(v -> {
            Coord c = new Coord( val > 0 ? x + v : x - v, y);
            if ( isVisited(c) )
                duplicated = c;
            visited.add(c);
        });
        x += val;
    }

    private static void modY(int val){
        IntStream.range(0, Math.abs(val)).forEach(v -> {
            Coord c = new Coord(x, val > 0 ? y + v : y - v);
            if ( isVisited(c) )
                duplicated = c;
            visited.add(c);
        });
        y += val;
    }

    private static int turn(int heading, Side side){
        int r = ( side == Side.RIGHT ? heading + 1 : heading - 1 );

        if ( r < 0 )
            r = WEST;
        else if ( r > 3 )
            r = NORTH;

        return  r;
    }

    private static Side instruction2Side(String instruction){
        return instruction.charAt(0) == 'R' ? Side.RIGHT : Side.LEFT;
    }

    private static boolean isVisited(Coord c){
        return visited.stream().anyMatch( o -> o.equals(c));
    }
}
