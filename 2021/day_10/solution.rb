def opposite(c)
  return '(' if c == ')'
  return '[' if c == ']'
  return '{' if c == '}'
  return '<' if c == '>'
end

def find_incorrect(line)
  open = []
  line.chars do |c|
    if ['(', '[', '{', '<'].include? c
      open << c
    else
      o = opposite(c)
      if !open.empty? && open[-1] == o
        open.pop
      else
        return [c, open]
      end
    end
  end
  [nil, open]
end

input = File.read(ARGV[0]).split("\n")
p1score = {
  ')' => 3,
  ']' => 57,
  '}' => 1197,
  '>' => 25137
}

part1 = input
  .map{ |line| find_incorrect(line)[0] }
  .compact
  .reject(&:empty?)
  .flatten
  .map{ |c| p1score[c] }
  .sum
puts "Part 1: #{part1.inspect}"

p2score = {
  '(' => 1,
  '[' => 2,
  '{' => 3,
  '<' => 4
}

incomplete_scores = input
  .map{|line| find_incorrect(line) }
  .select{|incorrect, open| incorrect.nil? }
  .map{|incorrect, open| open }
  .map { |open| open.reverse.inject(0) { |score, c| score * 5 + p2score[c] } }
  .sort

puts "Part 2: #{incomplete_scores[incomplete_scores.size / 2]}"



