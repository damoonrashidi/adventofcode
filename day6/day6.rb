instructions = File.read("input.txt").split("\n")
lights = (0..1000).to_a.map!{|l| l = Array.new(1000,0)}

def switch direction, value
  return value + 1 if direction == 'on'
  return value > 0 ? value - 1 : 0 if direction == 'off'
  return value + 2 if direction == 'toggle'
end

instructions.each do |instruction|
  grid = instruction.scan(/([\dx\d]+)/).flatten.map{|i| i.to_i}
  d = instruction.match(/on|off|toggle/)[0]
  (grid[0]..grid[2]).each do |row|
    (grid[1]..grid[3]).each do |col|
      lights[row][col] = switch d, lights[row][col]
    end
  end
end
puts lights.flatten.reduce(:+)