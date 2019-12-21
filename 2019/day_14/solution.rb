DEBUG = false

$recipes = {}
$warehouse = {}
$used_ore = 0

class Node
  attr_accessor :name, :value, :nodes, :parent

  def initialize(name, value, parent)
    @value = value
    @name = name
    @nodes = []
  end

  def add_node(name, value)
    n = Node.new(name, value, self)
    nodes << n
    n
  end

  def produce
    $warehouse[@name] ||= 0
    needed = @value

    #take from WH if any already produced
    in_wh = $warehouse[@name]
    if in_wh >= needed # take all from WH
      $warehouse[@name] = in_wh - needed
      needed = 0
    else
      $warehouse[@name] = 0
      needed -= in_wh
    end

    if needed > 0

      rec = $recipes[@name]
      rec_res = rec[0]

      # leaf node - raw material, produce from ore
      if @nodes.empty? 
        raise "Not an ORE recipe" if rec[1][0][1] != 'ORE' || rec[1].size > 1

        rec_ore_req = rec[1][0][0]

        to_produce = (needed.to_f / rec_res).ceil * rec_res.to_f
        ore_used = (to_produce / rec_res) * rec_ore_req
        $used_ore += ore_used
        puts "\tProducing #{to_produce} #{@name} using #{ore_used} ore" if DEBUG

        #store overproduction in WH
        $warehouse[@name] += (to_produce - needed) if to_produce > needed

      #not raw material, produce ingridiens
      else 
        to_produce = (needed.to_f / rec_res).ceil #* rec_res.to_f

        puts "Producing #{to_produce * rec_res.to_f} #{@name} val=#{@value} rec_res=#{rec_res}" if DEBUG
        to_produce.to_i.times {|| @nodes.each(&:produce) }
        
        produced = to_produce * rec_res.to_f

        #store overproduction in WH
        $warehouse[@name] += produced - needed if produced > needed
      end

    end
  end

end


def build_tree(node, ingridients)
  ingridients[1].each do |amount, name|
    next unless $recipes.key? name
    sub_node = node.add_node(name, amount)
    build_tree(sub_node, $recipes[name])
  end
end

def run(filename, clean=true)
  if clean
    $warehouse = {}
    $used_ore = 0
  end

  input = File.read filename
  $recipes = input.split("\n")
    .map{|l| l.split ' => '}
    .map{|ingr, res| [res.split(' '), ingr ] }
    .map{|r,i| [ [r[0].to_i, r[1] ], i] }
    .map{|r,i| [r, i.split(', ')] }
    .map{|r,i| [r, i.map{|v| x=v.split(' '); [x[0].to_i, x[1]]}]}
    .map{|r,i| [r[1], [r[0], i]] }
    .to_h
  
  fuel_rec = $recipes["FUEL"]
  
  tree = Node.new("FUEL", 1, nil)
  build_tree(tree, fuel_rec) 
  
  
  tree.produce
  puts "Leftovers: #{$warehouse.inspect}" if DEBUG
  puts "Used ore: #{$used_ore}" if DEBUG

  $used_ore.to_i
end

def part1
  raise "\n\n\n\t!!!INCORRECT input1\n\n" unless run('input_1') == 31
  raise "\n\n\n\t!!!INCORRECT input2\n\n" unless run('input_2') == 165
  raise "\n\n\n\t!!!INCORRECT input0\n\n" unless run('input_0') == 13312
  raise "\n\n\n\t!!!INCORRECT input3\n\n" unless run('input_3') == 180697
  raise "\n\n\n\t!!!INCORRECT input4\n\n" unless run('input_4') == 2210736
  
  run('input_5')
end

def part2_bruteforce
  trillion = 1000000000000
  fuel = 0
    
  while $used_ore < trillion
    run('input_5', false)
    fuel += 1
    puts "Fuel #{fuel} ore_left #{trillion - $used_ore}"
  end
end

puts part1
#part2_bruteforce

