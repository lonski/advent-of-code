import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class PuzzleNo9
{
    public static void main(String[] args)
            throws IOException
    {
        String content = Files.readAllLines(Paths.get("puzzle_9.input")).get(0);
        System.out.println(decompress(content));
    }

    private static long decompress(String input)
    {
        long i = 0;
        int index = 0;
        while (index < input.length()) {

            if (input.charAt(index) != '(') { //Not a marker
                i += 1;
            }
            else {
                String marker = "";

                //Read marker
                index++;
                while (input.charAt(index) != ')') {
                    marker += input.charAt(index);
                    index++;
                }

                //Parse marker
                String[] splitMarker = marker.split("x");
                int charCount = Integer.valueOf(splitMarker[0]);
                int timesCount = Integer.valueOf(splitMarker[1]);

                long temp_i = decompress(input.substring(index + 1, index + charCount + 1));

                while (timesCount-- > 0) {
                    i += temp_i;
                }

                index += charCount;
            }

            index++;
        }
        return i;
    }
}
