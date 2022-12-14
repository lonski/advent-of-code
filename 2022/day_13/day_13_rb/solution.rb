def in_order(left, right)

  return true if left.nil?
  return false if right.nil?

  i = 0
  while i < [left.size, right.size].max
    l = left[i]
    r = right[i]

    return true if l.nil?
    return false if r.nil?

    if l.is_a?(Array) || r.is_a?(Array)
      sub = in_order(l.is_a?(Array) ? l : [l], r.is_a?(Array) ? r : [r])
      return sub unless sub.nil?
    elsif l < r
      return true
    elsif r < l
      return false
    end

    i += 1
  end

  nil
end

puts File.read('input')
         .split("\n\n")
         .map { |p| p.split("\n") }
         .map { |p| [eval(p[0]), eval(p[1])] }
         .filter_map
         .with_index { |p, i| i + 1 if in_order(p[0], p[1]) }
         .sum

divider_packets = [[[2]], [[6]]]
puts (File.read('input')
          .gsub("\n\n", "\n")
          .split("\n")
          .map { |p| eval(p) } + divider_packets)
       .sort { |p1, p2| in_order(p1, p2) ? -1 : 1 }
       .filter_map
       .with_index { |p, i| i + 1 if divider_packets.include?(p) }
       .inject(&:*)
