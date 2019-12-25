def pattern_for(n)
  base_pattern = [0,1,0,-1]
  pattern = []
  base_pattern.each {|v| n.times{ pattern << v}}
  pattern
end

def fft(signal)
  (1..signal.size).each do |n|
    pattern = pattern_for(n).cycle.take(signal.size+1)[1..-1]
    signal[n-1] = signal.each_with_index.map{ |d,i| d * pattern[i]}.reduce(&:+).to_s[-1].to_i
  end
end

input = File.read 'input_0'

signal_p1 = input.chars.map(&:to_i)
100.times { fft(signal_p1) }
puts "Part 1: #{signal_p1.join}"
