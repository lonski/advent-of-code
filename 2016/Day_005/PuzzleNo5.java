import javax.xml.bind.annotation.adapters.HexBinaryAdapter;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Arrays;

public class PuzzleNo5 {

    public static void main(String[] args) throws NoSuchAlgorithmException {
        String input = "wtnhxymk";

        solvePart1(input);
        solvePart2(input);
    }

    private static void solvePart1(String input) throws NoSuchAlgorithmException {
        System.out.println("Solving part 1...");

        long i = 0;

        String pass = "";
        while(pass.length() < 8){
            String md5 = getMd5(input + String.valueOf(i));
            int zeros = countLeadingZeros(md5);
            if ( zeros == 5 ) {
                pass += md5.charAt(5);
                System.out.println("Pass: " + pass + " at index " + i + " withj md5=" + md5);
            }
            i++;
        }
        System.out.println(pass);
    }

    private static  void solvePart2(String input) throws NoSuchAlgorithmException {
        System.out.println("Solving part 2...");

        long i = 0;
        char[] pass = new char[8];
        int passLen = 0;

        while ( passLen < 8 ) {
            String md5 = getMd5(input + String.valueOf(i));
            int zeros = countLeadingZeros(md5);
            if ( zeros >= 5 ) {
                int pos = parseIndex(md5.charAt(5));
                if ( pos < 8 && pos != -1 && pass[pos] == '\u0000') {
                    pass[pos] = md5.charAt(6);
                    passLen += 1;
                    System.out.println("Pass: " + Arrays.toString(pass) + " at index " + i + " withj md5=" + md5);
                }
            }
            i++;
        }

        System.out.println("Pass = ");
        for (char c : pass)
            System.out.print(c);
    }

    private static int parseIndex(char c)
    {
        int i = -1;

        try{
            i = Integer.parseInt(String.valueOf(c));
        }
        catch(NumberFormatException ignored) {}

        return i;
    }

    private static int countLeadingZeros(String s) {
        int c = 0;

        for (char ch : s.toCharArray())
            if (ch == '0')
                c++;
            else
                break;

        return c;
    }

    private static String getMd5(String input) throws NoSuchAlgorithmException {
        MessageDigest md = MessageDigest.getInstance("MD5");
        byte[] md5sum = md.digest(input.getBytes());
        return (new HexBinaryAdapter()).marshal(md5sum);
    }
}
