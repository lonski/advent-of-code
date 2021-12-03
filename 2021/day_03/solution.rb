def count_bits(input)
  size = input[0].size
  ones = size.times.map { 0 }
  zeros = size.times.map { 0 }
  
  input.each do |line|
    line.chars.each_with_index do |c, i|
      c == '1' ? ones[i] = ones[i] + 1 : zeros[i] = zeros[i] + 1
    end
  end

  [ones, zeros]
end

def calc_rating(input, bit)
  input[0].size.times.inject(input) do |readings, i|
    if readings.size == 1
      readings
    else
      ones, zeros = count_bits(readings)
      readings.select do |r|
        char = ones[i] >= zeros[i] ? bit[0] : bit[1]
        r.chars[i] == char
      end
    end
  end[0].to_i(2)
end

input = File.read(ARGV[0]).split("\n")

ones, zeros = count_bits(input)
gamma = ones.zip(zeros).map{|o, z| o > z ? 1 : 0}.join.to_i(2)
epsilon = ones.zip(zeros).map{|o, z| o > z ? 0 : 1}.join.to_i(2)

puts "Part 1: #{gamma * epsilon}"
puts "Part 2: #{calc_rating(input, ['1', '0']) * calc_rating(input, ['0', '1'])}"

