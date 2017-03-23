#!/usr/bin/ruby
class Santa
  def initialize x, y
    @x = x
    @y = y
  end
  def position
    "#{@x}:#{@y}"
  end
  def move dir
    case dir
    when 'v'
      @y -= 1
    when '^'
      @y += 1
    when '<'
      @x -= 1
    when '>'
      @x += 1
    end
  end
end

map = {"0:0" => 2}
santa = Santa.new 0,0
robot = Santa.new 0,0

File.read('input.txt').split('').each_slice(2) do |set|
  santa.move set[0]
  robot.move set[1]
  map[santa.position] = true
  map[robot.position] = true
end
  
puts map.length