require 'set'

def permutation(part, next_parts)
  #puts next_parts.inspect
  next_part = next_parts.shift
  r = part.map do |p|
    next_part.map do |n|
      #puts "\t#{(p.chars + n.chars)}"
      (p.chars + n.chars).permutation.map(&:join)
    end
  end.flatten
  next_parts.empty? ? r : permutation(r, next_parts)
end

input = File.read(ARGV[0])
  .split("\n")
  .map{ |line| line.split(' | ') }
  .map{ |wiring, digits| [wiring.split(' '), digits.split(' ')] }

puts "Part 1: #{input.map{ |wiring, digits| digits.select{|d| [2, 3, 4, 7].include? d.size } }.map(&:size).sum}"

#  AAAA
#  F  B
#  F  B
#  GGGG
#  E  C
#  E  C
#  DDDD

part2 = input.map do |wiring, digits| 
  possible_mapping = {
    'A' => [],
    'B' => [],
    'C' => [],
    'D' => [],
    'E' => [],
    'F' => [],
    'G' => [],
  }

  # 1
  wiring.select{ |w| w.size == 2 }[0].chars.each do |c| 
    ['B', 'C'].each { |cc| possible_mapping[cc] << c unless possible_mapping[cc].include? c }
  end
  # 7
  blacklisted = ['B', 'C'].map{|cc| possible_mapping[cc].to_a }.flatten
  wiring.select{ |w| w.size == 3 }[0].chars.reject{|c| blacklisted.include? c }.each do |c| 
    ['A'].each { |cc| possible_mapping[cc] << c unless possible_mapping[cc].include? c }
  end
  # 4
  wiring.select{ |w| w.size == 4 }[0].chars.reject{|c| blacklisted.include? c }.each do |c| 
    ['F', 'G'].each { |cc| possible_mapping[cc] << c unless possible_mapping[cc].include? c }
  end
  # 8
  blacklisted = ['A', 'B', 'C', 'G', 'F'].map{|cc| possible_mapping[cc].to_a }.flatten
  wiring.select{ |w| w.size == 7 }[0].chars.reject{|c| blacklisted.include? c }.each do |c| 
    ['D', 'E'].each { |cc| possible_mapping[cc] << c unless possible_mapping[cc].include? c }
  end

  two = ['A', 'B', 'G', 'E', 'D']
  twoMappings = two.map{|cc| possible_mapping[cc] }
  twoPermutations = permutation(twoMappings[0], twoMappings[1..])

  wiring
    .select{|w| w.size == 5 }
    .select{|w| twoPermutations.include? w }
    .each do |w|
    w.chars.each do |c|
      deleted = possible_mapping['C'].delete(c)
      possible_mapping['B'] = [deleted] unless deleted.nil?

      deleted = possible_mapping['F'].delete(c)
      possible_mapping['G'] = [deleted] unless deleted.nil?
    end
  end

  five = ['A', 'F', 'G', 'C', 'D']
  fiveMappings = five.map{|cc| possible_mapping[cc] }
  fivePermutations = permutation(fiveMappings[0], fiveMappings[1..])

  wiring
    .select{|w| w.size == 5 }
    .select{|w| fivePermutations.include? w }
    .each do |w|
    w.chars.each do |c|
      deleted = possible_mapping['E'].delete(c)
      possible_mapping['D'] = [deleted] unless deleted.nil?
    end
  end

  digits = digits.map do |digit|
    case digit.size
    when 2
      1
    when 3
      7
    when 4
      4
    when 7
      8
    else 
      [
        [['A', 'B', 'G', 'E', 'D'],2],
        [['A', 'B', 'G', 'C', 'D'],3],
        [['A', 'F', 'G', 'C', 'D'],5],
        [['A', 'F', 'G', 'E', 'C', 'D'],6],
        [['A', 'F', 'B', 'G', 'C', 'D'],9],
        [['A', 'F', 'B', 'E', 'C', 'D'],0],
      ]
        .map { |set, number| [ set.map{|c| possible_mapping[c]}.flatten, number ] }
        .select { |set, number| digit.chars.all?{|c| set.include? c } }
        .map{|set, number| number }
        .first
    end
  end.join.to_i

  digits
end.sum

puts "Part 2: #{part2}"
