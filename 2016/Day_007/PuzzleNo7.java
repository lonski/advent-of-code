import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class PuzzleNo7 {
	public static void main(String[] args) throws IOException {
		List<String> lines = Files.readAllLines(Paths.get("puzzle_7.input"));

		solvePart1(lines);
		solvePart2(lines);
	}

	private static void solvePart1(List<String> lines) {
		System.out.println("Part1:");
		String regexp = "\\[[a-z]*\\]";
		int count = 0;
		for ( String line : lines ) {

			String[] outsideBracket = line.split(regexp);
			Pattern r = Pattern.compile(regexp);
			Matcher inBrackets = r.matcher(line);

			for (String s : outsideBracket) {
				//System.out.println(s + " -> " + isMirroredSequence(s));
				if (isMirroredSequence(s)) {
					count++;
					while (inBrackets.find()) {
						String bracked = inBrackets.group().replaceAll("\\[|\\]", "");
						if ( isMirroredSequence(bracked) ){
							count--;
							break;
						}
					}
					break;
				}

			}
		}
		System.out.println(count);
	}

	private static void solvePart2(List<String> lines) {
		System.out.println("Part2:");
		String regexp = "\\[[a-z]*\\]";
		int count = 0;
		for ( String line : lines ) {

			String[] outsideBracket = line.split(regexp);

			for (String s : outsideBracket) {
				//System.out.println(s + " -> " + isMirroredSequence(s));
				List<String> triplets = findMirroredTriplet(s);
				boolean found = false;
				for(String triplet : triplets) {
					//System.out.println("Triplet=" + triplet);
					if (triplet != null && hasReversedTriplet(line, triplet)) {
						//System.out.println(line);
						count++;
						found = true;
						break;
					}
				}
				if (found)
					break;
			}
		}
		System.out.println(count);
	}

	private static boolean isMirroredSequence(String s){
		for(int i = 0; i + 3 < s.length(); i++){
			if ( s.charAt(i) == s.charAt(i+3) && s.charAt(i+1) == s.charAt(i+2) && s.charAt(i) != s.charAt(i+1))
				return true;
		}
		return false;
	}

	private static List<String> findMirroredTriplet(String s){
		List<String> triplets = new ArrayList<>();
		for(int i = 0; i + 2 < s.length(); i++){
			if ( s.charAt(i) == s.charAt(i+2) && s.charAt(i) != s.charAt(i+1))
				triplets.add(s.substring(i,i+3));
		}
		return triplets;
	}

	private static boolean isReversedTriplet(String s, String tripet){
		return findMirroredTriplet(s).size() > 0 && s.charAt(0) == tripet.charAt(1) && s.charAt(1) == tripet.charAt(0);
	}

	private static boolean hasReversedTriplet(String line, String triplet){
		String regexp = "\\[[a-z]*\\]";
		Pattern r = Pattern.compile(regexp);
		Matcher inBrackets = r.matcher(line);
		while (inBrackets.find()) {
			String bracked = inBrackets.group().replaceAll("\\[|\\]", "");
			for(int i = 0; i + 2 < bracked.length(); i++){
				if ( bracked.charAt(i) == bracked.charAt(i+2) && bracked.charAt(i) != bracked.charAt(i+1))
					if ( isReversedTriplet(bracked.substring(i,i+3), triplet) )
						return true;
			}
		}
		return false;
	}

}