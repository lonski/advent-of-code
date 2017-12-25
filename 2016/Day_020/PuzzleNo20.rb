lines = File.readlines('input.txt').map{ |l| l.split("-").map(&:to_i) }.sort_by{|v| v[0]}

first_found = nil
total_alowed = 0
max_ip = 0
lines.each_slice(2) do |v1,v2|
  if v2[0] > v1[1] + 1 && v2[0] > max_ip + 1
    max_ip = v2[1]
    first_found = [v1, v2] if first_found.nil?
    allowed = (v2[0] - (v1[1] + 1))
    total_alowed += allowed
#    puts "#{v1.inspect} :: #{v2.inspect} -> #{allowed} (Total: #{total_alowed})"
  end
end

puts "First found: #{first_found[0][1] + 1}"
puts "Total allowed: #{total_alowed}"
