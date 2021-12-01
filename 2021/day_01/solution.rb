def count_increased(arr)
  arr.each_cons(2).select{ |a, b| b > a }.count
end

input = File.read(ARGV[0]).lines.map(&:to_i)

puts "Part 1: #{count_increased input}"
puts "Part 2: #{count_increased input.each_cons(3).map {|x| x.inject(&:+) }}"
