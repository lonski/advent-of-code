import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

class Screen
{
    private final int width;
    private final int height;

    private Character[] pixels;

    Screen(int width, int height)
    {
        this.width = width;
        this.height = height;
        this.pixels = Collections.nCopies(width * height, '.').toArray(new Character[width * height]);
    }

    long getLitPixelsCount()
    {
        return Arrays.stream(pixels).filter(c -> c == '#').count();
    }

    void rect(int w, int h)
    {
        while (h-- > 0) {
            int lw = w;
            while (lw-- > 0) {
                pixels[h * width + lw] = '#';
            }
        }
    }

    void rotateCol(int col, int factor)
    {
        while (factor-- > 0) {
            rotateCol(col);
        }
    }

    void rotateCol(int col)
    {
        if (col >= width) {
            return;
        }

        Character last = getChar(height - 1, col);
        for (int row = height - 1; row > 0; row--) {
            setChar(row, col, getChar(row - 1, col));
        }
        setChar(0, col, last);
    }

    void rotateRow(int row, int factor)
    {
        while (factor-- > 0) {
            rotateRow(row);
        }
    }

    void rotateRow(int row)
    {
        if (row >= height) {
            return;
        }

        Character last = getChar(row, width - 1);
        for (int col = width - 1; col > 0; col--) {
            setChar(row, col, getChar(row, col - 1));
        }
        setChar(row, 0, last);
    }

    void setChar(int row, int col, Character c)
    {
        int idx = row * width + col;
        if (idx < pixels.length) {
            pixels[idx] = c;
        }
        else {
            throw new RuntimeException("Invalid index: " + idx);
        }
    }

    Character getChar(int row, int col)
    {
        int idx = row * width + col;
        if (idx >= pixels.length) {
            throw new RuntimeException("Invalid index (get): " + idx);
        }
        return pixels[idx];
    }

    @Override
    public String toString()
    {
        String s = "";
        int i = 0;
        for (Character c : pixels) {
            s += c;
            i++;
            if (i % width == 0) {
                s += "\n";
            }
        }
        return s;
    }
}

public class PuzzleNo8
{
    public static void main(String[] args)
            throws IOException
    {
        List<String> commands = Files.readAllLines(Paths.get("puzzle_8.input"));
        Screen screen = new Screen(50, 6);

        for (String command : commands) {
            String[] tokens = command.split(" ");
            if (tokens[0].equals("rect")) {
                String[] coords = tokens[1].split("x");
                screen.rect(Integer.parseInt(coords[0]), Integer.parseInt(coords[1]));
            }
            else if (tokens[0].equals("rotate")) {
                if (tokens[1].equals("row")) {
                    int row = Integer.parseInt(tokens[2].split("=")[1]);
                    int factor = Integer.parseInt(tokens[4]);
                    screen.rotateRow(row, factor);
                }
                else if (tokens[1].equals("column")) {
                    int col = Integer.parseInt(tokens[2].split("=")[1]);
                    int factor = Integer.parseInt(tokens[4]);
                    screen.rotateCol(col, factor);
                }
            }
            System.out.println(screen);
        }
        System.out.println(screen.getLitPixelsCount());
    }
}
