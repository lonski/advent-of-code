ids = File.readlines 'input.txt'

checksum = ids.map do |id| 
  counts = id.chars
    .reduce(Hash.new(0)) { |acc, v| acc[v] += 1; acc }
    .select{ |c, count| (2..3) === count }
    .reduce([0,0]) { |acc, (k, v)| acc[v-2] = 1 if v > 0; acc }
end
  .reduce([0,0]) { |acc, v| acc[0] += v[0]; acc[1] += v[1]; acc }
  .reduce(1, :*)

puts "Checksum of box id list: #{checksum}"

common_chars = ids.map(&:chars).map do |id|
  ids.map(&:chars).map do |cmp|
    tmp = id.clone
    cmp.map{|c| c == tmp.shift ? c : '^'}
  end
end
  .flat_map{ |v| v }
  .select{ |v| v.count('^') == 1 }
  .map{ |v| v.delete('^'); v}[0].join

puts "Common chars are: #{common_chars}"
