def is_possible_password_part1(n)
  digits = n.digits.reverse
  return false unless digits.size == 6
  max_value = digits[0]
  duplicate_adjacent = false
  is_decreasing = false
  digits[1..-1].each_with_index do |v,i| 
    duplicate_adjacent = true if digits[i] == v
    is_decreasing = true if v < max_value
    max_value = [max_value, v].max
  end
  duplicate_adjacent && !is_decreasing
end

puts "Part 1: #{(372037..905157).select{|n| is_possible_password_part1(n) }.count}"

def is_possible_password_part2(n)
  digits = n.digits.reverse
  return false unless digits.size == 6
  max_value = digits[0]
  duplicate_adjacent = false
  is_decreasing = false
  digits[1..-1].each_with_index do |v,i| 
    if digits[i] == v
      if i == 0 && digits[i+2] != v
        duplicate_adjacent = true
      elsif i > 0 && digits[i+2] != v && digits[i-1] != v 
        duplicate_adjacent = true
      end
    end
    is_decreasing = true if v < max_value
    max_value = [max_value, v].max
  end
  duplicate_adjacent && !is_decreasing
end

puts "Part 2: #{(372037..905157).select{|n| is_possible_password_part2(n) }.count}"
