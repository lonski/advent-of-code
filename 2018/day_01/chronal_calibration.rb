require 'set'

changes = File.readlines('input.1').map(&:to_i)

puts "First resulting frequency: #{changes.reduce(0, :+)}"

freqs = Set.new
freq = 0
changes.cycle do |v|
  freq += v
  unless freqs.add? freq
      puts "First frequency reached twice: #{freq}"
      break
  end
end

