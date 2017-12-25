import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class PuzzleNo6 {
    public static void main(String[] args) throws IOException {
        List<String> lines = Files.readAllLines(Paths.get("src/puzzle_6.input"));

        final int rowSize = lines.get(0).length();

        String[] cols = new String[rowSize];
        IntStream.range(0, rowSize).forEach(i -> cols[i] = "");

        for (String s : lines)
            IntStream.range(0, rowSize).forEach(i -> cols[i] += s.charAt(i));

        System.out.println(
                Arrays.stream(cols).map(
                        s -> s.chars().mapToObj(i -> (char) i).
                                collect(Collectors.groupingBy(v -> v, Collectors.counting()))
                                .entrySet()
                                .stream()
                                ./*part1: max*/min(Comparator.comparingLong(Map.Entry::getValue))
                                .get())
                        .map(Map.Entry::getKey)
                        .map(String::valueOf)
                        .collect(Collectors.joining())
        );


    }
}
