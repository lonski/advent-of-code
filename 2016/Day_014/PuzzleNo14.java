import javax.xml.bind.annotation.adapters.HexBinaryAdapter;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class PuzzleNo14 {
    public static void main(String[] args) throws NoSuchAlgorithmException {

        long idx = 0;
        long deepIdx = 0;
        int keysFound = 0;

        while (true) {
            String md5 = stretch(getMd5(buildKeyBase(idx)));
            Matcher matcher = getPatternForThreeRepeatedChars().matcher(md5);
            if (matcher.matches()) {
                deepIdx = 1;
                System.out.println(String.format("Potential first key at %d: %s... ", idx, md5));
                while (deepIdx <= 1000) {
                    String s_md5 = stretch(getMd5(buildKeyBase(idx + deepIdx)));
                    if (getPatternForFiveRepeatedChars(matcher.group(1).charAt(0))
                            .matcher(s_md5)
                            .matches()) {
                        keysFound++;
                        System.out.println(String.format(
                                "First part '%s'. Second part '%s' of idx at [%d, %d (%d)]. Keys found = %d.",
                                md5, s_md5, idx, idx + deepIdx, deepIdx, keysFound));
                        break;
                    }
                    deepIdx++;
                }
            }
            if (keysFound == 64)
                break;
            idx++;
        }
        System.out.println(idx + deepIdx);

    }

    private static String stretch(String h) throws NoSuchAlgorithmException {
        String stretched = h;
        int times = 2016;
        while (times-- > 0)
            stretched = getMd5(stretched);

        return stretched;
    }

    private static String buildKeyBase(long index) {
        final String input = "qzyelonm";//"qzyelonm";
        return input + String.valueOf(index);
    }

    private static Pattern getPatternForThreeRepeatedChars() {
        return Pattern.compile(".*(\\w)\\1{2}.*");
    }

    private static Pattern getPatternForFiveRepeatedChars(char ch) {
        return Pattern.compile(".*" + String.valueOf(ch) + "{5}.*");
    }

    private static String getMd5(String input) throws NoSuchAlgorithmException {
        MessageDigest md = MessageDigest.getInstance("MD5");
        byte[] md5sum = md.digest(input.getBytes());
        return (new HexBinaryAdapter()).marshal(md5sum).toLowerCase();
    }
}
