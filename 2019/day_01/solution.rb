def calc_fuel(mass)
  fuel = mass / 3 - 2
  fuel > 6 ? fuel + calc_fuel(fuel) : fuel
end

puts File.read('input.1').lines.map(&:to_i).map{|m| m / 3 - 2}.inject(&:+)
puts File.read('input.1').lines.map(&:to_i).map{|m| calc_fuel(m)}.inject(&:+)
