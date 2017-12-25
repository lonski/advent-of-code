import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

class RoomData {
    private static final String ALPHABET = "abcdefghijklmnopqrstuvwxyz";

    int sector;
    private String checksum;
    private List<String> nameTokens;

    RoomData(int sector, String checksum, List<String> nameTokens) {
        this.sector = sector;
        this.checksum = checksum;
        this.nameTokens = nameTokens;
    }

    @Override
    public String toString() {
        return "Sector: " + sector + ", Checksum:" + checksum + ", Tokens: " + nameTokens;
    }

    boolean isReal() {
        return getEncryptedName().chars()
                .mapToObj(i -> (char) i)
                .collect(Collectors.groupingBy(v -> v, Collectors.counting()))
                .entrySet()
                .stream()
                .collect(Collectors.groupingBy(Map.Entry::getValue))
                .entrySet()
                .stream()
                .sorted((e1, e2) -> (int) (e2.getKey() - e1.getKey()))
                .map(
                        e -> e.getValue().stream()
                                .map(Map.Entry::getKey)
                                .sorted()
                                .collect(Collectors.toList()))
                .map(e -> e.stream().map(String::valueOf).collect(Collectors.joining()))
                .collect(Collectors.joining())
                .startsWith(checksum);
    }

    String getDecryptedName() {
        return nameTokens.stream()
                .map(str -> str.chars()
                        .mapToObj(i -> (char) i)
                        .map(this::rotateSectorTimes)
                        .map(String::valueOf)
                        .collect(Collectors.joining()))
                .map(s -> s + " ")
                .collect(Collectors.joining());
    }

    private char rotateSectorTimes(char c) {
        char r = c;
        for (int i = 0; i < sector; i++) {
            r = rotate(r);
        }
        return r;
    }

    private char rotate(char c) {
        int pos = ALPHABET.indexOf(c);
        if (pos + 1 < ALPHABET.length())
            return ALPHABET.charAt(pos + 1);
        return ALPHABET.charAt(0);
    }

    private String getEncryptedName() {
        return nameTokens.stream().collect(Collectors.joining());
    }
}

public class PuzzleNo4 {

    public static void main(String[] args) throws IOException {
//        Integer sum = Files.readAllLines(Paths.get("src/puzzle_4.input"))
//                .stream().map(PuzzleNo4::parseRoomData)
//                .filter(RoomData::isReal)
//                .map(r -> r.sector)
//                .reduce(0, Integer::sum);

        RoomData room = Files.readAllLines(Paths.get("src/puzzle_4.input"))
                .stream().map(PuzzleNo4::parseRoomData)
                .filter(r -> r.getDecryptedName().contains("pole"))
                .findFirst().get();

        System.out.println(room.getDecryptedName() + " Sector = " + room.sector);
    }

    private static RoomData parseRoomData(String room) {
        String[] rawTokens = room.split("-");
        String[] sectorAndChecksum = rawTokens[rawTokens.length - 1].split("\\[");

        int sectorId = Integer.valueOf(sectorAndChecksum[0]);
        String checksum = sectorAndChecksum[1].replaceAll("]", "");

        List<String> tokens = Arrays.stream(rawTokens).filter(t -> !t.contains("]"))
                .collect(Collectors.toList());

        return new RoomData(sectorId, checksum, tokens);
    }
}
