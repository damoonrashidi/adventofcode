#!/usr/bin/ruby
def wrap d
  area = 2*d[0]*d[1] + 2*d[1]*d[2] + 2*d[2]*d[0] + (d.sort[0] * d.sort[1])
end

def ribbon d
  d[0]*2 + d[1]*2 + d.reduce(:*)
end

packages = File.read("input.txt").split("\n").map{|line| line.split('x').map(&:to_i)}
paper = packages.inject(0){ |paper,package|
  paper += wrap package
}
string = packages.inject(0){ |string, package|
  string += ribbon package.sort
}

puts "Paper needed: #{paper} feet"
puts "Ribbon needed #{string} feet"