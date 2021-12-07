input = File.read(ARGV[0]).split(',').map(&:to_i)

puts "Part 1: #{input.max.times.map { |pos| input.inject(0) { |sum, val| sum += (pos - val).abs } }.min}"

part2 = input.max.times.map do |pos|
  input.inject(0) do |sum, val|
    range = (pos - val).abs
    sum += (range + 1) / 2.0 * range # fast sum of consecutive numbers
  end
end.min.to_i
puts "Part 2: #{part2}"

