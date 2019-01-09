def full_react(polymer)
  new_polymer = react(polymer)
  new_polymer.size != polymer.size ? full_react(new_polymer) : new_polymer.size
end

def react(polymer)
  new_polymer = polymer.clone
  ('a'..'z').each do |l| 
    new_polymer.gsub! "#{l}#{l.upcase}", ''
    new_polymer.gsub! "#{l.upcase}#{l}", ''
  end
  new_polymer
end

polymer = File.readlines('input.txt').first.gsub("\n", '')
puts "Part one: #{full_react(polymer)}"
puts "Part two: " + ('a'..'z')
  .map { |l| polymer.delete(l).delete(l.upcase) }
  .map { |p| full_react(p) }.min.to_s

