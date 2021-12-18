
class SnailfishNumber 

    attr_accessor :left, :right, :value, :parent

    def initialize(input, parent)
        @parent = parent
        if input.is_a? Array
            l, r = input
            @left = SnailfishNumber.new(l, self) unless l.nil?
            @right = SnailfishNumber.new(r, self) unless r.nil?
        else
            @value = input
        end
    end

    def to_s
        return @value.to_s unless @value.nil?
        s = "["
        s << "#{@left}#{right&.value.nil? ? '' : ','}" unless @left.nil?
        s << "#{right}" unless @right.nil?
        s << "]"

        s.gsub("][", "],[")
    end

    def leaf?
        @right.nil? && @left.nil?
    end

    def dig(n)
        return self if n == 0

        dig_left = @left&.dig(n - 1) 
        return dig_left unless dig_left.nil? || dig_left.leaf?

        @right&.dig(n - 1)
    end

    def climb_for_value(dir)
        next_node = parent&.send(dir)
        return nil if parent.nil? || next_node.nil?

        unless self == next_node
            dug = next_node.dig_for_value(dir == "left" ? "right" : "left")
            return dug unless dug.nil?
        end

        return parent.climb_for_value(dir)
    end

    def dig_for_value(dir)
        return self unless value.nil?
        send(dir).dig_for_value(dir)
    end

    def explode
        to_explode = dig(4)

        return false if to_explode.nil? || to_explode.leaf?

        left_val = to_explode.left&.value
        right_val = to_explode.right&.value

        unless left_val.nil?
            to_add_left = to_explode.climb_for_value('left')
            unless to_add_left.nil? 
                to_add_left.value += left_val
            end
        end

        unless right_val.nil?
            to_add_right = to_explode.climb_for_value('right')
            unless to_add_right.nil?
                to_add_right.value += right_val
            end
        end

        to_explode.left = nil
        to_explode.right = nil
        to_explode.value = 0

        true
    end

    def dig_for_split
        return self if !value.nil? && value > 9

        @left&.dig_for_split || @right&.dig_for_split
    end

    def split
        to_split = dig_for_split
        return false if to_split.nil?
        to_split.left = SnailfishNumber.new((to_split.value / 2.0).floor.to_i, to_split)
        to_split.right = SnailfishNumber.new((to_split.value / 2.0).ceil.to_i, to_split)
        to_split.value = nil
        true
    end

    def reduce
        loop do
            reduced = false

            while explode do
                reduced = true
            end
            if split
                reduced = true
            end

            break if !reduced
        end 
    end

    def magnitude
        return value unless value.nil?
        return left.magnitude * 3 + 2 * right.magnitude
    end

    def self.add(a, b)
        added = SnailfishNumber.new(nil, nil)
        added.left = a
        added.left.parent = added
        added.right = b
        added.right.parent = added
        added
    end

end

raw_numbers = File.read(ARGV[0]).split("\n").map{|n| eval n}

puts "Part 1: #{raw_numbers.map{|n| SnailfishNumber.new(n, nil)}.inject do |acc, n|
    added = SnailfishNumber.add(acc, n)
    added.reduce
    added
end.magnitude}"


puts "Part 2: #{raw_numbers
    .combination(2)
    .map{|a,b| [[a,b], [b,a]]}
    .flatten(1)
    .map{|a, b| [SnailfishNumber.new(a, nil), SnailfishNumber.new(b, nil)]}
    .map do |a, b|
         x = SnailfishNumber.add(a,b)
         x.reduce
         x.magnitude
    end.max}"

