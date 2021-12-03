input = File.read ARGV[0]

#
# Part one
#
horizontal = 0
depth = 0

input.lines.each do |cmd|
  if cmd =~ /forward \d+/
    horizontal += /forward (\d+)/.match(cmd)[1].to_i
  elsif cmd =~ /down \d+/
    depth += /down (\d+)/.match(cmd)[1].to_i
  elsif cmd =~ /up \d+/
    depth -= /up (\d+)/.match(cmd)[1].to_i
  end
end

puts "Part one:\n\tHorizontal: #{horizontal}, depth: #{depth}, multiply: #{horizontal * depth}"

#
# Part two
#
horizontal = 0
depth = 0
aim = 0
input.lines.each do |cmd|
  if cmd =~ /forward \d+/
    val = /forward (\d+)/.match(cmd)[1].to_i
    horizontal += val
    depth += val * aim
  elsif cmd =~ /down \d+/
    aim += /down (\d+)/.match(cmd)[1].to_i
  elsif cmd =~ /up \d+/
    aim -= /up (\d+)/.match(cmd)[1].to_i
  end
end

puts "Part two:\n\tHorizontal: #{horizontal}, depth: #{depth}, multiply: #{horizontal * depth}"
