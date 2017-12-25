import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

enum Direction {
    U,
    D,
    L,
    R
}

class Keypad {
    private String layout;

    private int rowSize;
    private int selectedIndex;

    Keypad(String layout, int rowSize, int startingIndex) {
        this.selectedIndex = startingIndex;
        this.layout = layout;
        this.rowSize = rowSize;
    }

    public char getSelected() {
        return layout.charAt(selectedIndex);
    }

    public void move(Direction d) {
        int currentRow = selectedIndex / rowSize;
        int currentColumn = selectedIndex % rowSize;

        int newIndex = selectedIndex;

        if (d == Direction.U) {
            if (currentRow > 0)
                newIndex -= rowSize;
        } else if (d == Direction.D) {
            if (currentRow < rowSize - 1)
                newIndex += rowSize;
        } else if (d == Direction.L) {
            if (currentColumn > 0)
                newIndex -= 1;
        } else if (d == Direction.R) {
            if (currentColumn < rowSize - 1)
                newIndex += 1;
        }

        if (layout.charAt(newIndex) != '.') {
            selectedIndex = newIndex;
        }
    }
}

public class PuzzleNo2 {

    public static void main(String[] args) throws IOException {
        List<String> input = Files.readAllLines(Paths.get("src/puzzle_2a.input"));

        Keypad keypad = new Keypad(
                "..1.." +
                        ".234." +
                        "56789" +
                        ".ABC." +
                        "..D..",
                5,
                10
        );

        for (String line : input) {

            for (char c : line.toCharArray()) {
                Direction dir = Direction.valueOf(String.valueOf(c));
                keypad.move(dir);
            }

            System.out.print(keypad.getSelected());
        }
    }

}
