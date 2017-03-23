BUILDING = File.read("day1.txt").split ''

def silver b
  b.inject(0){|floor, i| floor += i == "(" ? 1 : -1 }  
end

def gold b
  floor = 0
  b.map{|i| i == "(" ? 1 : -1}.each.with_index { |dir, index|
    floor += dir
    return index if floor == -1
  }
end

puts "Silver: #{silver(BUILDING)}"
puts "Gold: #{gold(BUILDING)}"