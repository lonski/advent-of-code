encoded_image = File.read 'input_0'
width = 25
height = 6

layers = encoded_image.chars.each_slice(width*height).to_a

mbl = layers
  .map{|s| [s.count('0'), s.count('1'), s.count('2')] }
  .reject{|a| a == [0,0,0]  }
  .min_by{|a| a[0] }

puts "Part 1: #{mbl[1] * mbl[2]}"
puts "Part 2:"
height.times do |y|
  width.times do |x|
    l = layers.select{|l| l[y*width+x] != '2'}.first
    putc l[y*width + x] == '1' ? '#' : ' '
  end
  putc "\n"
end
