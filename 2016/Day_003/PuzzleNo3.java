import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class PuzzleNo3 {

    public static void main(String[] args) throws IOException {
        List<String> input = Files.readAllLines(Paths.get("src/puzzle_3.input"));

        int possible = 0;
        int lineCounter = 0;
        int triangleCounter = 0;

        int agregate = 0;
        String agregated = "";

        for(String line : input){
            lineCounter += 1;
            if ( agregate < 3){
                agregated += "  " + line;
                agregate += 1;
            }

            if (agregate == 3) {
                List<Integer> triangles = Arrays.asList(agregated.split("\\s+"))
                        .stream()
                            .filter(s -> !s.isEmpty())
                            .map(Integer::valueOf)
                            .collect(Collectors.toList());

                final int rowSize = 3;
                for(int i = 0; i < rowSize; i++) {
                    int finalI = i;
                    List<Integer> sides = IntStream.range(0, triangles.size())
                            .filter(v -> v % rowSize == finalI)
                            .mapToObj(triangles::get)
                            .collect(Collectors.toList());
                    System.out.println(sides);
                    if (isTriangle(sides))
                        possible += 1;
                    triangleCounter += 1;
                }

                agregate = 0;
                agregated = "";
            }
        }

        System.out.println(String.format("Possible = %d, Lines = %d, Triangles = %d",possible, lineCounter, triangleCounter));
    }

    private static boolean isTriangle(List<Integer> sides) {
        assert sides.size() == 3;
        return ( (sides.get(0) + sides.get(1)) >  sides.get(2)) &&
               ( (sides.get(1) + sides.get(2)) >  sides.get(0)) &&
               ( (sides.get(2) + sides.get(0)) >  sides.get(1));
    }
}
